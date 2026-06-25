/// System tray icon with context menu.
///
/// Uses `tray-icon` for cross-platform tray support (Windows / macOS / Linux).
/// Events are communicated back to the GPUI main loop via an mpsc channel.
///
/// **Important**: The returned `TrayIcon` must be kept alive for as long as
/// the tray icon should remain visible.  Dropping it removes the icon.
use std::sync::mpsc;

use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIconBuilder};

// ── Public types ────────────────────────────────────────────────

/// Events emitted by tray menu interactions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayEvent {
    /// User clicked "Show Window" or left-clicked the tray icon.
    ShowWindow,
    /// User clicked "Toggle System Proxy".
    ToggleProxy,
    /// User clicked "Quit".
    Quit,
}

// ── Public API ─────────────────────────────────────────────────

/// Create the system tray icon and menu.  Call once **on the main thread**
/// before entering the GPUI event loop.
///
/// Returns the tray handle (keep alive!) and a channel receiver for
/// tray-initiated events.
pub fn init() -> (tray_icon::TrayIcon, mpsc::Receiver<TrayEvent>) {
    let (tx, rx) = mpsc::channel();

    let menu = build_menu(tx);
    let icon = load_tray_icon().expect("failed to load tray icon");

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Hopen")
        .with_icon(icon)
        .build()
        .expect("failed to create tray icon");

    (tray, rx)
}

// ── Menu construction ──────────────────────────────────────────

fn build_menu(tx: mpsc::Sender<TrayEvent>) -> Menu {
    let menu = Menu::new();

    let show = MenuItem::new(
        "Show Window",
        true,
        None::<muda::accelerator::Accelerator>,
    );
    let toggle = MenuItem::new(
        "Toggle System Proxy",
        true,
        None::<muda::accelerator::Accelerator>,
    );
    let separator = PredefinedMenuItem::separator();
    let quit = MenuItem::new("Quit", true, None::<muda::accelerator::Accelerator>);

    let show_id = show.id().clone();
    let toggle_id = toggle.id().clone();
    let quit_id = quit.id().clone();

    menu.append_items(&[&show, &toggle, &separator, &quit])
        .expect("append tray menu items");

    // Listen for menu events on a background thread so we don't block
    // the platform event loop.
    std::thread::spawn(move || {
        if let Ok(menu_rx) = MenuEvent::receiver().try_recv() {
            // receiver() returns a Result, but in this design we always
            // get the global receiver.  We keep the thread alive for the
            // lifetime of the app.
            drop(menu_rx);
        }
        loop {
            // Block until a menu event arrives.
            if let Ok(event) = MenuEvent::receiver().recv() {
                let ev = if event.id == show_id {
                    TrayEvent::ShowWindow
                } else if event.id == toggle_id {
                    TrayEvent::ToggleProxy
                } else if event.id == quit_id {
                    TrayEvent::Quit
                } else {
                    continue;
                };
                if tx.send(ev).is_err() {
                    break;
                }
            }
        }
    });

    menu
}

// ── Icon loading ───────────────────────────────────────────────

/// Load the application icon from the embedded resource and resize it to
/// a suitable tray-icon size (64×64 RGBA).
fn load_tray_icon() -> Result<Icon, String> {
    // Try loading the icon from the resources directory next to the exe,
    // fall back to an embedded 1-pixel placeholder.
    let rgba = load_png_to_rgba("resources/app-icon.png", 64)
        .or_else(|_| load_png_to_rgba("app-icon.png", 64))
        .unwrap_or_else(|_| {
            // Fallback: solid colour square (teal).
            let side = 64;
            let mut pixels = Vec::with_capacity(side * side * 4);
            for _ in 0..side * side {
                pixels.extend_from_slice(&[0x00, 0x96, 0x88, 0xFF]);
            }
            pixels
        });

    Icon::from_rgba(rgba, 64, 64).map_err(|e| format!("icon from_rgba: {e}"))
}

/// Decode a PNG file into RGBA pixel bytes, resizing to `max_side` pixels.
fn load_png_to_rgba(path: &str, max_side: u32) -> Result<Vec<u8>, String> {
    use image::imageops::FilterType;
    use image::GenericImageView;

    let img = image::open(path).map_err(|e| format!("open '{path}': {e}"))?;
    let (w, h) = img.dimensions();
    let scale = (max_side as f64 / w.max(h) as f64).min(1.0);
    let nw = (w as f64 * scale) as u32;
    let nh = (h as f64 * scale) as u32;
    let resized = img.resize_exact(nw, nh, FilterType::Lanczos3);
    Ok(resized.to_rgba8().into_raw())
}
