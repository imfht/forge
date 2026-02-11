use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::content::page::Page;
use crate::content::post::Post;
use crate::render::pagination::Paginator;
use crate::taxonomy::TaxonomyCollection;

/// Reference to a post for prev/next navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRef {
    pub title: String,
    pub slug: String,
    pub permalink: String,
}

impl From<&Post> for PostRef {
    fn from(post: &Post) -> Self {
        Self {
            title: post.title.clone(),
            slug: post.slug.clone(),
            permalink: post.permalink.clone(),
        }
    }
}

/// The fully assembled site data used during rendering
#[derive(Debug)]
pub struct Site {
    pub posts: Vec<Post>,
    pub pages: Vec<Page>,
    pub taxonomies: HashMap<String, TaxonomyCollection>,
    pub index_paginator: Paginator,
}

/// Build manifest for incremental builds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManifest {
    pub version: String,
    pub last_build: DateTime<Utc>,
    pub config_hash: String,
    pub template_hash: String,
    pub file_hashes: HashMap<String, FileRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRecord {
    pub content_hash: String,
    pub output_path: PathBuf,
    pub template_deps: Vec<String>,
}

impl BuildManifest {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            last_build: Utc::now(),
            config_hash: String::new(),
            template_hash: String::new(),
            file_hashes: HashMap::new(),
        }
    }
}

impl Default for BuildManifest {
    fn default() -> Self {
        Self::new()
    }
}
