use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{ForgeError, ForgeResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FrontMatter {
    pub title: String,

    #[serde(default)]
    pub date: Option<DateTime<Utc>>,

    #[serde(default)]
    pub draft: bool,

    #[serde(default)]
    pub slug: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub summary: Option<String>,

    #[serde(default)]
    pub categories: Vec<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub template: Option<String>,

    #[serde(default)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Split content at `---` delimiters and parse the YAML front matter.
/// Returns (FrontMatter, body_content).
pub fn parse_front_matter(content: &str, path: &str) -> ForgeResult<(FrontMatter, String)> {
    let content = content.trim_start_matches('\u{feff}'); // strip BOM

    if !content.starts_with("---") {
        return Err(ForgeError::FrontMatter {
            path: path.into(),
            message: "Missing opening --- delimiter".to_string(),
        });
    }

    let after_first = &content[3..];
    let end_pos = after_first.find("\n---").ok_or_else(|| ForgeError::FrontMatter {
        path: path.into(),
        message: "Missing closing --- delimiter".to_string(),
    })?;

    let yaml_str = &after_first[..end_pos];
    let body = &after_first[end_pos + 4..]; // skip past \n---

    let fm: FrontMatter = serde_yaml_ng::from_str(yaml_str).map_err(|e| {
        ForgeError::FrontMatter {
            path: path.into(),
            message: format!("YAML parse error: {e}"),
        }
    })?;

    Ok((fm, body.trim_start_matches('\n').to_string()))
}
