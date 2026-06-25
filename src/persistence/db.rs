//! Embedded database backed by sled.
//!
//! Structure:
//! ```text
//! <data_dir>/hopen.db/
//!   ├── profiles/       — key=i64(id) → JSON(ProfileRecord)
//!   ├── scripts/        — key=i64(id) → JSON(ScriptRecord)
//!   ├── rules/          — key=i64(id) → JSON(RuleRecord)
//!   ├── profile_rules/  — key=String(uuid) → JSON(ProfileRuleLink)
//!   ├── proxy_groups/   — key=i64(id) → JSON(ProxyGroupRecord)
//!   ├── icons/          — key=String(url) → i64(last_accessed)
//!   └── app_config/     — key=str → JSON / primitives
//! ```
//!
//! Additionally, the filesystem stores:
//! ```text
//! <data_dir>/
//!   ├── profiles/<id>.yaml   — raw profile YAML content
//!   ├── scripts/<id>.js      — JS override script text
//!   └── config.yaml           — generated clash runtime config
//! ```

use std::path::PathBuf;
use std::sync::Arc;

use gpui::Global;
use sled::Db;

// ── Tree name constants ─────────────────────────────────────────────

pub(crate) const TREE_PROFILES: &str = "profiles";
pub(crate) const TREE_SCRIPTS: &str = "scripts";
pub(crate) const TREE_RULES: &str = "rules";
pub(crate) const TREE_PROFILE_RULES: &str = "profile_rules";
pub(crate) const TREE_PROXY_GROUPS: &str = "proxy_groups";
pub(crate) const TREE_ICONS: &str = "icons";
pub(crate) const TREE_APP_CONFIG: &str = "app_config";

// ── Database ────────────────────────────────────────────────────────

/// Holds open sled trees and provides convenience accessors.
#[derive(Clone)]
pub struct Database {
    db: Arc<Db>,
    pub data_dir: PathBuf,
}

impl Database {
    /// Open (or create) the sled database at `data_dir/hopen.db`.
    pub fn open(data_dir: PathBuf) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("hopen.db");
        let db = sled::open(&db_path)?;
        Ok(Self {
            db: Arc::new(db),
            data_dir,
        })
    }

    // ── convenience tree accessors ──────────────────────────────────

    pub fn profiles(&self) -> sled::Tree {
        self.db.open_tree(TREE_PROFILES).unwrap()
    }

    pub fn scripts(&self) -> sled::Tree {
        self.db.open_tree(TREE_SCRIPTS).unwrap()
    }

    pub fn rules(&self) -> sled::Tree {
        self.db.open_tree(TREE_RULES).unwrap()
    }

    pub fn profile_rules(&self) -> sled::Tree {
        self.db.open_tree(TREE_PROFILE_RULES).unwrap()
    }

    pub fn proxy_groups(&self) -> sled::Tree {
        self.db.open_tree(TREE_PROXY_GROUPS).unwrap()
    }

    pub fn icons(&self) -> sled::Tree {
        self.db.open_tree(TREE_ICONS).unwrap()
    }

    pub fn app_config(&self) -> sled::Tree {
        self.db.open_tree(TREE_APP_CONFIG).unwrap()
    }

    // ── path helpers ────────────────────────────────────────────────

    /// `<data_dir>/profiles/<id>.yaml`
    pub fn profile_yaml_path(&self, id: i64) -> PathBuf {
        let mut p = self.data_dir.join("profiles");
        p.push(format!("{id}.yaml"));
        p
    }

    /// `<data_dir>/profiles/` directory
    pub fn profiles_dir(&self) -> PathBuf {
        self.data_dir.join("profiles")
    }

    /// `<data_dir>/scripts/<id>.js`
    pub fn script_path(&self, id: i64) -> PathBuf {
        let mut p = self.data_dir.join("scripts");
        p.push(format!("{id}.js"));
        p
    }

    /// `<data_dir>/scripts/` directory
    pub fn scripts_dir(&self) -> PathBuf {
        self.data_dir.join("scripts")
    }

    /// `<data_dir>/config.yaml` — the clash runtime config.
    pub fn clash_config_path(&self) -> PathBuf {
        self.data_dir.join("config.yaml")
    }

    /// `<data_dir>/backup.zip`
    pub fn backup_path(&self) -> PathBuf {
        self.data_dir.join("backup.zip")
    }

    /// `<data_dir>/restore/`
    pub fn restore_dir(&self) -> PathBuf {
        self.data_dir.join("restore")
    }

    /// Flush all pending writes.
    pub fn flush(&self) -> anyhow::Result<()> {
        self.db.flush()?;
        Ok(())
    }
}

// Allow Database to be stored as a GPUI Global.
impl Global for Database {}
