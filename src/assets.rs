/// Asset source that loads files from the filesystem.
///
/// Implements `gpui::AssetSource` so that `svg().path("svg/xxx.svg")`
/// resolves to files under `assets/` relative to the crate root.
use std::{fs, io};
use std::path::PathBuf;

use anyhow::Result;
use gpui::{AssetSource, SharedString};

/// Filesystem-backed asset provider rooted at `CARGO_MANIFEST_DIR/assets`.
pub struct Assets {
    base: PathBuf,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"),
        }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        let full = self.base.join(path);
        match fs::read(&full) {
            Ok(data) => Ok(Some(std::borrow::Cow::Owned(data))),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let full = self.base.join(path);
        match fs::read_dir(&full) {
            Ok(entries) => Ok(entries
                .filter_map(|entry| {
                    entry
                        .ok()
                        .and_then(|e| e.file_name().into_string().ok())
                        .map(SharedString::from)
                })
                .collect()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(e.into()),
        }
    }
}
