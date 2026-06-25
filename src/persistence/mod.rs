//! Data persistence layer.
//!
//! Replaces FlClash's three-tier storage:
//! | FlClash               | Hopen equivalent                     |
//! |-----------------------|--------------------------------------|
//! | SharedPreferences     | `app_config` sled tree               |
//! | SQLite (Drift)        | Six sled trees (profiles/rules/...)  |
//! | File system           | Same (`profiles/<id>.yaml`, etc.)    |
//!
//! Database: `<data_dir>/hopen.db/` (sled embedded)
//! Files:    `<data_dir>/profiles/`, `<data_dir>/scripts/`, etc.

pub mod dao;
pub mod db;
pub mod models;
pub mod preferences;

pub use db::Database;
