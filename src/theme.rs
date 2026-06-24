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
    /// Follow the system appearance (OS light / dark setting).
    System,
}

impl ThemeMode {
    /// Cycle Light → Dark → System → Light.
    pub fn toggle(self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::System,
            ThemeMode::System => ThemeMode::Light,
        }
    }

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
            ThemeMode::System => "System",
        }
    }
}

// ─── Accent Color ──────────────────────────────────────────────────

/// Predefined accent colour presets (matching FlClash's theme palette).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AccentColor {
    #[default]
    Teal,
    Blue,
    Purple,
    Pink,
    Red,
    Orange,
    Green,
    Yellow,
}

impl AccentColor {
    /// Number of presets.
    pub const COUNT: usize = 8;

    /// All presets in display order.
    pub const ALL: &'static [AccentColor] = &[
        AccentColor::Teal,
        AccentColor::Blue,
        AccentColor::Purple,
        AccentColor::Pink,
        AccentColor::Red,
        AccentColor::Orange,
        AccentColor::Green,
        AccentColor::Yellow,
    ];

    /// English label (universally understood for colour names).
    pub fn label(self) -> &'static str {
        match self {
            AccentColor::Teal => "Teal",
            AccentColor::Blue => "Blue",
            AccentColor::Purple => "Purple",
            AccentColor::Pink => "Pink",
            AccentColor::Red => "Red",
            AccentColor::Orange => "Orange",
            AccentColor::Green => "Green",
            AccentColor::Yellow => "Yellow",
        }
    }

    /// The primary hex colour for this accent (used in the swatch).
    pub fn swatch(self) -> u32 {
        match self {
            AccentColor::Teal => 0x2dd4bf,
            AccentColor::Blue => 0x60a5fa,
            AccentColor::Purple => 0xa78bfa,
            AccentColor::Pink => 0xf472b6,
            AccentColor::Red => 0xf87171,
            AccentColor::Orange => 0xfb923c,
            AccentColor::Green => 0x4ade80,
            AccentColor::Yellow => 0xfacc15,
        }
    }

    /// Returns (accent, accent_hover, accent_muted) for dark mode.
    pub fn dark_triple(self) -> (u32, u32, u32) {
        match self {
            AccentColor::Teal => (0x2dd4bf, 0x14b8a6, 0x0d3d38),
            AccentColor::Blue => (0x60a5fa, 0x3b82f6, 0x162447),
            AccentColor::Purple => (0xa78bfa, 0x8b5cf6, 0x1e1740),
            AccentColor::Pink => (0xf472b6, 0xec4899, 0x3a1528),
            AccentColor::Red => (0xf87171, 0xef4444, 0x3b1515),
            AccentColor::Orange => (0xfb923c, 0xf97316, 0x3a1a0a),
            AccentColor::Green => (0x4ade80, 0x22c55e, 0x0d2e18),
            AccentColor::Yellow => (0xfacc15, 0xeab308, 0x2e2408),
        }
    }

    /// Returns (accent, accent_hover, accent_muted) for light mode.
    pub fn light_triple(self) -> (u32, u32, u32) {
        match self {
            AccentColor::Teal => (0x0d9488, 0x0f766e, 0xccfbf1),
            AccentColor::Blue => (0x2563eb, 0x1d4ed8, 0xdbeafe),
            AccentColor::Purple => (0x7c3aed, 0x6d28d9, 0xede9fe),
            AccentColor::Pink => (0xdb2777, 0xbe185d, 0xfce7f3),
            AccentColor::Red => (0xdc2626, 0xb91c1c, 0xfee2e2),
            AccentColor::Orange => (0xea580c, 0xc2410c, 0xffedd5),
            AccentColor::Green => (0x16a34a, 0x15803d, 0xdcfce7),
            AccentColor::Yellow => (0xca8a04, 0xa16207, 0xfef9c3),
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
    // Titlebar
    pub titlebar_bg: u32,
    pub titlebar_border: u32,
    pub titlebar_text: u32,
    pub titlebar_icon: u32,
    pub titlebar_button_hover_bg: u32,
    pub titlebar_close_hover_bg: u32,
    pub titlebar_close_hover_text: u32,
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
            // Titlebar — dark
            titlebar_bg: 0x18181b,
            titlebar_border: 0x27272a,
            titlebar_text: 0xf4f4f5,
            titlebar_icon: 0xf4f4f5,
            titlebar_button_hover_bg: 0x3f3f46,
            titlebar_close_hover_bg: 0xef4444,
            titlebar_close_hover_text: 0xffffff,
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
            // Titlebar — light
            titlebar_bg: 0xffffff,
            titlebar_border: 0xd1d5db,
            titlebar_text: 0x1f2937,
            titlebar_icon: 0x18181b,
            titlebar_button_hover_bg: 0xe2e8f0,
            titlebar_close_hover_bg: 0xdc2626,
            titlebar_close_hover_text: 0xffffff,
        }
    }

    /// Return the palette matching `mode`, with accent colours overridden by `accent`.
    pub fn from_mode(mode: ThemeMode, accent: AccentColor) -> Self {
        let mut theme = match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
            ThemeMode::System => Self::light(), // fallback; resolved at call-site
        };
        let (a, ah, am) = match mode {
            ThemeMode::Dark => accent.dark_triple(),
            ThemeMode::Light => accent.light_triple(),
            ThemeMode::System => accent.light_triple(), // same fallback
        };
        theme.accent = a;
        theme.accent_hover = ah;
        theme.accent_muted = am;
        theme
    }
}

// ─── Sizing & Spacing (theme-independent) ────────────────────────

pub const SIDEBAR_WIDTH: f32 = 200.0;
pub const NAV_ITEM_HEIGHT: f32 = 40.0;
pub const CARD_RADIUS: f32 = 8.0;
