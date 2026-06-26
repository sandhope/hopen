/// Core service — manages the Go clash.meta process lifecycle and provides
/// a high-level RPC interface for all core operations.
use std::collections::HashMap;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::core::action::*;
use crate::core::event::CoreEventManager;

// ── Core Status ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreStatus {
    Disconnected,
    Connecting,
    Connected,
}

// ── Core Service ─────────────────────────────────────────────────

type PendingMap = Arc<Mutex<HashMap<String, Sender<ActionResult>>>>;

pub struct CoreService {
    /// Socket / pipe address for IPC.
    address: String,
    /// Path to the Go core binary.
    core_path: Mutex<Option<String>>,
    /// Channel to send raw bytes to the IPC writer thread.
    write_tx: Mutex<Option<Sender<Vec<u8>>>>,
    /// Pending RPC calls keyed by request id.
    pending: PendingMap,
    /// The Go core child process.
    child: Mutex<Option<Child>>,
    /// Current connection status.
    status: Mutex<CoreStatus>,
    /// Event manager for unsolicited events.
    event_manager: Arc<CoreEventManager>,
    /// Shutdown signal for the IPC thread.
    shutdown_tx: Mutex<Option<Sender<()>>>,
}

impl CoreService {
    /// Create a new CoreService.
    ///
    /// `address` should be a named-pipe path on Windows
    /// (`\\.\pipe\FlClashCore_<random>`) or a Unix socket path.
    pub fn new(address: String) -> Self {
        Self {
            address,
            core_path: Mutex::new(None),
            write_tx: Mutex::new(None),
            pending: Arc::new(Mutex::new(HashMap::new())),
            child: Mutex::new(None),
            status: Mutex::new(CoreStatus::Disconnected),
            event_manager: Arc::new(CoreEventManager::new()),
            shutdown_tx: Mutex::new(None),
        }
    }

    /// Access the event manager for registering listeners.
    pub fn event_manager(&self) -> &Arc<CoreEventManager> {
        &self.event_manager
    }

    /// Get current connection status.
    pub fn status(&self) -> CoreStatus {
        *self.status.lock().unwrap()
    }

    // ── Lifecycle ─────────────────────────────────────────────────

    /// Start the IPC server and launch the Go core.
    ///
    /// This spawns two background threads:
    /// 1. IPC listener + reader/writer thread
    /// 2. Data dispatch thread (parses JSON, matches responses, routes events)
    pub fn start(&self, core_path: &str) -> Result<(), String> {
        // Stop any existing instance first
        self.stop();

        *self.core_path.lock().unwrap() = Some(core_path.to_string());

        let addr = self.address.clone();
        let cp = core_path.to_string();

        // ── Channels ──────────────────────────────────────────
        // data_tx: IPC reader → dispatch thread
        let (data_tx, data_rx) = mpsc::channel::<Vec<u8>>();
        // write_rx: app → IPC writer
        let (write_tx, write_rx) = mpsc::channel::<Vec<u8>>();
        // ready_tx: IPC listener → main (signal listener is up)
        let (ready_tx, ready_rx) = mpsc::channel::<()>();
        // shutdown_tx: main → IPC (request shutdown)
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

        *self.write_tx.lock().unwrap() = Some(write_tx);
        *self.shutdown_tx.lock().unwrap() = Some(shutdown_tx);
        *self.status.lock().unwrap() = CoreStatus::Connecting;

        // ── IPC server thread ─────────────────────────────────
        thread::Builder::new()
            .name("hopen-ipc".into())
            .spawn(move || {
                ipc_server_loop(addr, ready_tx, data_tx, write_rx, shutdown_rx);
            })
            .map_err(|e| format!("Failed to spawn IPC thread: {e}"))?;

        // Wait for IPC server to be ready
        ready_rx
            .recv_timeout(Duration::from_secs(5))
            .map_err(|_| "IPC server did not become ready in time".to_string())?;

        // ── Start Go core process ─────────────────────────────
        let addr_arg = self.address.clone();
        let child = Command::new(&cp)
            .arg(&addr_arg)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start core '{cp}': {e}"))?;

        *self.child.lock().unwrap() = Some(child);
        *self.status.lock().unwrap() = CoreStatus::Connected;
        log::info!("[Core] Go process started, IPC connected");

        // ── Dispatch thread ───────────────────────────────────
        let pending = Arc::clone(&self.pending);
        let ev_mgr = Arc::clone(&self.event_manager);

        thread::Builder::new()
            .name("hopen-dispatch".into())
            .spawn(move || {
                dispatch_loop(data_rx, pending, ev_mgr);
            })
            .map_err(|e| format!("Failed to spawn dispatch thread: {e}"))?;

        Ok(())
    }

    /// Stop the Go core and tear down IPC.
    pub fn stop(&self) {
        // Signal IPC thread to shut down
        if let Ok(mut guard) = self.shutdown_tx.lock() {
            if let Some(tx) = guard.take() {
                let _ = tx.send(());
            }
        }

        // Kill child process
        if let Ok(mut guard) = self.child.lock() {
            if let Some(ref mut child) = *guard {
                let _ = child.kill();
                let _ = child.wait();
            }
            *guard = None;
        }

        // Clear pending callbacks
        if let Ok(mut guard) = self.pending.lock() {
            guard.clear();
        }

        // Clear write channel
        if let Ok(mut guard) = self.write_tx.lock() {
            *guard = None;
        }

        *self.status.lock().unwrap() = CoreStatus::Disconnected;
        log::info!("[Core] stopped");
    }

    /// Restart the core.
    pub fn restart(&self) -> Result<(), String> {
        let cp = self
            .core_path
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| "No core path set".to_string())?;
        self.stop();
        thread::sleep(Duration::from_millis(500));
        self.start(&cp)
    }

    // ── RPC ───────────────────────────────────────────────────────

    /// Send an action to the Go core and wait for the response.
    pub fn invoke(&self, method: ActionMethod, data: serde_json::Value) -> Result<ActionResult, String> {
        let action = Action::new(method, data);
        let json =
            serde_json::to_string(&action).map_err(|e| format!("JSON encode error: {e}"))?;

        let (tx, rx) = mpsc::channel::<ActionResult>();
        {
            let mut pending = self.pending.lock().unwrap();
            pending.insert(action.id.clone(), tx);
        }

        // Send to IPC
        {
            let guard = self.write_tx.lock().unwrap();
            let tx = guard
                .as_ref()
                .ok_or_else(|| "IPC not connected".to_string())?;
            tx.send(json.into_bytes())
                .map_err(|e| format!("IPC send error: {e}"))?;
        }

        // Wait for response
        let result = rx.recv_timeout(Duration::from_secs(15)).map_err(|_| {
            if let Ok(mut pending) = self.pending.lock() {
                pending.remove(&action.id);
            }
            format!("RPC timeout for {method}")
        })?;

        Ok(result)
    }

    /// Fire-and-forget (no response expected).
    pub fn invoke_fire(&self, method: ActionMethod, data: serde_json::Value) -> Result<(), String> {
        let action = Action::new(method, data);
        let json =
            serde_json::to_string(&action).map_err(|e| format!("JSON encode error: {e}"))?;

        let guard = self.write_tx.lock().unwrap();
        let tx = guard
            .as_ref()
            .ok_or_else(|| "IPC not connected".to_string())?;
        tx.send(json.into_bytes())
            .map_err(|e| format!("IPC send error: {e}"))?;
        Ok(())
    }

    // ── Convenience API ───────────────────────────────────────────

    pub fn init_clash(&self, home_dir: &str) -> Result<ActionResult, String> {
        let params = InitParams {
            home_dir: home_dir.to_string(),
            version: 1,
        };
        self.invoke(
            ActionMethod::InitClash,
            serde_json::to_value(params).unwrap_or_default(),
        )
    }

    pub fn get_proxies(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetProxies, serde_json::Value::Null)
    }

    pub fn change_proxy(&self, group: &str, proxy: &str) -> Result<ActionResult, String> {
        self.invoke(
            ActionMethod::ChangeProxy,
            serde_json::json!({"group-name": group, "proxy-name": proxy}),
        )
    }

    pub fn get_traffic(&self, stats_only: bool) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetTraffic, serde_json::Value::Bool(stats_only))
    }

    pub fn get_total_traffic(&self, stats_only: bool) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetTotalTraffic, serde_json::Value::Bool(stats_only))
    }

    pub fn async_test_delay(&self, url: &str, proxy: &str) -> Result<ActionResult, String> {
        self.invoke(
            ActionMethod::AsyncTestDelay,
            serde_json::json!({"proxy-name": proxy, "test-url": url, "timeout": 5000}),
        )
    }

    pub fn get_connections(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetConnections, serde_json::Value::Null)
    }

    pub fn close_connection(&self, id: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::CloseConnection, serde_json::Value::String(id.into()))
    }

    pub fn close_all_connections(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::CloseConnections, serde_json::Value::Null)
    }

    pub fn validate_config(&self, path: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::ValidateConfig, serde_json::Value::String(path.into()))
    }

    pub fn get_config(&self, path: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetConfig, serde_json::Value::String(path.into()))
    }

    pub fn update_config(&self, params: serde_json::Value) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::UpdateConfig, params)
    }

    pub fn is_init(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetIsInit, serde_json::Value::Null)
    }

    pub fn force_gc(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::ForceGc, serde_json::Value::Null)
    }

    pub fn shutdown_core(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::Shutdown, serde_json::Value::Null)
    }

    pub fn reset_traffic(&self) -> Result<(), String> {
        self.invoke_fire(ActionMethod::ResetTraffic, serde_json::Value::Null)
    }

    pub fn start_log(&self) -> Result<(), String> {
        self.invoke_fire(ActionMethod::StartLog, serde_json::Value::Null)
    }

    pub fn stop_log(&self) -> Result<(), String> {
        self.invoke_fire(ActionMethod::StopLog, serde_json::Value::Null)
    }

    pub fn get_memory(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetMemory, serde_json::Value::Null)
    }

    pub fn get_country_code(&self, ip: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetCountryCode, serde_json::Value::String(ip.into()))
    }

    pub fn get_external_providers(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetExternalProviders, serde_json::Value::Null)
    }

    pub fn get_external_provider(&self, name: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetExternalProvider, serde_json::Value::String(name.into()))
    }

    pub fn update_external_provider(&self, name: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::UpdateExternalProvider, serde_json::Value::String(name.into()))
    }

    pub fn update_geo_data(&self, geo_type: &str, geo_name: &str) -> Result<ActionResult, String> {
        self.invoke(
            ActionMethod::UpdateGeoData,
            serde_json::json!({"geo-type": geo_type, "geo-name": geo_name}),
        )
    }

    pub fn delete_file(&self, path: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::DeleteFile, serde_json::Value::String(path.into()))
    }

    pub fn get_current_profile_name(&self) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::GetCurrentProfileName, serde_json::Value::Null)
    }

    pub fn setup_config(&self, path: &str) -> Result<ActionResult, String> {
        self.invoke(ActionMethod::SetupConfig, serde_json::json!({"path": path}))
    }
}

impl Drop for CoreService {
    fn drop(&mut self) {
        self.stop();
    }
}

// ── IPC Server Loop (background thread) ──────────────────────────

fn ipc_server_loop(
    name: String,
    ready_tx: Sender<()>,
    data_tx: Sender<Vec<u8>>,
    write_rx: Receiver<Vec<u8>>,
    shutdown_rx: Receiver<()>,
) {
    use interprocess::local_socket::prelude::*;
    use interprocess::local_socket::{GenericFilePath, ListenerNonblockingMode, ListenerOptions};
    use std::io::{self, Read};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc as StdArc;

    // Convert to filesystem socket name
    let fs_name = match name.clone().to_fs_name::<GenericFilePath>() {
        Ok(n) => n,
        Err(e) => {
            log::error!("[IPC] name error: {e}");
            return;
        }
    };

    let listener = match ListenerOptions::new().name(fs_name).create_sync() {
        Ok(l) => l,
        Err(e) => {
            log::error!("[IPC] bind error: {e}");
            return;
        }
    };

    if let Err(e) = listener.set_nonblocking(ListenerNonblockingMode::Accept) {
        log::error!("[IPC] nonblocking error: {e}");
        return;
    }

    log::info!("[IPC] ready on {name}");
    let _ = ready_tx.send(());

    // Accept one connection with shutdown check
    let stream = loop {
        // Check shutdown before blocking
        if shutdown_rx.try_recv().is_ok() {
            log::info!("[IPC] shutdown before accept");
            cleanup_ipc_socket(&name);
            return;
        }

        match listener.accept() {
            Ok(s) => break s,
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                log::error!("[IPC] accept error: {e}");
                cleanup_ipc_socket(&name);
                return;
            }
        }
    };

    if let Err(e) = stream.set_nonblocking(false) {
        log::error!("[IPC] stream mode error: {e}");
        cleanup_ipc_socket(&name);
        return;
    }

    let (recv_half, send_half) = stream.split();

    // Writer thread: app → Go core
    let wr_flag = StdArc::new(AtomicBool::new(true));
    let wr = StdArc::clone(&wr_flag);

    thread::spawn(move || {
        let mut sender = send_half;
        while wr.load(Ordering::SeqCst) {
            match write_rx.recv() {
                Ok(data) => {
                    let len = data.len() as u32;
                    if sender.write_all(&len.to_le_bytes()).is_err()
                        || sender.write_all(&data).is_err()
                        || sender.flush().is_err()
                    {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        wr.store(false, Ordering::SeqCst);
    });

    // Reader loop: Go core → app
    let mut receiver = recv_half;
    loop {
        // Check for shutdown between frames
        if shutdown_rx.try_recv().is_ok() {
            log::info!("[IPC] shutdown during read");
            break;
        }

        let mut len_buf = [0u8; 4];
        if receiver.read_exact(&mut len_buf).is_err() {
            break;
        }
        let len = u32::from_le_bytes(len_buf) as usize;
        if len > 16 * 1024 * 1024 {
            log::error!("[IPC] frame too large: {len}");
            break;
        }
        let mut payload = vec![0u8; len];
        if receiver.read_exact(&mut payload).is_err() {
            break;
        }
        if data_tx.send(payload).is_err() {
            break;
        }
    }

    wr_flag.store(false, Ordering::SeqCst);
    cleanup_ipc_socket(&name);
    log::info!("[IPC] server stopped");
}

// ── Dispatch Loop (background thread) ────────────────────────────

fn dispatch_loop(
    data_rx: Receiver<Vec<u8>>,
    pending: PendingMap,
    event_manager: Arc<CoreEventManager>,
) {
    for payload in data_rx {
        let json_str = match std::str::from_utf8(&payload) {
            Ok(s) => s.trim().to_owned(),
            Err(e) => {
                log::error!("[Dispatch] invalid UTF-8: {e}");
                continue;
            }
        };

        let result: ActionResult = match serde_json::from_str(&json_str) {
            Ok(r) => r,
            Err(e) => {
                log::error!("[Dispatch] JSON parse error: {e} — {}", &json_str[..json_str.len().min(200)]);
                continue;
            }
        };

        // Check if this is an unsolicited event (empty id)
        if result.is_event() {
            event_manager.dispatch_event(&result);
            continue;
        }

        // Match to a pending RPC callback
        if let Ok(mut guard) = pending.lock() {
            if let Some(tx) = guard.remove(&result.id) {
                let _ = tx.send(result);
            }
        }
    }

    log::info!("[Dispatch] stopped");
}

// ── Helpers ──────────────────────────────────────────────────────

#[cfg(unix)]
fn cleanup_ipc_socket(path: &str) {
    let p = std::path::Path::new(path);
    if p.exists() {
        let _ = std::fs::remove_file(p);
    }
}

#[cfg(not(unix))]
fn cleanup_ipc_socket(_path: &str) {}
