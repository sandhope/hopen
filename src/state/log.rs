/// Log state — buffer of core log entries.
use gpui::Global;
use serde::{Deserialize, Serialize};

// ── LogLevel ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Silent,
}

// ── LogEntry ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp string.
    pub time: String,
    /// Log level.
    pub level: LogLevel,
    /// Log message payload.
    pub payload: String,
}

// ── LogState ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogState {
    /// Log entries buffer (most recent first).
    pub entries: Vec<LogEntry>,
    /// Currently active level filter.
    pub level_filter: LogLevel,
    /// Search text for filtering.
    pub search_text: String,
    /// Maximum number of entries to keep in memory.
    pub max_entries: usize,
}

impl Default for LogState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            level_filter: LogLevel::Info,
            search_text: String::new(),
            max_entries: 5000,
        }
    }
}

impl Global for LogState {}

impl LogState {
    /// Add a log entry, trimming to max_entries.
    pub fn push(&mut self, entry: LogEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            let excess = self.entries.len() - self.max_entries;
            self.entries.drain(0..excess);
        }
    }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Filtered view of entries by current level.
    pub fn filtered(&self) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level >= self.level_filter && e.level != LogLevel::Silent)
            .collect()
    }
}
