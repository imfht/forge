use std::collections::HashMap;
use tera::Context;

use crate::config::SiteConfig;
use crate::content::page::Page;
use crate::content::post::Post;
use crate::render::pagination::Paginator;
use crate::taxonomy::TaxonomyCollection;

pub fn build_post_context(
    post: &Post,
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = base_context(config, taxonomies);
    ctx.insert("post", post);
    ctx.insert("page_title", &post.title);
    ctx
}

pub fn build_page_context(
    page: &Page,
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = base_context(config, taxonomies);
    ctx.insert("page", page);
    ctx.insert("page_title", &page.title);
    ctx
}

pub fn build_index_context(
    posts: &[Post],
    paginator: &Paginator,
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = base_context(config, taxonomies);
    ctx.insert("posts", posts);
    ctx.insert("paginator", paginator);
    ctx.insert("page_title", &config.title);
    ctx
}

pub fn build_taxonomy_list_context(
    taxonomy: &TaxonomyCollection,
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = base_context(config, taxonomies);
    ctx.insert("taxonomy", taxonomy);
    ctx.insert("page_title", &taxonomy.name);
    ctx
}

pub fn build_taxonomy_single_context(
    taxonomy_name: &str,
    item: &crate::taxonomy::TaxonomyItem,
    paginator: &Paginator,
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = base_context(config, taxonomies);
    ctx.insert("taxonomy_name", taxonomy_name);
    ctx.insert("term", item);
    ctx.insert("paginator", paginator);
    ctx.insert("page_title", &format!("{}: {}", taxonomy_name, item.name));
    ctx
}

pub fn build_archive_context(
    posts: &[Post],
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = base_context(config, taxonomies);
    ctx.insert("posts", posts);
    ctx.insert("page_title", "Archive");
    ctx
}

pub fn build_404_context(
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = base_context(config, taxonomies);
    ctx.insert("page_title", "Page Not Found");
    ctx
}

fn base_context(
    config: &SiteConfig,
    taxonomies: &HashMap<String, TaxonomyCollection>,
) -> Context {
    let mut ctx = Context::new();
    ctx.insert("config", config);
    ctx.insert("site_title", &config.title);
    ctx.insert("base_url", &config.base_url);
    ctx.insert("language", &config.language);
    ctx.insert("author", &config.author);
    ctx.insert("description", &config.description);
    ctx.insert("taxonomies", taxonomies);
    ctx.insert("extra", &config.extra);
    ctx
}
