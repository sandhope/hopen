/// Theme system for the Hopen GPUI client.
///
/// Provides `Theme::dark()` and `Theme::light()` presets,
/// as well as the `ThemeMode` enum for runtime switching.
/// All colors are defined as u32 hex values for use with `gpui::rgb()`.
/// Spacing/sizing values are independent from color scheme.

use std::fmt;

// ─── Theme Mode ──────────────────────────────────────────────────

/// Which colour scheme is currently active.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ThemeMode {
    /// Light is the default — safer for users launching in bright environments.
    #[default]
    Light,
    Dark,
}

impl ThemeMode {
    /// Toggle between Dark and Light.
    pub fn toggle(self) -> Self {
        match self {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        }
    }

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
        }
    }
}

impl fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

// ─── Theme ───────────────────────────────────────────────────────

/// Complete colour palette for one theme variant.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    // Surface
    pub sidebar_bg: u32,
    pub content_bg: u32,
    pub surface: u32,
    pub surface_variant: u32,
    // Text
    pub text_primary: u32,
    pub text_secondary: u32,
    pub text_disabled: u32,
    // Accent
    pub accent: u32,
    pub accent_hover: u32,
    pub accent_muted: u32,
    // Status
    pub status_success: u32,
    pub status_warning: u32,
    pub status_error: u32,
    pub status_info: u32,
    // Border
    pub border: u32,
    pub border_light: u32,
    // Sidebar
    pub sidebar_active_bg: u32,
    pub sidebar_hover_bg: u32,
}

impl Theme {
    /// Zed-inspired dark theme.
    pub fn dark() -> Self {
        Self {
            sidebar_bg: 0x18181b,
            content_bg: 0x1e1e22,
            surface: 0x27272a,
            surface_variant: 0x3f3f46,
            text_primary: 0xfafafa,
            text_secondary: 0xa1a1aa,
            text_disabled: 0x52525b,
            accent: 0x2dd4bf,
            accent_hover: 0x14b8a6,
            accent_muted: 0x0d3d38,
            status_success: 0x4ade80,
            status_warning: 0xfbbf24,
            status_error: 0xf87171,
            status_info: 0x60a5fa,
            border: 0x3f3f46,
            border_light: 0x27272a,
            sidebar_active_bg: 0x27272a,
            sidebar_hover_bg: 0x1f1f23,
        }
    }

    /// Clean light theme.
    pub fn light() -> Self {
        Self {
            sidebar_bg: 0xf4f4f5,
            content_bg: 0xfafafa,
            surface: 0xffffff,
            surface_variant: 0xf4f4f5,
            text_primary: 0x18181b,
            text_secondary: 0x71717a,
            text_disabled: 0xa1a1aa,
            accent: 0x0d9488,
            accent_hover: 0x0f766e,
            accent_muted: 0xccfbf1,
            status_success: 0x16a34a,
            status_warning: 0xd97706,
            status_error: 0xdc2626,
            status_info: 0x2563eb,
            border: 0xd4d4d8,
            border_light: 0xe4e4e7,
            sidebar_active_bg: 0xe4e4e7,
            sidebar_hover_bg: 0xeeeeee,
        }
    }

    /// Return the palette matching `mode`.
    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }
}

// ─── Sizing & Spacing (theme-independent) ────────────────────────

pub const SIDEBAR_WIDTH: f32 = 200.0;
pub const NAV_ITEM_HEIGHT: f32 = 40.0;
pub const CARD_RADIUS: f32 = 8.0;
