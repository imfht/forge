use std::collections::HashMap;
use std::path::Path;

pub struct Translator {
    translations_dir: std::path::PathBuf,
    default_lang: String,
}

impl Translator {
    pub fn new(site_dir: &Path, default_lang: &str) -> Self {
        Self {
            translations_dir: site_dir.join("i18n"),
            default_lang: default_lang.to_string(),
        }
    }

    /// Load all translation files, returning language -> (key -> value)
    pub fn load_all(&self) -> HashMap<String, HashMap<String, String>> {
        let mut all = HashMap::new();

        if !self.translations_dir.exists() {
            return all;
        }

        if let Ok(entries) = std::fs::read_dir(&self.translations_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .is_some_and(|ext| ext == "yaml" || ext == "yml")
                {
                    if let Some(lang) = path.file_stem().and_then(|s| s.to_str()) {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if let Ok(map) =
                                serde_yaml_ng::from_str::<HashMap<String, String>>(&content)
                            {
                                all.insert(lang.to_string(), map);
                            }
                        }
                    }
                }
            }
        }

        all
    }

    pub fn default_lang(&self) -> &str {
        &self.default_lang
    }
}
