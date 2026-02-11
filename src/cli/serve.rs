use std::path::Path;

use crate::config;
use crate::error::ForgeResult;
use crate::server::http;

pub async fn serve_site(root: &Path, port: u16, drafts: bool, open: bool) -> ForgeResult<()> {
    let mut config = config::load_config(root)?;
    if drafts {
        config.build.include_drafts = true;
    }

    http::start_server(root.to_path_buf(), config, port, open).await
}
