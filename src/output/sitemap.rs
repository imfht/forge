use crate::config::SiteConfig;
use crate::content::page::Page;
use crate::content::post::Post;
use crate::taxonomy::TaxonomyCollection;
use std::collections::HashMap;

pub fn generate_sitemap(
    posts: &[Post],
    pages: &[Page],
    taxonomies: &HashMap<String, TaxonomyCollection>,
    config: &SiteConfig,
) -> String {
    let base_url = config.base_url.trim_end_matches('/');
    let mut xml = String::new();

    xml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    // Homepage
    xml.push_str(&format!(
        "  <url>\n    <loc>{base_url}/</loc>\n    <priority>1.0</priority>\n  </url>\n"
    ));

    // Posts
    for post in posts {
        xml.push_str(&format!(
            "  <url>\n    <loc>{}</loc>\n    <lastmod>{}</lastmod>\n    <priority>0.8</priority>\n  </url>\n",
            post.permalink,
            post.date.format("%Y-%m-%d")
        ));
    }

    // Pages
    for page in pages {
        xml.push_str(&format!(
            "  <url>\n    <loc>{}</loc>\n    <priority>0.6</priority>\n  </url>\n",
            page.permalink
        ));
    }

    // Taxonomy pages
    for collection in taxonomies.values() {
        xml.push_str(&format!(
            "  <url>\n    <loc>{base_url}/{}/</loc>\n    <priority>0.5</priority>\n  </url>\n",
            collection.slug
        ));
        for item in &collection.items {
            xml.push_str(&format!(
                "  <url>\n    <loc>{}</loc>\n    <priority>0.4</priority>\n  </url>\n",
                item.permalink
            ));
        }
    }

    xml.push_str("</urlset>\n");
    xml
}
