/// Hopen GPUI — A multi-platform proxy client.
///
/// Entry point: creates the GPUI application, opens the main window,
/// and wires up global state and keyboard shortcuts.

mod app;
mod assets;
mod components;
mod i18n;
mod navigation;
mod theme;
mod views;

use gpui::*;

use app::AppView;
use assets::Assets;
use i18n::I18nManager;
use navigation::Page;
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
    Application::new()
        .with_assets(Assets::new())
        .run(|cx: &mut App| {
        // Initialize global state
        cx.set_global(AppState {
            core_running: false,
            system_proxy: false,
            tun_mode: false,
            theme_mode: load_theme_mode(),
            accent_color: load_accent_color(),
            outbound_mode: OutboundMode::default(),
            upload_speed: 0,
            download_speed: 0,
            upload_total: 0,
            download_total: 0,
            public_ip: None,
            lan_ip: None,
            isp: None,
        });

        // Initialize i18n — auto-detect system language
        let lang_id = i18n::detect_system_language_id();
        let lang_id = load_language_id().unwrap_or(lang_id);
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
                save_theme_mode(state.theme_mode);
            });
            cx.refresh_windows();
        });

        cx.on_action(|_: &ToggleCore, cx| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.core_running = !state.core_running;
            });
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
            |_, cx| {
                cx.new(|_cx| AppView {
                    current_page: Page::Dashboard,
                    tools_sub_page: None,
                })
            },
        )
        .unwrap();

        cx.activate(true);
    });
}

// ─── Config Persistence ──────────────────────────────────────

/// Returns the Hopen config directory.
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

/// Load the saved theme mode from disk, defaulting to Light.
pub(crate) fn load_theme_mode() -> ThemeMode {
    let path = config_dir().join("theme");
    std::fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_lowercase())
        .and_then(|s| match s.as_str() {
            "dark" => Some(ThemeMode::Dark),
            "light" => Some(ThemeMode::Light),
            "system" => Some(ThemeMode::System),
            _ => None,
        })
        .unwrap_or_default()
}

/// Persist the current theme mode to disk so it survives restarts.
pub(crate) fn save_theme_mode(mode: ThemeMode) {
    let dir = config_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("theme"), mode.label());
}

/// Load the saved language id from disk.
pub(crate) fn load_language_id() -> Option<String> {
    let path = config_dir().join("lang");
    std::fs::read_to_string(&path).ok().map(|s| s.trim().to_owned())
}

/// Persist the current language id so it survives restarts.
pub(crate) fn save_language_id(id: &str) {
    let dir = config_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("lang"), id);
}

/// Load the saved accent colour from disk, defaulting to Teal.
pub(crate) fn load_accent_color() -> AccentColor {
    let path = config_dir().join("accent");
    std::fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_lowercase())
        .and_then(|s| match s.as_str() {
            "teal" => Some(AccentColor::Teal),
            "blue" => Some(AccentColor::Blue),
            "purple" => Some(AccentColor::Purple),
            "pink" => Some(AccentColor::Pink),
            "red" => Some(AccentColor::Red),
            "orange" => Some(AccentColor::Orange),
            "green" => Some(AccentColor::Green),
            "yellow" => Some(AccentColor::Yellow),
            _ => None,
        })
        .unwrap_or_default()
}

/// Persist the current accent colour to disk.
pub(crate) fn save_accent_color(color: AccentColor) {
    let dir = config_dir();
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("accent"), color.label());
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
