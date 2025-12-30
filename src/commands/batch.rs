use crate::app::App;
use crate::config::AppConfig;
use crate::ffmpeg::runner::Runner;
use crate::model::{BatchOperation, Intent};
use crate::util::system::ensure_ffmpeg_exists;
use anyhow::Result;
use glob::glob;
use std::io::{self, Write};

/// Draw a simple progress bar
fn draw_progress_bar(current: usize, total: usize, width: usize) {
    let percentage = if total > 0 {
        (current as f64 / total as f64 * 100.0) as usize
    } else {
        100
    };
    
    let filled = (current * width / total.max(1)).min(width);
    let empty = width.saturating_sub(filled);
    
    let bar = format!(
        "[{}{}] {}% ({}/{})",
        "=".repeat(filled),
        " ".repeat(empty),
        percentage,
        current,
        total
    );
    
    eprint!("\r\x1B[K{}", bar);
    io::stderr().flush().ok();
}

pub fn handle_batch(
    config: &AppConfig,
    _runner: &dyn Runner, // Not used directly, we create our own App
    pattern: &str,
    operation: BatchOperation,
) -> Result<()> {
    ensure_ffmpeg_exists()?;

    // Expand glob pattern to get matching files
    let files: Vec<_> = glob(pattern)?
        .filter_map(|entry| entry.ok())
        .filter(|path| path.is_file())
        .collect();

    if files.is_empty() {
        anyhow::bail!("No files found matching pattern: {}", pattern);
    }

    eprintln!("Processing {} files...\n", files.len());

    // Create a temporary app to execute intents
    let app = App::new(config.clone());
    
    let total = files.len();
    let mut successful = 0;
    let mut failed = 0;

    for (idx, file) in files.iter().enumerate() {
        // Draw progress bar
        draw_progress_bar(idx, total, 50);
        eprintln!("\n[{}/{}] Processing: {}", idx + 1, total, file.display());

        let intent = match &operation {
            BatchOperation::Convert(format) => {
                Intent::Convert {
                    input: file.clone(),
                    format: *format,
                    quality: None,
                    codec: None,
                }
            }
        };

        match app.execute(intent) {
            Ok(_) => {
                successful += 1;
                eprintln!("Completed: {}", file.display());
            }
            Err(e) => {
                failed += 1;
                eprintln!("Error processing {}: {}", file.display(), e);
                // Continue with next file instead of failing completely
            }
        }
    }

    // Final progress bar
    draw_progress_bar(total, total, 50);
    eprintln!("\n\nBatch processing complete!");
    eprintln!(" Successful: {}", successful);
    if failed > 0 {
        eprintln!(" Failed: {}", failed);
    }
    Ok(())
}

pub fn handle_conditional_batch(
    config: &AppConfig,
    _runner: &dyn Runner,
    pattern: &str,
    operation: BatchOperation,
    condition: crate::model::types::ProcessingCondition,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    use crate::ffmpeg::probe::duration_seconds;
    use crate::model::types::Duration;

    // Expand glob pattern to get matching files
    let files: Vec<_> = glob(pattern)?
        .filter_map(|entry| entry.ok())
        .filter(|path| path.is_file())
        .collect();

    if files.is_empty() {
        anyhow::bail!("No files found matching pattern: {}", pattern);
    }

    eprintln!("Processing {} files with condition...\n", files.len());

    // Create a temporary app to execute intents
    let app = App::new(config.clone());
    
    let total = files.len();
    let mut successful = 0;
    let mut failed = 0;
    let mut skipped = 0;

    for (idx, file) in files.iter().enumerate() {
        eprintln!("\n[{}/{}] Checking: {}", idx + 1, total, file.display());

        // Check condition
        let should_process = match &condition {
            crate::model::types::ProcessingCondition::DurationLessThan(threshold) => {
                match duration_seconds(file) {
                    Ok(duration) => {
                        let file_duration = Duration { seconds: duration };
                        let result = file_duration.to_seconds() < threshold.to_seconds();
                        if !result {
                            eprintln!("Skipped: duration {}s >= threshold {}s", 
                                file_duration.to_seconds(), threshold.to_seconds());
                        }
                        result
                    }
                    Err(e) => {
                        eprintln!(" Warning: Could not get duration: {}", e);
                        false
                    }
                }
            }
            crate::model::types::ProcessingCondition::DurationGreaterThan(threshold) => {
                match duration_seconds(file) {
                    Ok(duration) => {
                        let file_duration = Duration { seconds: duration };
                        let result = file_duration.to_seconds() > threshold.to_seconds();
                        if !result {
                            eprintln!("Skipped: duration {}s <= threshold {}s", 
                                file_duration.to_seconds(), threshold.to_seconds());
                        }
                        result
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not get duration: {}", e);
                        false
                    }
                }
            }
            crate::model::types::ProcessingCondition::DurationEquals(threshold) => {
                match duration_seconds(file) {
                    Ok(duration) => {
                        let file_duration = Duration { seconds: duration };
                        // Use a small epsilon for floating point comparison
                        let result = (file_duration.to_seconds() - threshold.to_seconds()).abs() < 0.1;
                        if !result {
                            eprintln!("Skipped: duration {}s != threshold {}s", 
                                file_duration.to_seconds(), threshold.to_seconds());
                        }
                        result
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not get duration: {}", e);
                        false
                    }
                }
            }
        };

        if !should_process {
            skipped += 1;
            continue;
        }

        // Draw progress bar
        draw_progress_bar(idx, total, 50);
        eprintln!("\n[{}/{}] Processing: {}", idx + 1, total, file.display());

        let intent = match &operation {
            BatchOperation::Convert(format) => {
                Intent::Convert {
                    input: file.clone(),
                    format: *format,
                    quality: None,
                    codec: None,
                }
            }
        };

        match app.execute(intent) {
            Ok(_) => {
                successful += 1;
                eprintln!("Completed: {}", file.display());
            }
            Err(e) => {
                failed += 1;
                eprintln!("Error processing {}: {}", file.display(), e);
            }
        }
    }

    // Final progress bar
    draw_progress_bar(total, total, 50);
    eprintln!("\n\nConditional batch processing complete!");
    eprintln!(" Successful: {}", successful);
    eprintln!(" Skipped: {}", skipped);
    if failed > 0 {
        eprintln!(" Failed: {}", failed);
    }
    Ok(())
}

