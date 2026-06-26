/// Configuration state — profile list, current active config.
use gpui::Global;
use serde::{Deserialize, Serialize};

// ── ConfigProfile ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProfile {
    /// Display name.
    pub name: String,
    /// File path or subscription URL.
    pub path: String,
    /// Profile type: local file or remote URL.
    pub profile_type: ProfileType,
    /// Last update timestamp (unix seconds).
    pub updated_at: Option<i64>,
    /// Subscription info (upload/download/total bytes).
    #[serde(skip)]
    #[allow(dead_code)]
    pub subscription_info: Option<SubscriptionInfo>,
    /// Fetched config YAML content (lazy-loaded from core).
    #[serde(skip)]
    pub config_content: Option<String>,
}

// ── ProfileType ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileType {
    File,
    Url,
}

// ── SubscriptionInfo ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionInfo {
    pub upload: i64,
    pub download: i64,
    pub total: i64,
    pub expire: i64,
}

// ── ConfigState ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigState {
    /// All known config profiles.
    pub profiles: Vec<ConfigProfile>,
    /// Currently active profile index.
    pub active_index: Option<usize>,
    /// Whether the "add subscription" panel is shown.
    pub show_add_panel: bool,
    /// URL being typed in the add panel.
    pub add_url: String,
}

impl Global for ConfigState {}

impl ConfigState {
    /// Get the currently active profile.
    #[allow(dead_code)]
    pub fn active_profile(&self) -> Option<&ConfigProfile> {
        self.active_index.and_then(|i| self.profiles.get(i))
    }

    /// Add a new profile.
    pub fn add_profile(&mut self, profile: ConfigProfile) {
        self.profiles.push(profile);
        if self.active_index.is_none() {
            self.active_index = Some(0);
        }
    }

    /// Remove a profile by index.
    pub fn remove_profile(&mut self, index: usize) {
        if index < self.profiles.len() {
            self.profiles.remove(index);
            if let Some(ref mut active) = self.active_index {
                if *active >= self.profiles.len() {
                    *active = self.profiles.len().saturating_sub(1);
                }
                if self.profiles.is_empty() {
                    *active = 0;
                    self.active_index = None;
                }
            }
        }
    }
}
