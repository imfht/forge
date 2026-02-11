use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::content::frontmatter::FrontMatter;
use crate::content::post::TocEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub title: String,
    pub slug: String,
    pub content_html: String,
    pub toc: Vec<TocEntry>,
    pub permalink: String,
    pub template: Option<String>,
    pub word_count: usize,
    pub reading_time: usize,
    pub source_path: String,
    pub content_hash: String,
    pub extra: HashMap<String, serde_json::Value>,
}

impl Page {
    pub fn from_frontmatter(
        fm: FrontMatter,
        content_html: String,
        content_raw: &str,
        toc: Vec<TocEntry>,
        source_path: String,
        base_url: &str,
    ) -> Self {
        let slug = fm.slug.unwrap_or_else(|| slug::slugify(&fm.title));
        let word_count = content_raw.split_whitespace().count();
        let reading_time = std::cmp::max(1, (word_count as f64 / 200.0).ceil() as usize);
        let permalink = format!("{}/{}/", base_url.trim_end_matches('/'), slug);

        Self {
            title: fm.title,
            slug,
            content_html,
            toc,
            permalink,
            template: fm.template,
            word_count,
            reading_time,
            source_path,
            content_hash: String::new(),
            extra: fm.extra,
        }
    }
}
