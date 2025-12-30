use crate::app::App;
use crate::config::AppConfig;
use crate::ffmpeg::runner::Runner;
use crate::model::{BatchOperation, Intent};
use crate::util::system::ensure_ffmpeg_exists;
use anyhow::Result;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration as StdDuration;

pub fn handle_watch_folder(
    config: &AppConfig,
    _runner: &dyn Runner,
    folder: impl AsRef<Path>,
    operation: BatchOperation,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let folder = folder.as_ref();

    if !folder.exists() {
        anyhow::bail!("Folder does not exist: {}", folder.display());
    }

    if !folder.is_dir() {
        anyhow::bail!("Path is not a directory: {}", folder.display());
    }

    eprintln!("Watching folder: {}", folder.display());
    eprintln!("Operation: {:?}", operation);
    eprintln!("Press Ctrl+C to stop...\n");

    // Create app to execute intents
    let app = App::new(config.clone());

    // Create channel for file events
    let (tx, rx) = mpsc::channel::<PathBuf>();

    // Create watcher
    let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(event) => {
                if let EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        if path.is_file() {
                            // Check if it's a video/audio file
                            if is_media_file(&path) {
                                if let Err(e) = tx.send(path) {
                                    eprintln!("Error sending file path: {}", e);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {}", e),
        }
    })?;

    // Watch the folder
    watcher.watch(folder, RecursiveMode::NonRecursive)?;

    // Track processed files to avoid duplicates
    let mut processed_files = std::collections::HashSet::new();

    // Process files as they arrive
    loop {
        match rx.recv_timeout(StdDuration::from_secs(1)) {
            Ok(file_path) => {
                // Skip if already processed
                if processed_files.contains(&file_path) {
                    continue;
                }

                // Wait a bit for file to be fully written
                std::thread::sleep(StdDuration::from_millis(500));

                // Check if file is still accessible
                if !file_path.exists() {
                    continue;
                }

                eprintln!("New file detected: {}", file_path.display());

                // Build intent based on operation
                let intent = match &operation {
                    BatchOperation::Convert(format) => {
                        Intent::Convert {
                            input: file_path.clone(),
                            format: *format,
                            quality: None,
                            codec: None,
                        }
                    }
                };

                match app.execute(intent) {
                    Ok(_) => {
                        eprintln!("Processed: {}\n", file_path.display());
                        processed_files.insert(file_path);
                    }
                    Err(e) => {
                        eprintln!("Error processing {}: {}\n", file_path.display(), e);
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Continue watching
                continue;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                eprintln!("Watcher disconnected");
                break;
            }
        }
    }

    Ok(())
}

fn is_media_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_lower.as_str(),
            "mp4" | "avi" | "mov" | "mkv" | "webm" | "flv" | "wmv" | "mpg" | "mpeg" |
            "mp3" | "wav" | "aac" | "flac" | "ogg" | "m4a" | "wma"
        )
    } else {
        false
    }
}

