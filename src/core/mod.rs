/// Core engine integration module.
///
/// Provides:
/// - `action` — Action/ActionResult types for Go core RPC protocol.
/// - `event` — Event bus for unsolicited core events (logs, delays, …).
/// - `manager` — GPUI-aware wrapper (Global) for the core service.
/// - `service` — Process lifecycle management and high-level RPC API.
/// - `transport` — Wire-level frame protocol helpers.
pub mod action;
pub mod event;
pub mod manager;
pub mod service;
pub mod transport;
