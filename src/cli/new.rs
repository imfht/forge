use std::fs;
use std::path::Path;

use crate::error::{ForgeError, ForgeResult};

pub fn create_new_site(name: &str) -> ForgeResult<()> {
    let site_dir = Path::new(name);

    if site_dir.exists() {
        return Err(ForgeError::PathExists(site_dir.to_path_buf()));
    }

    tracing::info!("Creating new site: {}", name);

    // Create directory structure
    fs::create_dir_all(site_dir.join("content/posts"))?;
    fs::create_dir_all(site_dir.join("content/pages"))?;
    fs::create_dir_all(site_dir.join("templates"))?;
    fs::create_dir_all(site_dir.join("static/css"))?;
    fs::create_dir_all(site_dir.join("static/js"))?;
    fs::create_dir_all(site_dir.join("static/images"))?;
    fs::create_dir_all(site_dir.join("i18n"))?;

    // Write forge.toml
    fs::write(
        site_dir.join("forge.toml"),
        format!(
            r#"title = "{name}"
base_url = "http://localhost:3000"
language = "en"
author = ""
description = "A site built with Forge"
theme = "default"

[build]
output_dir = "public"
content_dir = "content"
posts_per_page = 10
syntax_highlighting = true
syntax_theme = "base16-ocean.dark"
generate_toc = true
generate_feed = true
generate_sitemap = true
generate_search_index = true

[[taxonomies]]
name = "categories"
paginate = true

[[taxonomies]]
name = "tags"
paginate = true
"#
        ),
    )?;

    // Write sample post
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    fs::write(
        site_dir.join("content/posts/hello-world.md"),
        format!(
            r#"---
title: "Hello World"
date: {now}
categories:
  - General
tags:
  - introduction
  - forge
description: "Welcome to your new Forge site!"
---

# Hello World

Welcome to your new **Forge** site! This is your first post.

## Getting Started

Edit this file at `content/posts/hello-world.md` to get started.

### Code Highlighting

Forge supports syntax highlighting out of the box:

```rust
fn main() {{
    println!("Hello from Forge!");
}}
```

### Features

- Fast builds with incremental compilation
- Syntax highlighting
- Table of contents generation
- RSS and Atom feeds
- Sitemap generation
- Full-text search
- Categories and tags
- Pagination
- Internationalization
- Plugin system

Happy blogging!
"#
        ),
    )?;

    // Write sample page
    fs::write(
        site_dir.join("content/pages/about.md"),
        r#"---
title: "About"
slug: "about"
template: "page.html"
---

# About

This is a site built with [Forge](https://github.com/forge), a high-performance static blog generator written in Rust.
"#,
    )?;

    // Write default theme
    write_default_theme(site_dir)?;

    // Write sample i18n file
    fs::write(
        site_dir.join("i18n/en.yaml"),
        r#"nav_home: "Home"
nav_about: "About"
nav_archive: "Archive"
read_more: "Read more"
published_on: "Published on"
tagged_with: "Tagged with"
page_of: "Page %{current} of %{total}"
newer_posts: "Newer posts"
older_posts: "Older posts"
search_placeholder: "Search..."
no_results: "No results found"
toc_title: "Table of Contents"
"#,
    )?;

    println!("Created new Forge site: {name}");
    println!("  cd {name}");
    println!("  forge build");
    println!("  forge serve");

    Ok(())
}

fn write_default_theme(site_dir: &Path) -> ForgeResult<()> {
    let theme_dir = site_dir.join("themes/default");
    fs::create_dir_all(theme_dir.join("templates/partials"))?;
    fs::create_dir_all(theme_dir.join("static/css"))?;
    fs::create_dir_all(theme_dir.join("static/js"))?;

    // Use include_str! to embed all theme files at compile time
    let files: &[(&str, &str)] = &[
        ("theme.toml", include_str!("../../themes/default/theme.toml")),
        ("templates/base.html", include_str!("../../themes/default/templates/base.html")),
        ("templates/index.html", include_str!("../../themes/default/templates/index.html")),
        ("templates/post.html", include_str!("../../themes/default/templates/post.html")),
        ("templates/page.html", include_str!("../../themes/default/templates/page.html")),
        ("templates/archive.html", include_str!("../../themes/default/templates/archive.html")),
        ("templates/taxonomy.html", include_str!("../../themes/default/templates/taxonomy.html")),
        ("templates/taxonomy_single.html", include_str!("../../themes/default/templates/taxonomy_single.html")),
        ("templates/404.html", include_str!("../../themes/default/templates/404.html")),
        ("templates/partials/header.html", include_str!("../../themes/default/templates/partials/header.html")),
        ("templates/partials/footer.html", include_str!("../../themes/default/templates/partials/footer.html")),
        ("templates/partials/post_card.html", include_str!("../../themes/default/templates/partials/post_card.html")),
        ("templates/partials/pagination.html", include_str!("../../themes/default/templates/partials/pagination.html")),
        ("static/css/style.css", include_str!("../../themes/default/static/css/style.css")),
        ("static/css/syntax.css", include_str!("../../themes/default/static/css/syntax.css")),
        ("static/js/search.js", include_str!("../../themes/default/static/js/search.js")),
    ];

    for (path, content) in files {
        fs::write(theme_dir.join(path), content)?;
    }

    Ok(())
}

pub fn create_new_post(title: &str, draft: bool) -> ForgeResult<()> {
    let slug = slug::slugify(title);
    let content_dir = Path::new("content/posts");

    if !content_dir.exists() {
        return Err(ForgeError::Config(
            "content/posts directory not found. Are you in a Forge site?".to_string(),
        ));
    }

    let filename = format!("{}.md", slug);
    let path = content_dir.join(&filename);

    if path.exists() {
        return Err(ForgeError::PathExists(path));
    }

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let draft_line = if draft { "\ndraft: true" } else { "" };

    fs::write(
        &path,
        format!(
            r#"---
title: "{title}"
date: {now}{draft_line}
categories: []
tags: []
description: ""
---

Write your content here.
"#
        ),
    )?;

    println!("Created new post: {}", path.display());

    Ok(())
}
