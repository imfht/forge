use crate::content::post::Post;
use crate::error::ForgeResult;

/// Plugin lifecycle hooks
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;

    /// Priority (lower = runs first)
    fn priority(&self) -> i32 {
        100
    }

    /// Called after content is loaded, before rendering
    fn on_content_loaded(&self, _posts: &mut Vec<Post>) -> ForgeResult<()> {
        Ok(())
    }

    /// Called after HTML is generated for a post
    fn on_post_render(&self, _post: &mut Post) -> ForgeResult<()> {
        Ok(())
    }

    /// Called after the entire build is complete
    fn on_build_complete(&self, _output_dir: &std::path::Path) -> ForgeResult<()> {
        Ok(())
    }
}
