//! Data-access objects (DAOs) for each persistent entity.
//!
//! Each DAO works directly against a `Database` reference and
//! uses `serde_json` for (de)serialisation.
//!
//! Key design:
//! - All IDs are `i64`.
//! - JSON collections (`selected_map`, `unfold_set`, etc.) are stored
//!   as raw JSON strings to match FlClash's SQLite converters.
//! - LRU eviction for icon records is enforced on writes.

use std::collections::BTreeSet;

use chrono::Utc;

use super::db::Database;
use super::models::*;

// ── id counter helpers ──────────────────────────────────────────────
// We use a dedicated counter key inside each tree.  The key "~next_id"
// is unlikely to collide with real data because profile/script/rule ids
// are always non-negative.

fn next_id(tree: &sled::Tree) -> anyhow::Result<i64> {
    let raw = tree
        .update_and_fetch("~next_id", |old| {
            let cur = old
                .and_then(|b| b.try_into().ok().map(i64::from_be_bytes))
                .unwrap_or(0);
            let next = cur + 1;
            Some::<sled::IVec>(next.to_be_bytes().to_vec().into())
        })?
        .expect("update_and_fetch returned None");
    Ok(i64::from_be_bytes(raw.as_ref().try_into().unwrap()))
}

/// Generate a new UUIDv4 rule-link id.
pub fn new_link_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

// ════════════════════════════════════════════════════════════════════
//  ProfilesDao
// ════════════════════════════════════════════════════════════════════

pub struct ProfilesDao<'a> {
    db: &'a Database,
    tree: sled::Tree,
}

impl<'a> ProfilesDao<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db, tree: db.profiles() }
    }

    pub fn put(&self, mut profile: ProfileRecord) -> anyhow::Result<ProfileRecord> {
        if profile.id == 0 {
            profile.id = next_id(&self.tree)?;
        }
        let key = profile.id.to_be_bytes();
        let val = serde_json::to_vec(&profile)?;
        self.tree.insert(key, val)?;
        Ok(profile)
    }

    pub fn get(&self, id: i64) -> anyhow::Result<Option<ProfileRecord>> {
        let key = id.to_be_bytes();
        Ok(self
            .tree
            .get(key)?
            .and_then(|v| serde_json::from_slice(&v).ok()))
    }

    pub fn get_all(&self) -> anyhow::Result<Vec<ProfileRecord>> {
        let mut list: Vec<_> = self
            .tree
            .iter()
            .filter_map(|r| r.ok())
            .filter(|(k, _)| k.as_ref() != b"~next_id")
            .filter_map(|(_, v)| serde_json::from_slice::<ProfileRecord>(&v).ok())
            .collect();
        list.sort_by_key(|p| p.id);
        Ok(list)
    }

    /// Replace the entire set atomically: insert/update all entries
    /// in `list`, then delete any existing row whose id is not in the set.
    pub fn set_all(&self, list: &[ProfileRecord]) -> anyhow::Result<()> {
        let keep: BTreeSet<i64> = list.iter().map(|p| p.id).collect();

        // Remove stale keys
        for r in self.tree.iter().filter_map(|r| r.ok()) {
            if r.0.as_ref() == b"~next_id" {
                continue;
            }
            let id = i64::from_be_bytes(r.0.as_ref().try_into().unwrap_or_default());
            if !keep.contains(&id) {
                self.tree.remove(r.0)?;
            }
        }

        for p in list {
            let key = p.id.to_be_bytes();
            let val = serde_json::to_vec(p)?;
            self.tree.insert(key, val)?;
        }
        Ok(())
    }

    pub fn remove(&self, id: i64) -> anyhow::Result<()> {
        self.tree.remove(id.to_be_bytes())?;
        Ok(())
    }

    pub fn count(&self) -> anyhow::Result<usize> {
        Ok(self.tree.len().saturating_sub(1)) // minus ~next_id
    }
}

// ════════════════════════════════════════════════════════════════════
//  ScriptsDao
// ════════════════════════════════════════════════════════════════════

pub struct ScriptsDao {
    tree: sled::Tree,
}

impl ScriptsDao {
    pub fn new(db: &Database) -> Self {
        Self { tree: db.scripts() }
    }

    pub fn put(&self, mut s: ScriptRecord) -> anyhow::Result<ScriptRecord> {
        if s.id == 0 {
            s.id = next_id(&self.tree)?;
        }
        let key = s.id.to_be_bytes();
        let val = serde_json::to_vec(&s)?;
        self.tree.insert(key, val)?;
        Ok(s)
    }

    pub fn get(&self, id: i64) -> anyhow::Result<Option<ScriptRecord>> {
        let key = id.to_be_bytes();
        Ok(self.tree.get(key)?.and_then(|v| serde_json::from_slice(&v).ok()))
    }

    pub fn get_all(&self) -> anyhow::Result<Vec<ScriptRecord>> {
        let mut list: Vec<_> = self
            .tree
            .iter()
            .filter_map(|r| r.ok())
            .filter(|(k, _)| k.as_ref() != b"~next_id")
            .filter_map(|(_, v)| serde_json::from_slice::<ScriptRecord>(&v).ok())
            .collect();
        list.sort_by_key(|s| s.id);
        Ok(list)
    }

    pub fn remove(&self, id: i64) -> anyhow::Result<()> {
        self.tree.remove(id.to_be_bytes())?;
        Ok(())
    }
}

// ════════════════════════════════════════════════════════════════════
//  RulesDao
// ════════════════════════════════════════════════════════════════════

pub struct RulesDao {
    tree: sled::Tree,
}

impl RulesDao {
    pub fn new(db: &Database) -> Self {
        Self { tree: db.rules() }
    }

    pub fn put(&self, mut r: RuleRecord) -> anyhow::Result<RuleRecord> {
        if r.id == 0 {
            r.id = next_id(&self.tree)?;
        }
        let key = r.id.to_be_bytes();
        let val = serde_json::to_vec(&r)?;
        self.tree.insert(key, val)?;
        Ok(r)
    }

    pub fn get(&self, id: i64) -> anyhow::Result<Option<RuleRecord>> {
        let key = id.to_be_bytes();
        Ok(self.tree.get(key)?.and_then(|v| serde_json::from_slice(&v).ok()))
    }

    pub fn get_all(&self) -> anyhow::Result<Vec<RuleRecord>> {
        let mut list: Vec<_> = self
            .tree
            .iter()
            .filter_map(|r| r.ok())
            .filter(|(k, _)| k.as_ref() != b"~next_id")
            .filter_map(|(_, v)| serde_json::from_slice::<RuleRecord>(&v).ok())
            .collect();
        list.sort_by_key(|r| r.id);
        Ok(list)
    }

    pub fn remove(&self, id: i64) -> anyhow::Result<()> {
        self.tree.remove(id.to_be_bytes())?;
        Ok(())
    }
}

// ════════════════════════════════════════════════════════════════════
//  ProfileRulesDao
// ════════════════════════════════════════════════════════════════════

pub struct ProfileRulesDao {
    tree: sled::Tree,
}

impl ProfileRulesDao {
    pub fn new(db: &Database) -> Self {
        Self { tree: db.profile_rules() }
    }

    pub fn put(&self, link: ProfileRuleLink) -> anyhow::Result<()> {
        let key = link.id.as_bytes();
        let val = serde_json::to_vec(&link)?;
        self.tree.insert(key, val)?;
        Ok(())
    }

    /// Query links by profile_id and optional scene filter.
    pub fn query(
        &self,
        profile_id: i64,
        scene: Option<&str>,
    ) -> anyhow::Result<Vec<ProfileRuleLink>> {
        let mut links: Vec<_> = self
            .tree
            .iter()
            .filter_map(|r| r.ok())
            .filter_map(|(_, v)| serde_json::from_slice::<ProfileRuleLink>(&v).ok())
            .filter(|l| l.profile_id == Some(profile_id))
            .filter(|l| scene.map_or(true, |s| l.scene == s))
            .collect();
        links.sort_by(|a, b| a.order.cmp(&b.order));
        Ok(links)
    }

    pub fn remove_by_id(&self, link_id: &str) -> anyhow::Result<()> {
        self.tree.remove(link_id.as_bytes())?;
        Ok(())
    }

    pub fn remove_by_profile(&self, profile_id: i64) -> anyhow::Result<()> {
        let to_remove: Vec<Vec<u8>> = self
            .tree
            .iter()
            .filter_map(|r| r.ok())
            .filter_map(|(k, v)| {
                let link: ProfileRuleLink = serde_json::from_slice(&v).ok()?;
                if link.profile_id == Some(profile_id) {
                    Some(k.to_vec())
                } else {
                    None
                }
            })
            .collect();
        for k in to_remove {
            self.tree.remove(k)?;
        }
        Ok(())
    }
}

// ════════════════════════════════════════════════════════════════════
//  ProxyGroupsDao
// ════════════════════════════════════════════════════════════════════

pub struct ProxyGroupsDao {
    tree: sled::Tree,
}

impl ProxyGroupsDao {
    pub fn new(db: &Database) -> Self {
        Self { tree: db.proxy_groups() }
    }

    pub fn put(&self, mut g: ProxyGroupRecord) -> anyhow::Result<ProxyGroupRecord> {
        if g.id == 0 {
            g.id = next_id(&self.tree)?;
        }
        let key = g.id.to_be_bytes();
        let val = serde_json::to_vec(&g)?;
        self.tree.insert(key, val)?;
        Ok(g)
    }

    pub fn get(&self, id: i64) -> anyhow::Result<Option<ProxyGroupRecord>> {
        let key = id.to_be_bytes();
        Ok(self.tree.get(key)?.and_then(|v| serde_json::from_slice(&v).ok()))
    }

    pub fn query(&self, profile_id: i64) -> anyhow::Result<Vec<ProxyGroupRecord>> {
        let mut groups: Vec<_> = self
            .tree
            .iter()
            .filter_map(|r| r.ok())
            .filter(|(k, _)| k.as_ref() != b"~next_id")
            .filter_map(|(_, v)| serde_json::from_slice::<ProxyGroupRecord>(&v).ok())
            .filter(|g| g.profile_id == Some(profile_id))
            .collect();
        groups.sort_by(|a, b| a.order.cmp(&b.order));
        Ok(groups)
    }

    pub fn remove(&self, id: i64) -> anyhow::Result<()> {
        self.tree.remove(id.to_be_bytes())?;
        Ok(())
    }

    pub fn remove_by_profile(&self, profile_id: i64) -> anyhow::Result<()> {
        let to_remove: Vec<Vec<u8>> = self
            .tree
            .iter()
            .filter_map(|r| r.ok())
            .filter(|(k, _)| k.as_ref() != b"~next_id")
            .filter_map(|(k, v)| {
                let g: ProxyGroupRecord = serde_json::from_slice(&v).ok()?;
                if g.profile_id == Some(profile_id) {
                    Some(k.to_vec())
                } else {
                    None
                }
            })
            .collect();
        for k in to_remove {
            self.tree.remove(k)?;
        }
        Ok(())
    }
}

// ════════════════════════════════════════════════════════════════════
//  IconsDao
// ════════════════════════════════════════════════════════════════════

/// Maximum number of cached icon URL records (LRU eviction).
const ICON_CAPACITY: usize = 1000;

pub struct IconsDao {
    tree: sled::Tree,
}

impl IconsDao {
    pub fn new(db: &Database) -> Self {
        Self { tree: db.icons() }
    }

    /// Record an icon URL access, updating `last_accessed`.
    /// Evicts the oldest entry if capacity is exceeded.
    pub fn put_if_absent(&self, url: &str) -> anyhow::Result<()> {
        let now = Utc::now().timestamp_millis();
        // Always update the timestamp (upsert).
        self.tree.insert(url.as_bytes(), &now.to_be_bytes())?;
        // LRU eviction
        if self.tree.len() > ICON_CAPACITY {
            self.evict_oldest()?;
        }
        Ok(())
    }

    fn evict_oldest(&self) -> anyhow::Result<()> {
        // Find the entry with the smallest last_accessed.
        let mut oldest: Option<(Vec<u8>, i64)> = None;
        for r in self.tree.iter().filter_map(|r| r.ok()) {
            let ts = i64::from_be_bytes(r.1.as_ref().try_into().unwrap_or_default());
            match oldest {
                None => oldest = Some((r.0.to_vec(), ts)),
                Some((_, prev)) if ts < prev => oldest = Some((r.0.to_vec(), ts)),
                _ => {}
            }
        }
        if let Some((key, _)) = oldest {
            self.tree.remove(key)?;
        }
        Ok(())
    }
}

// ════════════════════════════════════════════════════════════════════
//  AppConfigDao
// ════════════════════════════════════════════════════════════════════

pub struct AppConfigDao {
    tree: sled::Tree,
}

impl AppConfigDao {
    pub fn new(db: &Database) -> Self {
        Self { tree: db.app_config() }
    }

    pub fn get_str(&self, key: &str) -> Option<String> {
        self.tree
            .get(key.as_bytes())
            .ok()
            .flatten()
            .map(|v| String::from_utf8_lossy(&v).into_owned())
    }

    pub fn set_str(&self, key: &str, val: &str) -> anyhow::Result<()> {
        self.tree.insert(key.as_bytes(), val.as_bytes())?;
        Ok(())
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.tree
            .get(key.as_bytes())
            .ok()
            .flatten()
            .and_then(|v| v.as_ref().try_into().ok().map(i32::from_be_bytes))
    }

    pub fn set_i32(&self, key: &str, val: i32) -> anyhow::Result<()> {
        self.tree.insert(key.as_bytes(), &val.to_be_bytes())?;
        Ok(())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.tree
            .get(key.as_bytes())
            .ok()
            .flatten()
            .map(|v| !v.is_empty() && v[0] != 0)
    }

    pub fn set_bool(&self, key: &str, val: bool) -> anyhow::Result<()> {
        self.tree
            .insert(key.as_bytes(), &[if val { 1u8 } else { 0u8 }])?;
        Ok(())
    }
}
