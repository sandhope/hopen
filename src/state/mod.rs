/// Application state types — separate GPUI Globals for different domains.
///
/// Each state module is independent and can be registered as a GPUI Global.
///
/// - `config` — Config profiles and active selection.
/// - `connection` — Active connections and request tracker logs.
/// - `log` — Core log buffer with level filtering.
/// - `proxy` — Proxy groups, nodes, delays, and current selection.
/// - `proxy_models` — Shared data types for proxy state.

pub mod config;
pub mod connection;
pub mod log;
pub mod proxy;
pub mod proxy_models;
