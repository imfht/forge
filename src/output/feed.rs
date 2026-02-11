use chrono::Utc;
use rss::{ChannelBuilder, ItemBuilder};

use crate::config::SiteConfig;
use crate::content::post::Post;
use crate::error::ForgeResult;

pub fn generate_rss(posts: &[Post], config: &SiteConfig) -> ForgeResult<String> {
    let items: Vec<rss::Item> = posts
        .iter()
        .take(20)
        .map(|post| {
            ItemBuilder::default()
                .title(Some(post.title.clone()))
                .link(Some(post.permalink.clone()))
                .description(Some(if post.description.is_empty() {
                    post.summary.clone()
                } else {
                    post.description.clone()
                }))
                .pub_date(Some(post.date.to_rfc2822()))
                .content(Some(post.content_html.clone()))
                .build()
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title(&config.title)
        .link(&config.base_url)
        .description(&config.description)
        .language(Some(config.language.clone()))
        .last_build_date(Some(Utc::now().to_rfc2822()))
        .items(items)
        .build();

    Ok(channel.to_string())
}

pub fn generate_atom(posts: &[Post], config: &SiteConfig) -> ForgeResult<String> {
    // Generate a simple Atom feed
    let base_url = config.base_url.trim_end_matches('/');
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    xml.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\">\n");
    xml.push_str(&format!("  <title>{}</title>\n", xml_escape(&config.title)));
    xml.push_str(&format!("  <link href=\"{base_url}/\" />\n"));
    xml.push_str(&format!(
        "  <link href=\"{base_url}/atom.xml\" rel=\"self\" />\n"
    ));
    xml.push_str(&format!("  <id>{base_url}/</id>\n"));
    xml.push_str(&format!(
        "  <updated>{}</updated>\n",
        Utc::now().to_rfc3339()
    ));

    if !config.author.is_empty() {
        xml.push_str("  <author>\n");
        xml.push_str(&format!(
            "    <name>{}</name>\n",
            xml_escape(&config.author)
        ));
        xml.push_str("  </author>\n");
    }

    for post in posts.iter().take(20) {
        xml.push_str("  <entry>\n");
        xml.push_str(&format!("    <title>{}</title>\n", xml_escape(&post.title)));
        xml.push_str(&format!("    <link href=\"{}\" />\n", post.permalink));
        xml.push_str(&format!("    <id>{}</id>\n", post.permalink));
        xml.push_str(&format!(
            "    <updated>{}</updated>\n",
            post.date.to_rfc3339()
        ));
        if !post.description.is_empty() {
            xml.push_str(&format!(
                "    <summary>{}</summary>\n",
                xml_escape(&post.description)
            ));
        }
        xml.push_str(&format!(
            "    <content type=\"html\">{}</content>\n",
            xml_escape(&post.content_html)
        ));
        xml.push_str("  </entry>\n");
    }

    xml.push_str("</feed>\n");

    Ok(xml)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
