//! Database models — mirror FlClash's SQLite tables.
//!
//! These are the persisted versions of domain objects, distinct from
//! the in-memory `state::*` types. They include DB-level fields like
//! `id`, `order`, and association links.

use serde::{Deserialize, Serialize};

// ── Profile ──────────────────────────────────────────────────────────

/// Persisted profile record (mirrors FlClash `profiles` table).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRecord {
    pub id: i64,
    pub label: String,
    pub url: String,
    pub last_update_date: Option<i64>, // unix millis
    pub auto_update: bool,
    /// milliseconds between auto-updates.
    pub auto_update_duration_millis: i64,
    pub overwrite_type: OverwriteType,
    pub script_id: Option<i64>,
    /// JSON: `Map<String, String>` — group → selected proxy name.
    pub selected_map: String,
    /// JSON: `Set<String>` — unfolded group names.
    pub unfold_set: String,
    /// JSON: `SubscriptionInfo?` — upload/download/total/expire.
    pub subscription_info: String,
    /// Current proxy-group name (runtime).
    pub current_group_name: Option<String>,
    /// Fractional-indexing order key.
    pub order: Option<String>,
}

// ── OverwriteType ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OverwriteType {
    #[default]
    Standard,
    Script,
    Custom,
}

// ── Script ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptRecord {
    pub id: i64,
    pub label: String,
    pub last_update_time: Option<i64>, // unix millis
}

// ── Rule ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleRecord {
    pub id: i64,
    pub rule_action: String,
    pub content: Option<String>,
    pub rule_target: Option<String>,
    pub rule_provider: Option<String>,
    pub sub_rule: Option<String>,
    pub no_resolve: bool,
    pub src: bool,
}

// ── ProfileRuleLink ──────────────────────────────────────────────────

/// Many-to-many link between profiles and rules with scene context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileRuleLink {
    pub id: String,
    pub profile_id: Option<i64>,
    pub rule_id: i64,
    /// One of: "added", "disabled", "custom", "global".
    pub scene: String,
    pub order: Option<String>,
}

// ── ProxyGroup (DB version) ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyGroupRecord {
    pub id: i64,
    pub profile_id: Option<i64>,
    pub name: String,
    pub group_type: String,
    /// JSON: `Vec<String>`
    pub proxies: Option<String>,
    /// JSON: `Vec<String>`
    pub use_providers: Option<String>,
    pub url: Option<String>,
    pub interval: Option<i32>,
    pub timeout: Option<i32>,
    pub max_failed_times: Option<i32>,
    pub lazy: Option<bool>,
    pub disable_udp: Option<bool>,
    pub filter: Option<String>,
    pub exclude_filter: Option<String>,
    pub exclude_type: Option<String>,
    pub expected_status: Option<String>,
    pub include_all: Option<bool>,
    pub include_all_proxies: Option<bool>,
    pub include_all_providers: Option<bool>,
    pub hidden: Option<bool>,
    pub icon: Option<String>,
    pub order: Option<String>,
}

// ── IconRecord ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconRecord {
    pub url: String,
    /// unix millis of last access.
    pub last_accessed: i64,
}

// ── AppConfig (persisted preferences snapshot) ───────────────────────

/// Full application configuration persisted as JSON.
/// Mirrors FlClash's `Config` model stored in SharedPreferences.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistedConfig {
    pub version: i32,
    pub current_profile_id: Option<i64>,
    pub theme_mode: String,       // "light" / "dark" / "system"
    pub accent_color: String,     // "teal" / "blue" / etc.
    pub language_id: String,      // "zh-CN" / "en" / etc.
    pub override_dns: bool,
    pub hotkeys_json: String,     // JSON array of hotkey bindings
    /// JSON-serialised PatchClashConfig.
    pub patch_config_json: String,
}
