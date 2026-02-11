use std::path::Path;

use crate::config;
use crate::error::ForgeResult;
use crate::pipeline::orchestrator::PipelineOrchestrator;

pub fn build_site(root: &Path, drafts: bool, force: bool) -> ForgeResult<()> {
    let mut config = config::load_config(root)?;
    if drafts {
        config.build.include_drafts = true;
    }

    let orchestrator = PipelineOrchestrator::new(root.to_path_buf(), config, force);
    orchestrator.run()?;

    Ok(())
}
