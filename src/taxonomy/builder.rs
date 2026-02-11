use std::collections::HashMap;

use crate::config::types::TaxonomyConfig;
use crate::content::post::Post;
use crate::taxonomy::{TaxonomyCollection, TaxonomyItem};
use crate::types::PostRef;

pub fn build_taxonomies(
    posts: &[Post],
    taxonomy_configs: &[TaxonomyConfig],
    base_url: &str,
) -> HashMap<String, TaxonomyCollection> {
    let mut result = HashMap::new();

    for tax_config in taxonomy_configs {
        let collection = build_taxonomy(posts, tax_config, base_url);
        result.insert(tax_config.name.clone(), collection);
    }

    result
}

fn build_taxonomy(
    posts: &[Post],
    config: &TaxonomyConfig,
    base_url: &str,
) -> TaxonomyCollection {
    let slug = config
        .slug
        .clone()
        .unwrap_or_else(|| slug::slugify(&config.name));

    // Group posts by taxonomy value
    let mut groups: HashMap<String, Vec<PostRef>> = HashMap::new();

    for post in posts {
        let values = match config.name.as_str() {
            "categories" => &post.categories,
            "tags" => &post.tags,
            _ => continue,
        };

        for value in values {
            groups
                .entry(value.clone())
                .or_default()
                .push(PostRef::from(post));
        }
    }

    // Build taxonomy items sorted by name
    let mut items: Vec<TaxonomyItem> = groups
        .into_iter()
        .map(|(name, posts)| {
            let item_slug = slug::slugify(&name);
            let post_count = posts.len();
            let permalink = format!(
                "{}/{}/{}/",
                base_url.trim_end_matches('/'),
                slug,
                item_slug
            );

            TaxonomyItem {
                name,
                slug: item_slug,
                post_count,
                posts,
                permalink,
            }
        })
        .collect();

    items.sort_by(|a, b| a.name.cmp(&b.name));

    TaxonomyCollection {
        name: config.name.clone(),
        slug,
        items,
    }
}
