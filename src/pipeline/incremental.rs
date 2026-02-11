use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::ForgeResult;
use crate::types::BuildManifest;

const CACHE_DIR: &str = ".forge_cache";
const MANIFEST_FILE: &str = "manifest.json";

pub struct IncrementalCache {
    cache_dir: PathBuf,
    manifest: BuildManifest,
    force: bool,
}

impl IncrementalCache {
    pub fn load(site_dir: &Path, force: bool) -> ForgeResult<Self> {
        let cache_dir = site_dir.join(CACHE_DIR);
        let manifest_path = cache_dir.join(MANIFEST_FILE);

        let manifest = if !force && manifest_path.exists() {
            let data = fs::read_to_string(&manifest_path)?;
            serde_json::from_str(&data).unwrap_or_else(|_| BuildManifest::new())
        } else {
            BuildManifest::new()
        };

        Ok(Self {
            cache_dir,
            manifest,
            force,
        })
    }

    pub fn is_dirty(&self, path: &str, content_hash: &str) -> bool {
        if self.force {
            return true;
        }
        match self.manifest.file_hashes.get(path) {
            Some(record) => record.content_hash != content_hash,
            None => true,
        }
    }

    pub fn config_changed(&self, config_hash: &str) -> bool {
        if self.force {
            return true;
        }
        self.manifest.config_hash != config_hash
    }

    pub fn templates_changed(&self, template_hash: &str) -> bool {
        if self.force {
            return true;
        }
        self.manifest.template_hash != template_hash
    }

    pub fn update_file(&mut self, path: String, content_hash: String, output_path: PathBuf) {
        self.manifest.file_hashes.insert(
            path,
            crate::types::FileRecord {
                content_hash,
                output_path,
                template_deps: Vec::new(),
            },
        );
    }

    pub fn set_config_hash(&mut self, hash: String) {
        self.manifest.config_hash = hash;
    }

    pub fn set_template_hash(&mut self, hash: String) {
        self.manifest.template_hash = hash;
    }

    pub fn save(&mut self) -> ForgeResult<()> {
        self.manifest.last_build = chrono::Utc::now();
        fs::create_dir_all(&self.cache_dir)?;
        let data = serde_json::to_string_pretty(&self.manifest)?;
        fs::write(self.cache_dir.join(MANIFEST_FILE), data)?;
        Ok(())
    }

    pub fn file_hashes(&self) -> &HashMap<String, crate::types::FileRecord> {
        &self.manifest.file_hashes
    }
}
