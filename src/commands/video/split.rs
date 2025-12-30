use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::types::SplitMode;
use crate::util::system::ensure_ffmpeg_exists;
use anyhow::Result;
use std::path::Path;

pub fn handle_split(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    mode: SplitMode,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    // Create output directory for segments
    let output_dir = if let Some(output_dir) = &config.output_dir {
        std::fs::create_dir_all(output_dir)?;
        output_dir.clone()
    } else {
        let dir = input.parent().unwrap_or_else(|| Path::new("."));
        let dir_name = format!("{}_split", input.file_stem().unwrap().to_string_lossy());
        let split_dir = dir.join(dir_name);
        std::fs::create_dir_all(&split_dir)?;
        split_dir
    };

    let steps = recipes::split_steps(input, &output_dir, &mode, config.overwrite)?;
    
    eprintln!("Splitting into {} segments...", steps.len());
    for (idx, step) in steps.iter().enumerate() {
        eprintln!(" Processing segment {}/{}...", idx + 1, steps.len());
        runner.run(step)?;
    }

    eprintln!("Output directory: {}", output_dir.display());
    Ok(())
}

