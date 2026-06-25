/// Platform-native feature abstractions.
///
/// Each sub-module provides OS-adapted implementations:
/// - `tray`       – system tray icon + context menu
/// - `proxy`      – system-wide proxy configuration
/// - `autostart`  – launch-on-boot registration
/// - `admin`      – privilege detection + elevation
pub mod admin;
pub mod autostart;
pub mod proxy;
pub mod tray;

pub use tray::TrayEvent;
