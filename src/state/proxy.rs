/// Proxy state — holds all proxy group, node, and delay data.
///
/// Populated by RPC calls to the Go core (`getProxies`, `asyncTestDelay`, etc.).
use std::collections::HashMap;

use gpui::Global;
use serde::{Deserialize, Serialize};

use super::proxy_models::*;

// ── ProxyState ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyState {
    /// All proxy groups keyed by name.
    pub groups: HashMap<String, Group>,
    /// List of group names in display order.
    pub group_order: Vec<String>,
    /// Delay results keyed by proxy/group name.
    pub delays: HashMap<String, Option<i32>>,
    /// Currently selected proxy per group.
    pub selected: HashMap<String, String>,
    /// Search/filter text for proxy list.
    pub search_text: String,
    /// Sort mode for proxy list display.
    pub sort_mode: ProxySortMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProxySortMode {
    #[default]
    None,
    Delay,
    Name,
}

impl Default for ProxyState {
    fn default() -> Self {
        Self {
            groups: HashMap::new(),
            group_order: Vec::new(),
            delays: HashMap::new(),
            selected: HashMap::new(),
            search_text: String::new(),
            sort_mode: ProxySortMode::None,
        }
    }
}

impl Global for ProxyState {}

impl ProxyState {
    /// Update proxy data from a core response.
    pub fn update_from_core(&mut self, groups: HashMap<String, Group>) {
        self.groups = groups;
        self.group_order = self.groups.keys().cloned().collect();
    }

    /// Update delay for a specific proxy.
    pub fn update_delay(&mut self, delay: &Delay) {
        self.delays.insert(delay.name.clone(), delay.value);
    }

    /// Record a proxy selection.
    #[allow(dead_code)]
    pub fn select(&mut self, group_name: &str, proxy_name: &str) {
        self.selected
            .insert(group_name.to_string(), proxy_name.to_string());
        if let Some(group) = self.groups.get_mut(group_name) {
            group.now = Some(proxy_name.to_string());
        }
    }
}
