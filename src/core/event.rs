/// Core event bus — dispatches unsolicited events (logs, delays, requests, …)
/// from the Go core to registered listeners.
use std::sync::{Arc, Mutex, Weak};

use crate::core::action::{ActionResult, CoreEventType};

// ── Event Listener Trait ─────────────────────────────────────────

/// Implement this trait to receive core events.
pub trait CoreEventListener: Send + Sync {
    fn on_log(&self, _data: &serde_json::Value) {}
    fn on_delay(&self, _data: &serde_json::Value) {}
    fn on_request(&self, _data: &serde_json::Value) {}
    fn on_loaded(&self, _data: &serde_json::Value) {}
    fn on_crash(&self, _data: &serde_json::Value) {}
}

// ── Event Manager ────────────────────────────────────────────────

/// Thread-safe singleton that holds listener references and dispatches events.
pub struct CoreEventManager {
    listeners: Mutex<Vec<Weak<dyn CoreEventListener>>>,
}

impl CoreEventManager {
    pub fn new() -> Self {
        Self {
            listeners: Mutex::new(Vec::new()),
        }
    }

    /// Register a listener. The manager holds a weak reference so listeners
    /// are automatically cleaned up when they are dropped.
    pub fn add_listener(&self, listener: Arc<dyn CoreEventListener>) {
        if let Ok(mut guard) = self.listeners.lock() {
            guard.push(Arc::downgrade(&listener));
        }
    }

    /// Dispatch an `ActionResult` that has no request id (unsolicited event).
    pub fn dispatch_event(&self, result: &ActionResult) {
        // Determine the event type from method/data
        let ty = match result.method {
            crate::core::action::ActionMethod::Message => {
                // Try to parse as CoreEvent { type, data }
                if let Ok(event) = serde_json::from_value::<super::action::CoreEvent>(
                    result.data.clone(),
                ) {
                    match event.ty.as_str() {
                        "log" => CoreEventType::Log,
                        "delay" => CoreEventType::Delay,
                        "request" => CoreEventType::Request,
                        "loaded" => CoreEventType::Loaded,
                        "crash" => CoreEventType::Crash,
                        _ => return,
                    }
                } else {
                    return;
                }
            }
            _ => return,
        };

        let data = &result.data;
        if let Ok(guard) = self.listeners.lock() {
            // Prune dead listeners
            let mut listeners = guard.clone();
            listeners.retain(|w| w.strong_count() > 0);

            for weak in &listeners {
                if let Some(listener) = weak.upgrade() {
                    match ty {
                        CoreEventType::Log => listener.on_log(data),
                        CoreEventType::Delay => listener.on_delay(data),
                        CoreEventType::Request => listener.on_request(data),
                        CoreEventType::Loaded => listener.on_loaded(data),
                        CoreEventType::Crash => listener.on_crash(data),
                    }
                }
            }
        }
    }
}

impl Default for CoreEventManager {
    fn default() -> Self {
        Self::new()
    }
}
