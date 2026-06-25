//! User-preference store backed by the `app_config` tree.
//!
//! Mirrors FlClash's `SharedPreferences` layer — simple key-value
//! persistence for theme, language, accent colour, and misc flags.

use super::dao::AppConfigDao;
use super::db::Database;

pub struct PreferencesStore {
    cfg: AppConfigDao,
}

// ── well-known keys ────────────────────────────────────────────────

const KEY_THEME: &str = "theme_mode";
const KEY_ACCENT: &str = "accent_color";
const KEY_LANG: &str = "language_id";
const KEY_CLOSE_TO_TRAY: &str = "close_to_tray";
const KEY_AUTO_LAUNCH: &str = "auto_launch";
const KEY_OVERRIDE_DNS: &str = "override_dns";
const KEY_VERSION: &str = "version";

impl PreferencesStore {
    pub fn new(db: &Database) -> Self {
        Self { cfg: AppConfigDao::new(db) }
    }

    // ── theme ────────────────────────────────────────────────────

    pub fn theme_mode(&self) -> Option<String> {
        self.cfg.get_str(KEY_THEME)
    }

    pub fn set_theme_mode(&self, mode: &str) -> anyhow::Result<()> {
        self.cfg.set_str(KEY_THEME, mode)
    }

    // ── accent ───────────────────────────────────────────────────

    pub fn accent_color(&self) -> Option<String> {
        self.cfg.get_str(KEY_ACCENT)
    }

    pub fn set_accent_color(&self, color: &str) -> anyhow::Result<()> {
        self.cfg.set_str(KEY_ACCENT, color)
    }

    // ── language ─────────────────────────────────────────────────

    pub fn language_id(&self) -> Option<String> {
        self.cfg.get_str(KEY_LANG)
    }

    pub fn set_language_id(&self, id: &str) -> anyhow::Result<()> {
        self.cfg.set_str(KEY_LANG, id)
    }

    // ── misc ─────────────────────────────────────────────────────

    pub fn close_to_tray(&self) -> Option<bool> {
        self.cfg.get_bool(KEY_CLOSE_TO_TRAY)
    }

    pub fn set_close_to_tray(&self, v: bool) -> anyhow::Result<()> {
        self.cfg.set_bool(KEY_CLOSE_TO_TRAY, v)
    }

    pub fn auto_launch(&self) -> Option<bool> {
        self.cfg.get_bool(KEY_AUTO_LAUNCH)
    }

    pub fn set_auto_launch(&self, v: bool) -> anyhow::Result<()> {
        self.cfg.set_bool(KEY_AUTO_LAUNCH, v)
    }

    pub fn override_dns(&self) -> bool {
        self.cfg.get_bool(KEY_OVERRIDE_DNS).unwrap_or(false)
    }

    pub fn set_override_dns(&self, v: bool) -> anyhow::Result<()> {
        self.cfg.set_bool(KEY_OVERRIDE_DNS, v)
    }

    pub fn version(&self) -> i32 {
        self.cfg.get_i32(KEY_VERSION).unwrap_or(1)
    }

    pub fn set_version(&self, v: i32) -> anyhow::Result<()> {
        self.cfg.set_i32(KEY_VERSION, v)
    }
}
