use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use rayon::prelude::*;

use crate::config::SiteConfig;
use crate::content::loader::ContentLoader;
use crate::error::ForgeResult;
use crate::i18n::translator::Translator;
use crate::output::{assets, feed, search_index, sitemap, writer};
use crate::pipeline::incremental::IncrementalCache;
use crate::render::context;
use crate::render::engine;
use crate::render::pagination::Paginator;
use crate::taxonomy::builder::build_taxonomies;
use crate::types::{PostRef, Site};

pub struct PipelineOrchestrator {
    site_dir: PathBuf,
    config: SiteConfig,
    force: bool,
}

impl PipelineOrchestrator {
    pub fn new(site_dir: PathBuf, config: SiteConfig, force: bool) -> Self {
        Self {
            site_dir,
            config,
            force,
        }
    }

    pub fn run(&self) -> ForgeResult<()> {
        let total_start = Instant::now();

        // Load incremental cache
        let mut cache = IncrementalCache::load(&self.site_dir, self.force)?;

        // Compute config hash
        let config_str = toml::to_string(&self.config).unwrap_or_default();
        let config_hash = blake3::hash(config_str.as_bytes()).to_hex().to_string();

        // Compute template hash
        let template_hash = self.hash_templates();

        let _full_rebuild =
            cache.config_changed(&config_hash) || cache.templates_changed(&template_hash);

        // ── Phase 1: LOAD ──
        let load_start = Instant::now();
        tracing::info!("Phase 1: Loading content...");
        let loader = ContentLoader::new(&self.config);
        let loaded = loader.load(&self.site_dir)?;
        let load_time = load_start.elapsed();

        // ── Phase 2: PARSE (already done during load with parallel potential) ──
        let parse_start = Instant::now();
        tracing::info!(
            "Phase 2: Processing {} posts, {} pages...",
            loaded.posts.len(),
            loaded.pages.len()
        );
        let mut posts = loaded.posts;
        let pages = loaded.pages;
        let parse_time = parse_start.elapsed();

        // ── Phase 3: ANALYZE ──
        let analyze_start = Instant::now();
        tracing::info!("Phase 3: Analyzing content...");

        // Set prev/next navigation
        for i in 0..posts.len() {
            if i > 0 {
                let later_ref = PostRef::from(&posts[i - 1]);
                posts[i].later = Some(later_ref);
            }
            if i + 1 < posts.len() {
                let earlier_ref = PostRef::from(&posts[i + 1]);
                posts[i].earlier = Some(earlier_ref);
            }
        }

        // Build taxonomies
        let taxonomies = build_taxonomies(&posts, &self.config.taxonomies, &self.config.base_url);

        // Build index paginator
        let post_refs: Vec<PostRef> = posts.iter().map(PostRef::from).collect();
        let index_paginator = Paginator::new(&post_refs, self.config.build.posts_per_page, 1, "");

        let site = Site {
            posts,
            pages,
            taxonomies,
            index_paginator,
        };

        let analyze_time = analyze_start.elapsed();

        // ── Phase 4: RENDER ──
        let render_start = Instant::now();
        tracing::info!("Phase 4: Rendering templates...");

        let mut tera = engine::create_tera_engine(&self.site_dir, &self.config.theme)?;

        // Load translations
        let translator = Translator::new(&self.site_dir, &self.config.language);
        let translations = translator.load_all();

        engine::register_functions(
            &mut tera,
            self.config.base_url.clone(),
            translations,
            self.config.language.clone(),
        );

        let output_dir = self.site_dir.join(&self.config.build.output_dir);
        fs::create_dir_all(&output_dir)?;

        // Render index pages (pagination)
        let all_post_refs: Vec<PostRef> = site.posts.iter().map(PostRef::from).collect();
        let index_paginators =
            Paginator::paginate_all(&all_post_refs, self.config.build.posts_per_page, "");

        for paginator in &index_paginators {
            let ctx = context::build_index_context(
                &site.posts,
                paginator,
                &self.config,
                &site.taxonomies,
            );
            let html = tera.render("index.html", &ctx)?;
            let path = if paginator.current_page == 1 {
                String::new()
            } else {
                format!("page/{}", paginator.current_page)
            };
            writer::write_page(&output_dir, &path, &html)?;
        }

        // Render posts in parallel
        let post_results: Vec<ForgeResult<()>> = site
            .posts
            .par_iter()
            .map(|post| {
                let ctx = context::build_post_context(post, &self.config, &site.taxonomies);
                let template = post.template.as_deref().unwrap_or("post.html");
                let html = tera.render(template, &ctx)?;
                writer::write_page(&output_dir, &format!("posts/{}", post.slug), &html)?;
                Ok(())
            })
            .collect();

        for result in post_results {
            result?;
        }

        // Render pages in parallel
        let page_results: Vec<ForgeResult<()>> = site
            .pages
            .par_iter()
            .map(|page| {
                let ctx = context::build_page_context(page, &self.config, &site.taxonomies);
                let template = page.template.as_deref().unwrap_or("page.html");
                let html = tera.render(template, &ctx)?;
                writer::write_page(&output_dir, &page.slug, &html)?;
                Ok(())
            })
            .collect();

        for result in page_results {
            result?;
        }

        // Render archive page
        let archive_ctx =
            context::build_archive_context(&site.posts, &self.config, &site.taxonomies);
        if let Ok(html) = tera.render("archive.html", &archive_ctx) {
            writer::write_page(&output_dir, "archive", &html)?;
        }

        // Render taxonomy pages
        for (tax_name, collection) in &site.taxonomies {
            // Taxonomy listing page
            let tax_ctx =
                context::build_taxonomy_list_context(collection, &self.config, &site.taxonomies);
            if let Ok(html) = tera.render("taxonomy.html", &tax_ctx) {
                writer::write_page(&output_dir, &collection.slug, &html)?;
            }

            // Individual taxonomy term pages
            for item in &collection.items {
                let item_paginator = Paginator::new(
                    &item.posts,
                    self.config.build.posts_per_page,
                    1,
                    &format!("{}/{}", collection.slug, item.slug),
                );
                let ctx = context::build_taxonomy_single_context(
                    tax_name,
                    item,
                    &item_paginator,
                    &self.config,
                    &site.taxonomies,
                );
                if let Ok(html) = tera.render("taxonomy_single.html", &ctx) {
                    writer::write_page(
                        &output_dir,
                        &format!("{}/{}", collection.slug, item.slug),
                        &html,
                    )?;
                }
            }
        }

        // Render 404 page
        let ctx_404 = context::build_404_context(&self.config, &site.taxonomies);
        if let Ok(html) = tera.render("404.html", &ctx_404) {
            writer::write_html(&output_dir, "404.html", &html)?;
        }

        let render_time = render_start.elapsed();

        // ── Phase 5: WRITE ──
        let write_start = Instant::now();
        tracing::info!("Phase 5: Writing output files...");

        // Copy static assets
        assets::copy_static_assets(&self.site_dir, &self.config.theme, &output_dir)?;

        // Generate RSS feed
        if self.config.build.generate_feed {
            let rss_xml = feed::generate_rss(&site.posts, &self.config)?;
            fs::write(output_dir.join("feed.xml"), &rss_xml)?;

            let atom_xml = feed::generate_atom(&site.posts, &self.config)?;
            fs::write(output_dir.join("atom.xml"), &atom_xml)?;
        }

        // Generate sitemap
        if self.config.build.generate_sitemap {
            let sitemap_xml =
                sitemap::generate_sitemap(&site.posts, &site.pages, &site.taxonomies, &self.config);
            fs::write(output_dir.join("sitemap.xml"), &sitemap_xml)?;
        }

        // Generate search index
        if self.config.build.generate_search_index {
            let search_json = search_index::generate_search_index(&site.posts);
            fs::write(output_dir.join("search_index.json"), &search_json)?;
        }

        let write_time = write_start.elapsed();

        // Update cache
        for post in &site.posts {
            cache.update_file(
                post.source_path.clone(),
                post.content_hash.clone(),
                output_dir.join(format!("posts/{}/index.html", post.slug)),
            );
        }
        cache.set_config_hash(config_hash);
        cache.set_template_hash(template_hash);
        cache.save()?;

        let total_time = total_start.elapsed();

        // Print build statistics
        println!("\n  Build complete!");
        println!("  Posts: {}, Pages: {}", site.posts.len(), site.pages.len());
        println!(
            "  Taxonomies: {}",
            site.taxonomies
                .values()
                .map(|t| format!("{} ({})", t.name, t.items.len()))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("  Output: {}", output_dir.display());
        println!("\n  Timing:");
        println!("    Load:    {:>8.2?}", load_time);
        println!("    Parse:   {:>8.2?}", parse_time);
        println!("    Analyze: {:>8.2?}", analyze_time);
        println!("    Render:  {:>8.2?}", render_time);
        println!("    Write:   {:>8.2?}", write_time);
        println!("    Total:   {:>8.2?}", total_time);

        Ok(())
    }

    fn hash_templates(&self) -> String {
        let mut hasher = blake3::Hasher::new();

        let theme_dir = self
            .site_dir
            .join("themes")
            .join(&self.config.theme)
            .join("templates");
        if theme_dir.exists() {
            Self::hash_directory(&mut hasher, &theme_dir);
        }

        let site_templates = self.site_dir.join("templates");
        if site_templates.exists() {
            Self::hash_directory(&mut hasher, &site_templates);
        }

        hasher.finalize().to_hex().to_string()
    }

    fn hash_directory(hasher: &mut blake3::Hasher, dir: &std::path::Path) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            paths.sort_by_key(|e| e.path());

            for entry in paths {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(content) = std::fs::read(&path) {
                        hasher.update(&content);
                    }
                } else if path.is_dir() {
                    Self::hash_directory(hasher, &path);
                }
            }
        }
    }
}
