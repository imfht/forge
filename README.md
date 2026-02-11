# Forge

A high-performance static blog generator written in Rust.

Forge transforms Markdown content into fast, modern static sites with built-in support for syntax highlighting, full-text search, RSS feeds, and more.

## Features

- **Fast builds** — Parallel rendering with [Rayon](https://github.com/rayon-rs/rayon) and incremental builds via content hashing
- **Markdown** — [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) with SIMD acceleration
- **Syntax highlighting** — Built-in code highlighting powered by [Syntect](https://github.com/trishume/syntect)
- **Full-text search** — Client-side search with a generated JSON index
- **RSS & Atom feeds** — Auto-generated feed files
- **XML sitemap** — SEO-ready sitemap generation
- **Taxonomies** — Categories, tags, and custom taxonomy support with pagination
- **Table of contents** — Auto-generated from headings
- **Theming** — Template-based theme system using [Tera](https://github.com/Keats/tera) (Jinja2-like syntax)
- **Dark mode** — Default theme supports `prefers-color-scheme`
- **i18n** — Multi-language support via YAML translation files
- **Dev server** — Live-reloading development server with file watching
- **Incremental builds** — Only rebuild changed content using BLAKE3 content hashing

## Installation

### From source

```bash
git clone https://github.com/imfht/forge.git
cd forge
cargo install --path .
```

### Build from source

```bash
cargo build --release
# Binary at ./target/release/forge
```

## Quick Start

```bash
# Create a new site
forge new my-blog

# Enter the site directory
cd my-blog

# Start the dev server with live reload
forge serve

# Build for production
forge build
```

Your site will be generated in the `public/` directory, ready to deploy to any static hosting service.

## Usage

```
forge <COMMAND>

Commands:
  new    Create a new site
  post   Create a new post
  build  Build the site
  serve  Start development server
  clean  Clean build artifacts
```

### Create a new post

```bash
forge post "My New Post"
forge post "Draft Post" --draft
```

### Build options

```bash
forge build                  # Standard build
forge build --drafts         # Include draft posts
forge build --force          # Force full rebuild (ignore cache)
```

### Dev server options

```bash
forge serve                  # Start on port 3000
forge serve --port 8080      # Custom port
forge serve --drafts         # Include drafts
forge serve --open           # Open browser automatically
```

## Site Structure

```
my-blog/
├── forge.toml               # Site configuration
├── content/
│   ├── posts/               # Blog posts (Markdown)
│   └── pages/               # Static pages (Markdown)
├── templates/               # Template overrides
├── static/                  # Static assets (copied as-is)
├── themes/
│   └── default/             # Theme files
│       ├── theme.toml
│       ├── templates/       # Tera HTML templates
│       └── static/          # Theme assets (CSS, JS)
├── i18n/                    # Translation files
│   └── en.yaml
└── public/                  # Generated output (git-ignored)
```

## Configuration

Site configuration lives in `forge.toml`:

```toml
title = "My Blog"
base_url = "https://example.com"
language = "en"
author = "Your Name"
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
```

## Post Front Matter

Posts use YAML front matter:

```markdown
---
title: "My Post Title"
date: 2025-01-01T12:00:00Z
categories:
  - Rust
tags:
  - static-site
  - tutorial
description: "A short description for SEO and feeds"
draft: true          # Optional, exclude from production builds
template: post.html  # Optional, override the default template
---

Your markdown content here.
```

## Theming

Forge uses [Tera](https://keats.github.io/tera/) templates. The default theme includes:

| Template | Purpose |
|---|---|
| `base.html` | Base layout with `<head>`, header, footer |
| `index.html` | Homepage with paginated post list |
| `post.html` | Single blog post |
| `page.html` | Static page |
| `archive.html` | Chronological archive |
| `taxonomy.html` | Taxonomy index (all categories/tags) |
| `taxonomy_single.html` | Single taxonomy term page |
| `404.html` | Error page |

Override any template by placing a file with the same name in your site's `templates/` directory.

### Template Functions

- `get_url(path)` — Generate absolute URL from a path
- `get_taxonomy_url(taxonomy, term)` — Generate taxonomy term URL
- `trans(key)` — Look up a translation string

### Template Filters

- `date_format(format)` — Format a date string
- `truncate_words(count)` — Truncate text to N words

## Architecture

Forge follows a 5-phase build pipeline:

```
Load → Parse → Analyze → Render → Write
```

1. **Load** — Read config, discover content files
2. **Parse** — Parse front matter and Markdown in parallel
3. **Analyze** — Build taxonomies, sort posts, compute navigation
4. **Render** — Generate HTML from templates (parallelized with Rayon)
5. **Write** — Output HTML, feeds, sitemap, search index, and copy assets

Incremental builds hash each content file with BLAKE3 and skip unchanged files using a build manifest cache.

## License

[MIT](LICENSE)
