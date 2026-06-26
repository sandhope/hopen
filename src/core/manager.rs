#![allow(dead_code)]
/// Core manager — GPUI-aware wrapper around CoreService.
///
/// This module bridges the lower-level `CoreService` (background threads,
/// IPC, process management) with the GPUI application event loop.
///
/// # Binary embedding
///
/// In **release** builds, the Go core binary (`FlClashCore`) is embedded
/// into the Rust binary via `include_bytes!`. On first launch, it is
/// extracted to the app data directory. This ensures the final package
/// is a single executable with no external dependencies.
///
/// In **debug** builds, the core is searched next to the executable
/// (for fast iteration).
use gpui::Global;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::action::{ActionResult, ActionMethod};
use crate::core::event::CoreEventListener;
use crate::core::service::{CoreService, CoreStatus};

// ── Embedded core binary (release only) ───────────────────────────

/// Name of the Go core binary (platform-dependent).
#[cfg(windows)]
const CORE_EXE_NAME: &str = "FlClashCore.exe";
#[cfg(not(windows))]
const CORE_EXE_NAME: &str = "FlClashCore";

/// In release mode, the Go core binary is compiled by build.rs and
/// embedded directly into the Rust binary.
///
/// `core_bin_disabled` is set by build.rs when Go toolchain is missing.
#[cfg(all(not(debug_assertions), not(core_bin_disabled)))]
mod embedded {
    #[cfg(windows)]
    pub const DATA: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/FlClashCore.exe"));
    #[cfg(not(windows))]
    pub const DATA: &[u8] =
        include_bytes!(concat!(env!("OUT_DIR"), "/FlClashCore"));
}

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

/// Resolve the Go core binary path.
///
/// Resolution order:
/// 1. Next to the Rust executable (convenience for debug/dev)
/// 2. Extract embedded binary from release build to data directory
/// 3. Current working directory
pub fn resolve_core_path() -> Option<String> {
    // 1. Check next to the executable (developer convenience)
    if let Some(p) = find_core_next_to_exe() {
        log::info!("[CoreManager] found core next to exe: {}", p.display());
        return Some(p.to_string_lossy().to_string());
    }

    // 2. Extract embedded binary (release builds only)
    #[cfg(not(debug_assertions))]
    if let Some(p) = extract_embedded_core() {
        log::info!("[CoreManager] extracted embedded core to: {}", p.display());
        return Some(p.to_string_lossy().to_string());
    }

    // 3. Check cached extraction from previous run
    if let Some(p) = cached_core_path() {
        if p.exists() {
            log::info!("[CoreManager] found cached core: {}", p.display());
            return Some(p.to_string_lossy().to_string());
        }
    }

    // 4. Current working directory (fallback)
    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join(CORE_EXE_NAME);
        if candidate.exists() {
            return Some(candidate.to_string_lossy().to_string());
        }
    }

    log::error!("[CoreManager] core binary not found ({})", CORE_EXE_NAME);
    None
}

/// Look for FlClashCore next to the current executable.
fn find_core_next_to_exe() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    let candidate = dir.join(CORE_EXE_NAME);
    if candidate.exists() {
        Some(candidate)
    } else {
        None
    }
}

/// Get the cached core path in the app data directory.
fn cached_core_path() -> Option<PathBuf> {
    let data_dir = dirs::data_dir()?;
    Some(data_dir.join("hopen").join("core").join(CORE_EXE_NAME))
}

/// Extract the embedded Go core binary to the app data directory.
///
/// This is called on first launch (or after app update).
/// The extracted binary is cached for subsequent launches.
#[cfg(not(debug_assertions))]
fn extract_embedded_core() -> Option<PathBuf> {
    // In debug builds, or if Go wasn't available at build time,
    // the embedded module isn't available.
    #[cfg(core_bin_disabled)]
    {
        return None;
    }

    #[cfg(not(core_bin_disabled))]
    {
        let target = cached_core_path()?;

        // Only extract if the cached binary is missing or outdated
        // (compare size as a quick heuristic; full hash check too expensive)
        let need_extract = if target.exists() {
            if let Ok(meta) = std::fs::metadata(&target) {
                meta.len() != embedded::DATA.len() as u64
            } else {
                true
            }
        } else {
            true
        };

        if need_extract {
            // Ensure parent directories exist
            if let Some(parent) = target.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    log::error!(
                        "[CoreManager] failed to create core cache dir: {}",
                        e
                    );
                    return None;
                }
            }

            if let Err(e) = std::fs::write(&target, embedded::DATA) {
                log::error!("[CoreManager] failed to extract core binary: {}", e);
                return None;
            }

            log::info!(
                "[CoreManager] extracted core binary ({} bytes) to {}",
                embedded::DATA.len(),
                target.display()
            );
        }

        // Make executable on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(
                &target,
                std::fs::Permissions::from_mode(0o755),
            );
        }

        Some(target)
    }
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
