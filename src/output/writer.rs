use std::fs;
use std::path::Path;

use crate::error::ForgeResult;

/// Write HTML content to the output directory, creating parent dirs as needed.
pub fn write_html(output_dir: &Path, relative_path: &str, content: &str) -> ForgeResult<()> {
    let path = output_dir.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, content)?;
    Ok(())
}

/// Write an index.html inside a directory path (for clean URLs).
pub fn write_page(output_dir: &Path, url_path: &str, content: &str) -> ForgeResult<()> {
    let clean_path = url_path.trim_matches('/');
    let dir_path = if clean_path.is_empty() {
        output_dir.to_path_buf()
    } else {
        output_dir.join(clean_path)
    };
    fs::create_dir_all(&dir_path)?;
    fs::write(dir_path.join("index.html"), content)?;
    Ok(())
}
