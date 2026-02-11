use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::error::ForgeResult;

/// Copy static assets from theme and site static directories to output.
pub fn copy_static_assets(site_dir: &Path, theme: &str, output_dir: &Path) -> ForgeResult<()> {
    // Copy theme static files first
    let theme_static = site_dir.join("themes").join(theme).join("static");
    if theme_static.exists() {
        copy_dir_recursive(&theme_static, output_dir)?;
    }

    // Copy site-level static files (override theme files)
    let site_static = site_dir.join("static");
    if site_static.exists() {
        copy_dir_recursive(&site_static, output_dir)?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> ForgeResult<()> {
    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let relative = path.strip_prefix(src).unwrap();
        let target = dest.join(relative);

        if path.is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &target)?;
        }
    }
    Ok(())
}
