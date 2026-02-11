use clap::Parser;
use tracing_subscriber::EnvFilter;

use forge::cli::commands::{Cli, Commands};
use forge::cli::{build, clean, new, serve};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name } => new::create_new_site(&name),
        Commands::Post { title, draft } => new::create_new_post(&title, draft),
        Commands::Build {
            root,
            drafts,
            force,
        } => build::build_site(&root, drafts, force),
        Commands::Serve {
            root,
            port,
            drafts,
            open,
        } => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            rt.block_on(serve::serve_site(&root, port, drafts, open))
        }
        Commands::Clean { root } => clean::clean_site(&root),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
