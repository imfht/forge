pub mod types;

use std::path::Path;

use crate::error::{ForgeError, ForgeResult};
pub use types::SiteConfig;

/// Load site configuration from forge.toml
pub fn load_config(site_dir: &Path) -> ForgeResult<SiteConfig> {
    let config_path = site_dir.join("forge.toml");
    if !config_path.exists() {
        return Err(ForgeError::ConfigNotFound {
            path: config_path,
        });
    }

    let content = std::fs::read_to_string(&config_path)?;
    let config: SiteConfig = toml::from_str(&content)?;
    validate_config(&config)?;
    Ok(config)
}

fn validate_config(config: &SiteConfig) -> ForgeResult<()> {
    if config.title.is_empty() {
        return Err(ForgeError::Config("Site title cannot be empty".to_string()));
    }
    if config.build.posts_per_page == 0 {
        return Err(ForgeError::Config(
            "posts_per_page must be greater than 0".to_string(),
        ));
    }
    Ok(())
}
