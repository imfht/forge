use std::path::PathBuf;
use std::time::Duration;

use axum::Router;
use notify::Watcher;
use tokio::sync::mpsc;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use crate::config::SiteConfig;
use crate::error::ForgeResult;
use crate::pipeline::orchestrator::PipelineOrchestrator;

pub async fn start_server(
    site_dir: PathBuf,
    config: SiteConfig,
    port: u16,
    _open: bool,
) -> ForgeResult<()> {
    // Do an initial build
    let orchestrator = PipelineOrchestrator::new(site_dir.clone(), config.clone(), false);
    orchestrator.run()?;

    let output_dir = site_dir.join(&config.build.output_dir);

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let app = Router::new()
        .fallback_service(ServeDir::new(&output_dir).append_index_html_on_directories(true))
        .layer(livereload);

    // Set up file watcher
    let (tx, mut rx) = mpsc::channel::<()>(1);

    let watch_site_dir = site_dir.clone();

    // Spawn file watcher in a blocking thread
    let _watcher_handle = tokio::task::spawn_blocking(move || {
        let tx_clone = tx.clone();
        let mut watcher =
            notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
                if let Ok(event) = res {
                    match event.kind {
                        notify::EventKind::Modify(_)
                        | notify::EventKind::Create(_)
                        | notify::EventKind::Remove(_) => {
                            let _ = tx_clone.blocking_send(());
                        }
                        _ => {}
                    }
                }
            })
            .expect("Failed to create file watcher");

        // Watch content, templates, static, and config
        let dirs_to_watch = [
            watch_site_dir.join("content"),
            watch_site_dir.join("templates"),
            watch_site_dir.join("static"),
            watch_site_dir.join("themes"),
        ];

        for dir in &dirs_to_watch {
            if dir.exists() {
                let _ = watcher.watch(dir, notify::RecursiveMode::Recursive);
            }
        }

        // Watch forge.toml
        let config_file = watch_site_dir.join("forge.toml");
        if config_file.exists() {
            let _ = watcher.watch(&config_file, notify::RecursiveMode::NonRecursive);
        }

        // Keep watcher alive
        loop {
            std::thread::sleep(Duration::from_secs(3600));
        }
    });

    // Spawn rebuild handler
    let rebuild_site_dir = site_dir.clone();
    let rebuild_config = config.clone();
    tokio::spawn(async move {
        let mut debounce_interval = tokio::time::interval(Duration::from_millis(300));
        debounce_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        let mut pending = false;

        loop {
            tokio::select! {
                _ = rx.recv() => {
                    pending = true;
                }
                _ = debounce_interval.tick() => {
                    if pending {
                        pending = false;
                        println!("\n  File change detected, rebuilding...");
                        // Reload config in case it changed
                        let current_config = crate::config::load_config(&rebuild_site_dir)
                            .unwrap_or(rebuild_config.clone());
                        let orchestrator = PipelineOrchestrator::new(
                            rebuild_site_dir.clone(),
                            current_config,
                            false,
                        );
                        match orchestrator.run() {
                            Ok(_) => {
                                reloader.reload();
                                println!("  Live reload triggered.");
                            }
                            Err(e) => {
                                eprintln!("  Rebuild error: {e}");
                            }
                        }
                    }
                }
            }
        }
    });

    let addr = format!("0.0.0.0:{port}");
    println!("\n  Development server running at http://localhost:{port}/");
    println!("  Press Ctrl+C to stop.\n");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| crate::error::ForgeError::Server(e.to_string()))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| crate::error::ForgeError::Server(e.to_string()))?;

    Ok(())
}
