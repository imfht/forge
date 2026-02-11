use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    /// Site title
    pub title: String,

    /// Base URL for the site (e.g., "https://example.com")
    #[serde(default = "default_base_url")]
    pub base_url: String,

    /// Primary language code
    #[serde(default = "default_language")]
    pub language: String,

    /// Site author
    #[serde(default)]
    pub author: String,

    /// Site description
    #[serde(default)]
    pub description: String,

    /// Theme name
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Build configuration
    #[serde(default)]
    pub build: BuildConfig,

    /// Taxonomy definitions
    #[serde(default = "default_taxonomies")]
    pub taxonomies: Vec<TaxonomyConfig>,

    /// i18n configuration
    #[serde(default)]
    pub i18n: I18nConfig,

    /// Arbitrary extra data available in templates
    #[serde(default)]
    pub extra: HashMap<String, toml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Output directory
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    /// Content directory
    #[serde(default = "default_content_dir")]
    pub content_dir: PathBuf,

    /// Templates directory (site-level overrides)
    #[serde(default = "default_templates_dir")]
    pub templates_dir: PathBuf,

    /// Static files directory
    #[serde(default = "default_static_dir")]
    pub static_dir: PathBuf,

    /// Number of posts per page
    #[serde(default = "default_posts_per_page")]
    pub posts_per_page: usize,

    /// Whether to include drafts in build
    #[serde(default)]
    pub include_drafts: bool,

    /// Whether to generate RSS feed
    #[serde(default = "default_true")]
    pub generate_feed: bool,

    /// Whether to generate sitemap
    #[serde(default = "default_true")]
    pub generate_sitemap: bool,

    /// Whether to generate search index
    #[serde(default = "default_true")]
    pub generate_search_index: bool,

    /// Whether to enable syntax highlighting
    #[serde(default = "default_true")]
    pub syntax_highlighting: bool,

    /// Syntax highlighting theme
    #[serde(default = "default_syntax_theme")]
    pub syntax_theme: String,

    /// Whether to generate TOC
    #[serde(default = "default_true")]
    pub generate_toc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyConfig {
    /// Taxonomy name (e.g., "categories", "tags")
    pub name: String,

    /// URL slug (defaults to name)
    #[serde(default)]
    pub slug: Option<String>,

    /// Whether to paginate taxonomy listings
    #[serde(default = "default_true")]
    pub paginate: bool,

    /// Feed for this taxonomy
    #[serde(default)]
    pub feed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct I18nConfig {
    /// Available languages
    #[serde(default)]
    pub languages: Vec<LanguageConfig>,

    /// Default language (falls back to site language)
    #[serde(default)]
    pub default_language: Option<String>,

    /// Translations directory
    #[serde(default = "default_i18n_dir")]
    pub translations_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub weight: i32,
}

fn default_base_url() -> String {
    "http://localhost:3000".to_string()
}

fn default_language() -> String {
    "en".to_string()
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("public")
}

fn default_content_dir() -> PathBuf {
    PathBuf::from("content")
}

fn default_templates_dir() -> PathBuf {
    PathBuf::from("templates")
}

fn default_static_dir() -> PathBuf {
    PathBuf::from("static")
}

fn default_i18n_dir() -> PathBuf {
    PathBuf::from("i18n")
}

fn default_posts_per_page() -> usize {
    10
}

fn default_true() -> bool {
    true
}

fn default_syntax_theme() -> String {
    "base16-ocean.dark".to_string()
}

fn default_taxonomies() -> Vec<TaxonomyConfig> {
    vec![
        TaxonomyConfig {
            name: "categories".to_string(),
            slug: None,
            paginate: true,
            feed: false,
        },
        TaxonomyConfig {
            name: "tags".to_string(),
            slug: None,
            paginate: true,
            feed: false,
        },
    ]
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "My Forge Site".to_string(),
            base_url: default_base_url(),
            language: default_language(),
            author: String::new(),
            description: String::new(),
            theme: default_theme(),
            build: BuildConfig::default(),
            taxonomies: default_taxonomies(),
            i18n: I18nConfig::default(),
            extra: HashMap::new(),
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_dir: default_output_dir(),
            content_dir: default_content_dir(),
            templates_dir: default_templates_dir(),
            static_dir: default_static_dir(),
            posts_per_page: default_posts_per_page(),
            include_drafts: false,
            generate_feed: true,
            generate_sitemap: true,
            generate_search_index: true,
            syntax_highlighting: true,
            syntax_theme: default_syntax_theme(),
            generate_toc: true,
        }
    }
}
