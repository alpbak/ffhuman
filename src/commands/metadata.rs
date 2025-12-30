use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::types::MetadataField;
use crate::util::{default_out, system::ensure_ffmpeg_exists, system::ensure_ffprobe_exists};
use anyhow::{Result, Context};
use std::path::Path;
use std::io::Write;

pub fn handle_set_metadata(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    field: MetadataField,
    value: &str,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "metadata", "mp4")?;
    let steps = recipes::set_metadata_steps(input, &out, &field, value, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}


pub fn handle_extract_metadata(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    format: crate::model::intent::MetadataFormat,
) -> Result<()> {
    ensure_ffprobe_exists()?;
    let input = input.as_ref();

    let ext = match format {
        crate::model::intent::MetadataFormat::Json => "json",
        crate::model::intent::MetadataFormat::Xml => "xml",
    };
    
    let out = if let Some(out_path) = &config.out {
        out_path.clone()
    } else {
        let dir = if let Some(output_dir) = &config.output_dir {
            std::fs::create_dir_all(output_dir)?;
            output_dir
        } else {
            input.parent().unwrap_or_else(|| Path::new("."))
        };
        dir.join(format!("{}_metadata.{}", 
            input.file_stem().unwrap().to_string_lossy(), ext))
    };

    let format_str = match format {
        crate::model::intent::MetadataFormat::Json => "json",
        crate::model::intent::MetadataFormat::Xml => "xml",
    };
    
    let steps = recipes::extract_metadata_steps(input, &out, format_str, config.overwrite);
    
    // For metadata extraction, we need to capture output and write to file
    // since ffprobe outputs to stdout
    if !config.dry_run {
        let step = &steps[0];
        use std::process::Command;
        let output = Command::new(&step.program)
            .args(&step.args)
            .output()
            .context("ffprobe failed")?;
        
        if !output.status.success() {
            anyhow::bail!("ffprobe failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        
        let mut file = std::fs::File::create(&out)?;
        file.write_all(&output.stdout)?;
    } else {
        // In dry-run mode, just show what would be executed
        for step in steps {
            runner.run(&step)?;
        }
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_info(
    _config: &AppConfig,
    _runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    use crate::ffmpeg::probe;
    use crate::util::system::ensure_ffprobe_exists;
    
    ensure_ffprobe_exists()?;
    let input = input.as_ref();

    let video_info = probe::get_video_info(input)?;
    
    eprintln!("Video Information");
    eprintln!("----------------------------------------");
    eprintln!("File: {}", input.display());
    eprintln!("Duration: {:.2}s ({})", video_info.duration, format_duration(video_info.duration));
    eprintln!("Resolution: {}x{}", video_info.width, video_info.height);
    eprintln!("Aspect Ratio: {:.2}", video_info.width as f64 / video_info.height as f64);
    eprintln!("Frame Rate: {:.2} fps", video_info.fps);
    eprintln!("Video Codec: {}", video_info.video_codec);
    eprintln!("Video Bitrate: {}", video_info.video_bitrate);
    eprintln!("Audio Codec: {}", video_info.audio_codec);
    eprintln!("Audio Bitrate: {}", video_info.audio_bitrate);
    eprintln!("Total Bitrate: {}", video_info.total_bitrate);
    eprintln!("File Size: {}", video_info.file_size);
    eprintln!("----------------------------------------");
    
    Ok(())
}

fn format_duration(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}:{:02}", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn handle_export_edl(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffprobe_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "edl", "txt")?;
    let steps = recipes::export_edl_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

