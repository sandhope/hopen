/// Proxy-related data models — mirror FlClash `lib/models/clash_config.dart` and `common.dart`.
use serde::{Deserialize, Serialize};

// ── Proxy ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Proxy {
    pub name: String,
    #[serde(rename = "type")]
    pub proxy_type: String,
    pub now: Option<String>,
}

// ── GroupType ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum GroupType {
    #[default]
    Select,
    #[serde(rename = "url-test")]
    UrlTest,
    Fallback,
    #[serde(rename = "load-balance")]
    LoadBalance,
    Relay,
}

// ── ProxyGroup (raw from core) ────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProxyGroup {
    pub name: String,
    #[serde(rename = "type")]
    pub group_type: GroupType,
    #[serde(default)]
    pub proxies: Option<Vec<String>>,
    #[serde(rename = "use", default)]
    pub use_providers: Option<Vec<String>>,
    #[serde(default)]
    pub interval: Option<i32>,
    #[serde(default)]
    pub lazy: Option<bool>,
    #[serde(rename = "disable-udp", default)]
    pub disable_udp: Option<bool>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub timeout: Option<i32>,
    #[serde(rename = "max-failed-times", default)]
    pub max_failed_times: Option<i32>,
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(rename = "exclude-filter", default)]
    pub exclude_filter: Option<String>,
    #[serde(rename = "exclude-type", default)]
    pub exclude_type: Option<String>,
    #[serde(rename = "expected-status", default)]
    pub expected_status: Option<String>,
    #[serde(rename = "include-all", default)]
    pub include_all: Option<bool>,
    #[serde(rename = "include-all-proxies", default)]
    pub include_all_proxies: Option<bool>,
    #[serde(rename = "include-all-providers", default)]
    pub include_all_providers: Option<bool>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub icon: Option<String>,
}

// ── Group (runtime state) ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Group {
    #[serde(rename = "type")]
    pub group_type: GroupType,
    #[serde(default)]
    pub all: Vec<Proxy>,
    pub now: Option<String>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub test_url: Option<String>,
    #[serde(default)]
    pub icon: String,
    pub name: String,
}

// ── Delay ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delay {
    pub name: String,
    pub url: String,
    pub value: Option<i32>,
}

// ── Now ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Now {
    pub name: String,
    pub value: String,
}
