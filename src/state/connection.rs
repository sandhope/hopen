/// Connection state — active connections and request tracker logs.
use gpui::Global;
use serde::{Deserialize, Serialize};

// ── Connection ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Unique connection id.
    pub id: String,
    /// Source address.
    pub source: String,
    /// Destination address.
    pub destination: String,
    /// Host / domain name.
    pub host: Option<String>,
    /// Proxy chain (e.g. "DIRECT" or proxy name).
    pub chain: Vec<String>,
    /// Matching rule used.
    pub rule: Option<String>,
    /// Current upload speed (bytes/s).
    pub upload: u64,
    /// Current download speed (bytes/s).
    pub download: u64,
    /// Total upload bytes.
    pub upload_total: u64,
    /// Total download bytes.
    pub download_total: u64,
    /// Connection start time (unix seconds).
    pub start_time: i64,
    /// Network protocol (tcp/udp).
    pub network: String,
    /// Connection type.
    pub conn_type: String,
}

// ── Request Log ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    /// Timestamp.
    pub time: String,
    /// Target domain / host.
    pub host: String,
    /// Proxy used.
    pub proxy: String,
    /// Delay in milliseconds.
    pub delay: i32,
    /// Matching rule.
    pub rule: String,
    /// Extra info.
    pub info: Option<String>,
}

// ── ConnectionState ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionState {
    /// Active connections.
    pub connections: Vec<Connection>,
    /// Request tracker logs.
    pub request_logs: Vec<RequestLog>,
    /// Selected connection index for detail view.
    pub selected_index: Option<usize>,
    /// Selected request log index.
    pub request_selected_index: Option<usize>,
    /// Search text for filtering.
    pub search_text: String,
    /// Request log search text.
    pub request_search_text: String,
}

impl Global for ConnectionState {}
