use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::content::frontmatter::FrontMatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub date: DateTime<Utc>,
    pub draft: bool,
    pub description: String,
    pub summary: String,
    pub content_raw: String,
    pub content_html: String,
    pub toc: Vec<TocEntry>,
    pub word_count: usize,
    pub reading_time: usize,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub permalink: String,
    pub template: Option<String>,
    pub earlier: Option<crate::types::PostRef>,
    pub later: Option<crate::types::PostRef>,
    pub content_hash: String,
    pub source_path: String,
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    pub level: u32,
    pub id: String,
    pub title: String,
}

impl Post {
    pub fn from_frontmatter(
        fm: FrontMatter,
        content_html: String,
        content_raw: String,
        toc: Vec<TocEntry>,
        source_path: String,
        base_url: &str,
    ) -> Self {
        let slug = fm
            .slug
            .unwrap_or_else(|| slug::slugify(&fm.title));
        let word_count = content_raw
            .split_whitespace()
            .count();
        let reading_time = (word_count as f64 / 200.0).ceil() as usize;
        let reading_time = if reading_time == 0 { 1 } else { reading_time };
        let permalink = format!("{}/posts/{}/", base_url.trim_end_matches('/'), slug);

        let summary = fm.summary.clone().unwrap_or_else(|| {
            fm.description.clone().unwrap_or_default()
        });

        Self {
            title: fm.title,
            slug,
            date: fm.date.unwrap_or_else(Utc::now),
            draft: fm.draft,
            description: fm.description.unwrap_or_default(),
            summary,
            content_raw,
            content_html,
            toc,
            word_count,
            reading_time,
            categories: fm.categories,
            tags: fm.tags,
            permalink,
            template: fm.template,
            earlier: None,
            later: None,
            content_hash: String::new(),
            source_path,
            extra: fm.extra,
        }
    }
}
