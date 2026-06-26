/// Core manager — GPUI-aware wrapper around CoreService.
///
/// This module bridges the lower-level `CoreService` (background threads,
/// IPC, process management) with the GPUI application event loop.
use gpui::Global;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::action::{ActionResult, ActionMethod};
use crate::core::event::CoreEventListener;
use crate::core::service::{CoreService, CoreStatus};

// ── Random address generation ────────────────────────────────────

/// Generate a random named-pipe path (Windows) or Unix socket path.
pub fn random_address() -> String {
    let rng = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos()
        % 10000;
    #[cfg(windows)]
    {
        format!("\\\\.\\pipe\\hopen_core_{rng}")
    }
    #[cfg(not(windows))]
    {
        format!("/tmp/hopen_core_{rng}.sock")
    }
}

/// Resolve the Go core binary path relative to the current executable.
pub fn resolve_core_path() -> Option<String> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;

    #[cfg(windows)]
    let name = "FlClashCore.exe";
    #[cfg(not(windows))]
    let name = "FlClashCore";

    let candidate = dir.join(name);
    if candidate.exists() {
        return Some(candidate.to_string_lossy().to_string());
    }

    // Fallback: try current directory
    let cwd = std::env::current_dir().ok()?;
    let cwd_candidate = cwd.join(name);
    if cwd_candidate.exists() {
        return Some(cwd_candidate.to_string_lossy().to_string());
    }

    None
}

// ── CoreManager ──────────────────────────────────────────────────

/// Application-level wrapper for the core service.
///
/// Stored as a GPUI Global so any view can access it.
pub struct CoreManager {
    /// The underlying core service.
    pub service: CoreService,
    /// Path to the Go core binary (cached after first resolution).
    pub core_path: Option<String>,
    /// IPC address used for this session.
    pub address: String,
}

impl CoreManager {
    /// Create a new manager with a random IPC address.
    pub fn new() -> Self {
        let address = random_address();
        let core_path = resolve_core_path();

        log::info!(
            "[CoreManager] address={address}, core={}",
            core_path.as_deref().unwrap_or("not found")
        );

        Self {
            service: CoreService::new(address.clone()),
            core_path,
            address,
        }
    }

    /// Start the Go core if it's available.
    pub fn start(&self) -> Result<(), String> {
        let path = self
            .core_path
            .as_deref()
            .ok_or_else(|| "Core binary not found".to_string())?;
        self.service.start(path)
    }

    /// Stop the Go core.
    pub fn stop(&self) {
        self.service.stop();
    }

    /// Restart the Go core.
    pub fn restart(&self) -> Result<(), String> {
        self.service.restart()
    }

    /// Get current connection status.
    pub fn status(&self) -> CoreStatus {
        self.service.status()
    }

    /// Register an event listener to receive core events (logs, delays, …).
    pub fn add_listener(&self, listener: Arc<dyn CoreEventListener>) {
        self.service.event_manager().add_listener(listener);
    }

    // ── Convenience RPC shortcuts ─────────────────────────────────

    pub fn invoke(&self, method: ActionMethod, data: serde_json::Value) -> Result<ActionResult, String> {
        self.service.invoke(method, data)
    }

    pub fn init_clash(&self, home_dir: &str) -> Result<ActionResult, String> {
        self.service.init_clash(home_dir)
    }

    pub fn get_proxies(&self) -> Result<ActionResult, String> {
        self.service.get_proxies()
    }

    pub fn change_proxy(&self, group: &str, proxy: &str) -> Result<ActionResult, String> {
        self.service.change_proxy(group, proxy)
    }

    pub fn get_traffic(&self, stats_only: bool) -> Result<ActionResult, String> {
        self.service.get_traffic(stats_only)
    }

    pub fn async_test_delay(&self, url: &str, proxy: &str) -> Result<ActionResult, String> {
        self.service.async_test_delay(url, proxy)
    }

    pub fn get_connections(&self) -> Result<ActionResult, String> {
        self.service.get_connections()
    }

    pub fn close_connection(&self, id: &str) -> Result<ActionResult, String> {
        self.service.close_connection(id)
    }

    pub fn close_all_connections(&self) -> Result<ActionResult, String> {
        self.service.close_all_connections()
    }

    pub fn get_current_profile_name(&self) -> Result<ActionResult, String> {
        self.service.get_current_profile_name()
    }

    pub fn setup_config(&self, path: &str) -> Result<ActionResult, String> {
        self.service.setup_config(path)
    }

    pub fn delete_file(&self, path: &str) -> Result<ActionResult, String> {
        self.service.delete_file(path)
    }
}

impl Global for CoreManager {}

impl Default for CoreManager {
    fn default() -> Self {
        Self::new()
    }
}
