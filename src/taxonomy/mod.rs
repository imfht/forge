pub mod builder;

use serde::{Deserialize, Serialize};

use crate::types::PostRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyCollection {
    pub name: String,
    pub slug: String,
    pub items: Vec<TaxonomyItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyItem {
    pub name: String,
    pub slug: String,
    pub post_count: usize,
    pub posts: Vec<PostRef>,
    pub permalink: String,
}
