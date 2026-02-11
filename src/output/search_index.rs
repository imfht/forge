use serde::Serialize;

use crate::content::post::Post;

#[derive(Debug, Serialize)]
struct SearchEntry {
    title: String,
    url: String,
    body: String,
    description: String,
    categories: Vec<String>,
    tags: Vec<String>,
    date: String,
}

pub fn generate_search_index(posts: &[Post]) -> String {
    let entries: Vec<SearchEntry> = posts
        .iter()
        .map(|post| {
            // Strip HTML tags for plain text search body
            let body = strip_html(&post.content_html);
            // Limit body to first 500 words for index size
            let body: String = body
                .split_whitespace()
                .take(500)
                .collect::<Vec<_>>()
                .join(" ");

            SearchEntry {
                title: post.title.clone(),
                url: post.permalink.clone(),
                body,
                description: post.description.clone(),
                categories: post.categories.clone(),
                tags: post.tags.clone(),
                date: post.date.format("%Y-%m-%d").to_string(),
            }
        })
        .collect();

    serde_json::to_string_pretty(&entries).unwrap_or_else(|_| "[]".to_string())
}

fn strip_html(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
}
