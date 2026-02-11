use std::path::Path;
use walkdir::WalkDir;

use crate::config::SiteConfig;
use crate::content::frontmatter::parse_front_matter;
use crate::content::markdown::MarkdownRenderer;
use crate::content::page::Page;
use crate::content::post::Post;
use crate::error::ForgeResult;

pub struct ContentLoader {
    renderer: MarkdownRenderer,
    base_url: String,
    include_drafts: bool,
}

#[derive(Debug)]
pub struct LoadedContent {
    pub posts: Vec<Post>,
    pub pages: Vec<Page>,
}

impl ContentLoader {
    pub fn new(config: &SiteConfig) -> Self {
        Self {
            renderer: MarkdownRenderer::new(
                &config.build.syntax_theme,
                config.build.syntax_highlighting,
                config.build.generate_toc,
            ),
            base_url: config.base_url.clone(),
            include_drafts: config.build.include_drafts,
        }
    }

    pub fn load(&self, site_dir: &Path) -> ForgeResult<LoadedContent> {
        let content_dir = site_dir.join(std::path::PathBuf::from("content"));
        let mut posts = Vec::new();
        let mut pages = Vec::new();

        if !content_dir.exists() {
            return Ok(LoadedContent { posts, pages });
        }

        // Load posts
        let posts_dir = content_dir.join("posts");
        if posts_dir.exists() {
            posts = self.load_posts(&posts_dir)?;
        }

        // Load pages
        let pages_dir = content_dir.join("pages");
        if pages_dir.exists() {
            pages = self.load_pages(&pages_dir)?;
        }

        Ok(LoadedContent { posts, pages })
    }

    fn load_posts(&self, posts_dir: &Path) -> ForgeResult<Vec<Post>> {
        let mut posts = Vec::new();

        for entry in WalkDir::new(posts_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            let path = entry.path();
            let content = std::fs::read_to_string(path)?;
            let source_path = path.to_string_lossy().to_string();

            let (fm, body) = parse_front_matter(&content, &source_path)?;

            if fm.draft && !self.include_drafts {
                continue;
            }

            let (html, toc) = self.renderer.render(&body);

            let mut post = Post::from_frontmatter(fm, html, body, toc, source_path, &self.base_url);

            // Compute content hash
            post.content_hash = blake3::hash(content.as_bytes()).to_hex().to_string();

            posts.push(post);
        }

        // Sort by date, newest first
        posts.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(posts)
    }

    fn load_pages(&self, pages_dir: &Path) -> ForgeResult<Vec<Page>> {
        let mut pages = Vec::new();

        for entry in WalkDir::new(pages_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            let path = entry.path();
            let content = std::fs::read_to_string(path)?;
            let source_path = path.to_string_lossy().to_string();

            let (fm, body) = parse_front_matter(&content, &source_path)?;

            let (html, toc) = self.renderer.render(&body);

            let mut page =
                Page::from_frontmatter(fm, html, &body, toc, source_path, &self.base_url);

            page.content_hash = blake3::hash(content.as_bytes()).to_hex().to_string();

            pages.push(page);
        }

        Ok(pages)
    }
}
