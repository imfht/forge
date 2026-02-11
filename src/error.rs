use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ForgeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Config file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("YAML parse error: {0}")]
    YamlParse(#[from] serde_yaml_ng::Error),

    #[error("Template error: {0}")]
    Template(#[from] tera::Error),

    #[error("Front matter error in {path}: {message}")]
    FrontMatter { path: PathBuf, message: String },

    #[error("Content error: {0}")]
    Content(String),

    #[error("Build error: {0}")]
    Build(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Path already exists: {0}")]
    PathExists(PathBuf),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("RSS error: {0}")]
    Rss(#[from] rss::Error),

    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),
}

pub type ForgeResult<T> = Result<T, ForgeError>;
