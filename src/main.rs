/// Hopen GPUI — A multi-platform proxy client.
///
/// Entry point: creates the GPUI application, opens the main window,
/// and wires up global state and keyboard shortcuts.

mod app;
mod assets;
mod components;
mod core;
mod i18n;
mod navigation;
mod persistence;
mod state;
mod theme;
mod views;

use gpui::*;

use app::AppView;
use assets::Assets;
use core::manager::CoreManager;
use i18n::I18nManager;
use navigation::Page;
use persistence::Database;
use state::config::ConfigState;
use state::connection::ConnectionState;
use state::log::LogState;
use state::proxy::ProxyState;
use theme::{AccentColor, Theme, ThemeMode};

// ─── Outbound Mode ────────────────────────────────────────────

/// Proxy outbound routing mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OutboundMode {
    /// Route all traffic through the proxy.
    #[default]
    Global,
    /// Use rule-based routing.
    Rule,
    /// Bypass the proxy entirely.
    Direct,
}

impl OutboundMode {
    pub fn label(self) -> &'static str {
        match self {
            OutboundMode::Global => "Global",
            OutboundMode::Rule => "Rule",
            OutboundMode::Direct => "Direct",
        }
    }

    /// Cycle to the next mode: Global → Rule → Direct → Global.
    pub fn next(self) -> Self {
        match self {
            OutboundMode::Global => OutboundMode::Rule,
            OutboundMode::Rule => OutboundMode::Direct,
            OutboundMode::Direct => OutboundMode::Global,
        }
    }
}

// ─── Global State ──────────────────────────────────────────────

/// Application-wide state accessible from any context.
pub struct AppState {
    /// Whether the proxy core is currently running.
    pub core_running: bool,
    /// Current system proxy status.
    pub system_proxy: bool,
    /// TUN mode status.
    pub tun_mode: bool,
    /// Active colour theme (dark / light).
    pub theme_mode: ThemeMode,
    /// Selected accent colour preset.
    pub accent_color: AccentColor,
    /// Current outbound routing mode.
    pub outbound_mode: OutboundMode,
    /// Network traffic statistics (placeholder).
    pub upload_speed: u64,
    pub download_speed: u64,
    pub upload_total: u64,
    pub download_total: u64,
    /// Network info (placeholder).
    pub public_ip: Option<String>,
    pub lan_ip: Option<String>,
    pub isp: Option<String>,
}

impl Global for AppState {}

/// Convenience: read the current `Theme` palette from global state.
pub fn current_theme(cx: &App) -> Theme {
    let state = cx.global::<AppState>();
    let mode = match state.theme_mode {
        ThemeMode::System => match cx.window_appearance() {
            WindowAppearance::Dark | WindowAppearance::VibrantDark => ThemeMode::Dark,
            _ => ThemeMode::Light,
        },
        other => other,
    };
    Theme::from_mode(mode, state.accent_color)
}

// ─── Keyboard Actions ──────────────────────────────────────────

actions!(
    hopen,
    [
        Quit,
        NavigateToDashboard,
        NavigateToProxies,
        NavigateToProfiles,
        NavigateToTools,
        ToggleTheme,
        ToggleCore,
        ToggleSystemProxy,
        ToggleTunMode,
        SetOutboundMode,
    ]
);

// ─── Main ──────────────────────────────────────────────────────

fn main() {
    // ── Open embedded database early ───────────────────────────────
    let data_dir = config_dir();
    let db = Database::open(data_dir.clone()).expect("failed to open database");

    // Load saved preferences so AppState starts with persisted values.
    let prefs = persistence::preferences::PreferencesStore::new(&db);
    let saved_theme = prefs.theme_mode().unwrap_or_default();
    let saved_accent = prefs.accent_color().unwrap_or_default();
    let saved_lang = prefs.language_id();

    Application::new()
        .with_assets(Assets::new())
        .run(move |cx: &mut App| {
        // Initialize global state
        cx.set_global(AppState {
            core_running: false,
            system_proxy: false,
            tun_mode: false,
            theme_mode: parse_theme_mode(&saved_theme),
            accent_color: parse_accent_color(&saved_accent),
            outbound_mode: OutboundMode::default(),
            upload_speed: 0,
            download_speed: 0,
            upload_total: 0,
            download_total: 0,
            public_ip: None,
            lan_ip: None,
            isp: None,
        });

        // Register the database for use by action handlers.
        cx.set_global(db);

        // Initialize core engine manager (IPC + Go process management)
        cx.set_global(CoreManager::new());

        // Initialize domain state types
        cx.set_global(ConfigState::default());
        cx.set_global(ProxyState::default());
        cx.set_global(ConnectionState::default());
        cx.set_global(LogState::default());

        // Initialize i18n — auto-detect system language, prefer persisted
        let lang_id = saved_lang.unwrap_or_else(|| i18n::detect_system_language_id());
        I18nManager::init_with_language_id(cx, &lang_id);

        // Bind keyboard shortcuts
        cx.bind_keys([
            KeyBinding::new("ctrl-q", Quit, None),
            KeyBinding::new("ctrl-t", ToggleTheme, None),
            KeyBinding::new("ctrl-1", NavigateToDashboard, None),
            KeyBinding::new("ctrl-2", NavigateToProxies, None),
            KeyBinding::new("ctrl-3", NavigateToProfiles, None),
            KeyBinding::new("ctrl-comma", NavigateToTools, None),
        ]);

        // Register action handlers
        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });

        cx.on_action(|_: &NavigateToDashboard, cx| {
            navigate_to(cx, Page::Dashboard);
        });

        cx.on_action(|_: &NavigateToProxies, cx| {
            navigate_to(cx, Page::Proxies);
        });

        cx.on_action(|_: &NavigateToProfiles, cx| {
            navigate_to(cx, Page::Profiles);
        });

        cx.on_action(|_: &NavigateToTools, cx| {
            navigate_to(cx, Page::Tools);
        });

        cx.on_action(|_: &ToggleTheme, cx| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.theme_mode = state.theme_mode.toggle();
            });
            save_theme_mode(cx);
            cx.refresh_windows();
        });

        cx.on_action(|_: &ToggleCore, cx| {
            let mut core_running = false;
            cx.update_global::<AppState, _>(|state, _cx| {
                state.core_running = !state.core_running;
                core_running = state.core_running;
            });

            // Actually start/stop the Go core engine
            if core_running {
                if let Some(manager) = cx.try_global::<CoreManager>() {
                    match manager.start() {
                        Ok(()) => log::info!("Core engine started"),
                        Err(e) => {
                            log::error!("Failed to start core: {e}");
                            // Revert state
                            cx.update_global::<AppState, _>(|state, _cx| {
                                state.core_running = false;
                            });
                        }
                    }
                }
            } else {
                if let Some(manager) = cx.try_global::<CoreManager>() {
                    manager.stop();
                }
            }
            cx.refresh_windows();
        });

        cx.on_action(|_: &ToggleSystemProxy, cx| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.system_proxy = !state.system_proxy;
            });
            cx.refresh_windows();
        });

        cx.on_action(|_: &ToggleTunMode, cx| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.tun_mode = !state.tun_mode;
            });
            cx.refresh_windows();
        });

        cx.on_action(|_: &SetOutboundMode, cx| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.outbound_mode = state.outbound_mode.next();
            });
            cx.refresh_windows();
        });

        // Open main window
        let window_bounds = Bounds::centered(None, size(px(1100.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Hopen")),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_background: WindowBackgroundAppearance::Opaque,
                window_min_size: Some(size(px(800.0), px(500.0))),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| AppView::new(window, cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}

// ─── Config Persistence ──────────────────────────────────────

/// Returns the Hopen data directory.
/// Windows → `%APPDATA%/hopen`, Unix → `$HOME/.config/hopen`.
pub(crate) fn config_dir() -> std::path::PathBuf {
    #[cfg(windows)]
    {
        std::env::var("APPDATA")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("hopen")
    }
    #[cfg(not(windows))]
    {
        std::env::var("HOME")
            .or_else(|_| std::env::var("XDG_CONFIG_HOME"))
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join(".config")
            .join("hopen")
    }
}

/// Parse a theme mode string from sled.
fn parse_theme_mode(s: &str) -> ThemeMode {
    match s.trim().to_lowercase().as_str() {
        "dark" => ThemeMode::Dark,
        "light" => ThemeMode::Light,
        "system" => ThemeMode::System,
        _ => ThemeMode::default(),
    }
}

/// Parse an accent colour string from sled.
fn parse_accent_color(s: &str) -> AccentColor {
    match s.trim().to_lowercase().as_str() {
        "blue" => AccentColor::Blue,
        "purple" => AccentColor::Purple,
        "pink" => AccentColor::Pink,
        "red" => AccentColor::Red,
        "orange" => AccentColor::Orange,
        "green" => AccentColor::Green,
        "yellow" => AccentColor::Yellow,
        "teal" => AccentColor::Teal,
        _ => AccentColor::default(),
    }
}

/// Persist the current theme mode through the Database global.
pub(crate) fn save_theme_mode(cx: &mut App) {
    let mode = cx.global::<AppState>().theme_mode;
    if let Some(db) = cx.try_global::<Database>() {
        let prefs = persistence::preferences::PreferencesStore::new(&db);
        let _ = prefs.set_theme_mode(mode.label());
    }
}

/// Persist the current accent colour through the Database global.
pub(crate) fn save_accent_color(cx: &mut App) {
    let color = cx.global::<AppState>().accent_color;
    if let Some(db) = cx.try_global::<Database>() {
        let prefs = persistence::preferences::PreferencesStore::new(&db);
        let _ = prefs.set_accent_color(color.label());
    }
}

/// Persist the current language id through the Database global.
pub(crate) fn save_language_id(cx: &mut App, id: &str) {
    if let Some(db) = cx.try_global::<Database>() {
        let prefs = persistence::preferences::PreferencesStore::new(&db);
        let _ = prefs.set_language_id(id);
    }
}

// ─── Navigation ─────────────────────────────────────────────

/// Helper: find the AppView entity in the window and update its current page.
fn navigate_to(cx: &mut App, page: Page) {
    let windows = cx.windows();
    for window in windows {
        let _ = window.update(cx, |root_view: AnyView, _window, cx| {
            if let Ok(app_view) = root_view.downcast::<AppView>() {
                app_view.update(cx, |app_view, cx| {
                    app_view.current_page = page;
                    app_view.tools_sub_page = None; // reset sub-page on main nav
                    cx.notify();
                });
            }
        });
    }
    cx.refresh_windows();
}
