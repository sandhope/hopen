/// Core bridge — data pipeline from Go core to GPUI Global state.
///
/// Push events (log, delay, request) from the Go core are received via
/// `CoreEventListener` on the IPC dispatch thread and buffered into a
/// thread-safe queue. The main GPUI event loop periodically drains this
/// queue and writes data into the corresponding Global state types.
///
/// Poll-based data (proxies, connections, traffic) is fetched directly
/// from the main GPUI event loop via `poll_all()`.
use std::sync::{Arc, Mutex};

use gpui::*;

use crate::core::event::CoreEventListener;
use crate::core::manager::CoreManager;
use crate::state::connection::{Connection, ConnectionState, RequestLog};
use crate::state::log::{LogEntry, LogLevel, LogState};
use crate::state::config::{ConfigProfile, ConfigState};
use crate::state::proxy::ProxyState;
use crate::state::proxy_models::*;
use crate::AppState;

// ── Thread-safe event buffer ─────────────────────────────────────

#[derive(Debug, Clone)]
enum BridgeEvent {
    Log { time: String, level: String, payload: String },
    Delay { name: String, value: Option<i32> },
    Request { time: String, host: String, proxy: String, delay: i32, rule: String, info: Option<String> },
}

/// Shared buffer of pending bridge events.
/// The IPC dispatch thread pushes events; the main thread drains them.
pub struct BridgeBuffer {
    events: Mutex<Vec<BridgeEvent>>,
}

impl BridgeBuffer {
    fn new() -> Self {
        Self { events: Mutex::new(Vec::new()) }
    }

    fn push(&self, event: BridgeEvent) {
        if let Ok(mut guard) = self.events.lock() {
            guard.push(event);
        }
    }

    /// Drain all pending events. Called from the main thread.
    pub fn drain(&self) -> Vec<BridgeEvent> {
        if let Ok(mut guard) = self.events.lock() {
            std::mem::take(&mut *guard)
        } else {
            Vec::new()
        }
    }
}

// ── CoreEventListener implementation ─────────────────────────────

struct BridgeListener {
    buffer: Arc<BridgeBuffer>,
}

impl CoreEventListener for BridgeListener {
    fn on_log(&self, data: &serde_json::Value) {
        let level = data.get("level").and_then(|v| v.as_str()).unwrap_or("info").to_string();
        let payload = data.get("payload").and_then(|v| v.as_str())
            .or_else(|| data.as_str()).unwrap_or("").to_string();
        let time = chrono::Local::now().format("%H:%M:%S").to_string();
        self.buffer.push(BridgeEvent::Log { time, level, payload });
    }

    fn on_delay(&self, data: &serde_json::Value) {
        if let Ok(delay) = serde_json::from_value::<Delay>(data.clone()) {
            self.buffer.push(BridgeEvent::Delay { name: delay.name, value: delay.value });
        }
    }

    fn on_request(&self, data: &serde_json::Value) {
        let time = data.get("time").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let host = data.get("host").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let proxy = data.get("proxy").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let delay = data.get("delay").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let rule = data.get("rule").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let info = data.get("info").and_then(|v| v.as_str()).map(|s| s.to_string());
        self.buffer.push(BridgeEvent::Request { time, host, proxy, delay, rule, info });
    }

    fn on_loaded(&self, _data: &serde_json::Value) {}
    fn on_crash(&self, _data: &serde_json::Value) {
        log::error!("[Bridge] Core crash event received");
    }
}

// ── Public API ───────────────────────────────────────────────────

/// Start the bridge: creates the listener and returns it.
/// The listener should be registered with `CoreManager::add_listener()`.
///
/// Also returns the `BridgeBuffer` Arc for periodic draining from the
/// main event loop.
pub fn start_bridge() -> (Arc<dyn CoreEventListener>, Arc<BridgeBuffer>) {
    let buffer = Arc::new(BridgeBuffer::new());
    let listener = Arc::new(BridgeListener { buffer: buffer.clone() });
    (listener, buffer)
}

/// Process all pending bridge events and apply them to GPUI Global state.
/// Call this periodically from the main GPUI event loop.
pub fn process_pending_events(buffer: &BridgeBuffer, cx: &mut App) {
    for event in buffer.drain() {
        match event {
            BridgeEvent::Log { time, level, payload } => {
                let log_level = match level.to_lowercase().as_str() {
                    "debug" => LogLevel::Debug,
                    "info" => LogLevel::Info,
                    "warning" | "warn" => LogLevel::Warning,
                    "error" => LogLevel::Error,
                    _ => LogLevel::Info,
                };
                cx.update_global::<LogState, _>(|state, _cx| {
                    state.push(LogEntry { time, level: log_level, payload });
                });
            }
            BridgeEvent::Delay { name, value } => {
                cx.update_global::<ProxyState, _>(|state, _cx| {
                    state.update_delay(&Delay { name, url: String::new(), value });
                });
            }
            BridgeEvent::Request { time, host, proxy, delay, rule, info } => {
                cx.update_global::<ConnectionState, _>(|state, _cx| {
                    state.request_logs.push(RequestLog { time, host, proxy, delay, rule, info });
                    if state.request_logs.len() > 5000 {
                        let excess = state.request_logs.len() - 5000;
                        state.request_logs.drain(0..excess);
                    }
                });
            }
        }
    }
}

/// Poll all core state (proxies, connections, traffic). Called from the
/// main GPUI event loop on a timer.
pub fn poll_all(cx: &mut App) {
    let core_running = cx.global::<AppState>().core_running;
    if !core_running {
        return;
    }
    if cx.try_global::<CoreManager>().is_none() {
        return;
    }

    // Proxies — scoped fetch to release immutable borrow before mutable updates
    {
        let proxy_groups = cx.try_global::<CoreManager>()
            .and_then(|mgr| mgr.get_proxies().ok())
            .and_then(|r| serde_json::from_value::<std::collections::HashMap<String, Group>>(r.data).ok());
        if let Some(groups) = proxy_groups {
            cx.update_global::<ProxyState, _>(|state, _cx| {
                state.update_from_core(groups);
            });
        }
    }

    // Connections
    {
        let connections = cx.try_global::<CoreManager>()
            .and_then(|mgr| mgr.get_connections().ok())
            .and_then(|r| serde_json::from_value::<Vec<Connection>>(r.data).ok());
        if let Some(conns) = connections {
            cx.update_global::<ConnectionState, _>(|state, _cx| {
                state.connections = conns;
            });
        }
    }

    // Traffic (speed)
    {
        let traffic = cx.try_global::<CoreManager>()
            .and_then(|mgr| mgr.get_traffic(true).ok());
        if let Some(ref result) = traffic {
            if let Some(up) = result.data.get("up").and_then(|v| v.as_u64()) {
                cx.update_global::<AppState, _>(|state, _cx| { state.upload_speed = up; });
            }
            if let Some(down) = result.data.get("down").and_then(|v| v.as_u64()) {
                cx.update_global::<AppState, _>(|state, _cx| { state.download_speed = down; });
            }
        }
    }

    // Traffic (total)
    {
        let total_traffic = cx.try_global::<CoreManager>()
            .and_then(|mgr| mgr.service.get_total_traffic(true).ok());
        if let Some(ref result) = total_traffic {
            if let Some(up) = result.data.get("up").and_then(|v| v.as_u64()) {
                cx.update_global::<AppState, _>(|state, _cx| { state.upload_total = up; });
            }
            if let Some(down) = result.data.get("down").and_then(|v| v.as_u64()) {
                cx.update_global::<AppState, _>(|state, _cx| { state.download_total = down; });
            }
        }
    }

    // Profiles — poll current profile name
    {
        let profile_name = cx.try_global::<CoreManager>()
            .and_then(|mgr| mgr.get_current_profile_name().ok())
            .and_then(|r| r.data.get("profile-name").and_then(|v| v.as_str()).map(|s| s.to_string()));
        if let Some(ref name) = profile_name {
            cx.update_global::<ConfigState, _>(|state, _cx| {
                // Add the profile if not already known
                if !state.profiles.iter().any(|p| p.name == *name) {
                    state.add_profile(ConfigProfile {
                        name: name.clone(),
                        path: String::new(),
                        profile_type: crate::state::config::ProfileType::File,
                        updated_at: Some(chrono::Utc::now().timestamp()),
                        subscription_info: None,
                        config_content: None,
                    });
                }
            });
        }
    }
}

// ── Initialization / Teardown ─────────────────────────────────────

/// Called after core is started.
pub fn on_core_started(cx: &mut App) {
    let home_dir = crate::config_dir().to_string_lossy().to_string();
    let Some(manager) = cx.try_global::<CoreManager>() else { return };

    match manager.init_clash(&home_dir) {
        Ok(_) => {
            log::info!("[Bridge] Core initialized with home_dir={home_dir}");
            let _ = manager.invoke(
                crate::core::action::ActionMethod::StartLog,
                serde_json::Value::Null,
            );
        }
        Err(e) => log::error!("[Bridge] initClash failed: {e}"),
    }
}

pub fn on_core_stopped(_cx: &mut App) {}

// ── Profile management helpers ─────────────────────────────────────

/// Add a subscription URL as a new config profile.
/// Always adds to local state first (optimistic), then calls core as best-effort.
pub fn add_subscription(url: &str, cx: &mut App) {
    let name = url
        .rsplit('/')
        .next()
        .and_then(|s| {
            // Strip query params for display name
            let q = s.find('?').unwrap_or(s.len());
            if q > 0 { Some(&s[..q]) } else { None }
        })
        .unwrap_or(url)
        .to_string();

    // ── Optimistic local add ──
    cx.update_global::<ConfigState, _>(|state, _cx| {
        state.add_profile(ConfigProfile {
            name: name.clone(),
            path: url.to_string(),
            profile_type: crate::state::config::ProfileType::Url,
            updated_at: Some(chrono::Utc::now().timestamp()),
            subscription_info: None,
            config_content: None,
        });
    });
    log::info!("[Bridge] Added subscription (local): {url}");

    // ── Best-effort RPC to core ──
    let url_owned = url.to_string();
    let name_owned = name.clone();
    if let Some(manager) = cx.try_global::<CoreManager>() {
        match manager.setup_config(&url_owned) {
            Ok(r) if r.is_ok() => {
                log::info!("[Bridge] setup_config succeeded: {url}");
                // Try to fetch the actual config content
                fetch_profile_config_content(&url_owned, &name_owned, cx);
            }
            Ok(r) => log::error!("[Bridge] setup_config failed: code={}", r.code),
            Err(e) => log::error!("[Bridge] setup_config error: {e}"),
        }
    }
}

/// Fetch YAML content for a profile from the Go core and store it in ConfigState.
fn fetch_profile_config_content(path: &str, _name: &str, cx: &mut App) {
    let Some(manager) = cx.try_global::<CoreManager>() else { return };
    match manager.service.get_config(path) {
        Ok(r) if r.is_ok() => {
            if let Some(content) = r.data.as_str() {
                let content = content.to_string();
                cx.update_global::<ConfigState, _>(|state, _cx| {
                    if let Some(profile) = state.profiles.iter_mut().find(|p| p.path == path) {
                        profile.config_content = Some(content);
                    }
                });
            } else if let Some(obj) = r.data.as_object() {
                // Maybe returned as structured JSON — serialize as YAML-like
                if let Ok(pretty) = serde_json::to_string_pretty(obj) {
                    cx.update_global::<ConfigState, _>(|state, _cx| {
                        if let Some(profile) = state.profiles.iter_mut().find(|p| p.path == path) {
                            profile.config_content = Some(pretty);
                        }
                    });
                }
            }
            log::info!("[Bridge] Fetched config content for {path}");
        }
        Ok(r) => log::error!("[Bridge] get_config failed for {path}: code={}", r.code),
        Err(e) => log::error!("[Bridge] get_config error: {e}"),
    }
}

/// Update a subscription (re-fetch from URL).
pub fn update_subscription(url: &str, cx: &mut App) {
    let Some(manager) = cx.try_global::<CoreManager>() else { return };
    let params = serde_json::json!({"path": url});
    let url_owned = url.to_string();
    match manager.service.update_config(params) {
        Ok(r) if r.is_ok() => {
            cx.update_global::<ConfigState, _>(|state, _cx| {
                if let Some(profile) = state.profiles.iter_mut().find(|p| p.path == url) {
                    profile.updated_at = Some(chrono::Utc::now().timestamp());
                }
            });
            log::info!("[Bridge] Updated subscription: {url}");
            // Re-fetch config content after update
            let name = String::new();
            fetch_profile_config_content(&url_owned, &name, cx);
        }
        Ok(r) => log::error!("[Bridge] update_config failed: code={}", r.code),
        Err(e) => log::error!("[Bridge] update_config error: {e}"),
    }
}

/// Delete a profile by index, removing from core and ConfigState.
pub fn delete_profile(path: &str, idx: usize, cx: &mut App) {
    // Call core to delete the file (if it exists)
    if !path.is_empty() {
        if let Some(manager) = cx.try_global::<CoreManager>() {
            match manager.delete_file(path) {
                Ok(r) if r.is_ok() => log::info!("[Bridge] Deleted file: {path}"),
                Ok(r) => log::error!("[Bridge] delete_file failed for {path}: code={}", r.code),
                Err(e) => log::error!("[Bridge] delete_file error: {e}"),
            }
        }
    }
    // Remove from local state
    cx.update_global::<ConfigState, _>(|state, _cx| {
        state.remove_profile(idx);
    });
    log::info!("[Bridge] Removed profile at index {idx}");
}
