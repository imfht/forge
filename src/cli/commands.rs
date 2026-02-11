use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "forge",
    version,
    about = "A high-performance static blog generator"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new site
    New {
        /// Site name / directory
        name: String,
    },

    /// Create a new post
    Post {
        /// Post title
        title: String,

        /// Include as draft
        #[arg(short, long)]
        draft: bool,
    },

    /// Build the site
    Build {
        /// Site root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Include draft posts
        #[arg(short, long)]
        drafts: bool,

        /// Force full rebuild (ignore cache)
        #[arg(short, long)]
        force: bool,
    },

    /// Start development server
    Serve {
        /// Site root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,

        /// Port number
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Include draft posts
        #[arg(short, long)]
        drafts: bool,

        /// Open browser automatically
        #[arg(short, long)]
        open: bool,
    },

    /// Clean build artifacts
    Clean {
        /// Site root directory
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
}
