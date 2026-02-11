use std::fs;
use std::path::Path;

use crate::error::ForgeResult;

pub fn clean_site(root: &Path) -> ForgeResult<()> {
    let output_dir = root.join("public");
    let cache_dir = root.join(".forge_cache");
    let mut cleaned = false;

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
        println!("Removed {}", output_dir.display());
        cleaned = true;
    }

    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir)?;
        println!("Removed {}", cache_dir.display());
        cleaned = true;
    }

    if cleaned {
        println!("Clean complete.");
    } else {
        println!("Nothing to clean.");
    }

    Ok(())
}
