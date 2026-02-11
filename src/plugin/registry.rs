use crate::content::post::Post;
use crate::error::ForgeResult;
use crate::plugin::hooks::Plugin;

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
        self.plugins.sort_by_key(|p| p.priority());
    }

    pub fn on_content_loaded(&self, posts: &mut Vec<Post>) -> ForgeResult<()> {
        for plugin in &self.plugins {
            plugin.on_content_loaded(posts)?;
        }
        Ok(())
    }

    pub fn on_post_render(&self, post: &mut Post) -> ForgeResult<()> {
        for plugin in &self.plugins {
            plugin.on_post_render(post)?;
        }
        Ok(())
    }

    pub fn on_build_complete(&self, output_dir: &std::path::Path) -> ForgeResult<()> {
        for plugin in &self.plugins {
            plugin.on_build_complete(output_dir)?;
        }
        Ok(())
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
