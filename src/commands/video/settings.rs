use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::Result;
use std::path::Path;

pub fn handle_set_fps(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    fps: u32,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "fps", "mp4")?;
    let steps = recipes::set_fps_steps(input, &out, fps, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_generate_test_pattern(
    config: &AppConfig,
    runner: &dyn Runner,
    resolution: &str,
    duration: crate::model::types::Duration,
) -> Result<()> {
    ensure_ffmpeg_exists()?;

    let out = if let Some(out) = &config.out {
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        out.clone()
    } else {
        let dir = if let Some(output_dir) = &config.output_dir {
            std::fs::create_dir_all(output_dir)?;
            output_dir
        } else {
            std::path::Path::new(".")
        };
        dir.join("test_pattern.mp4")
    };

    let steps = recipes::generate_test_pattern_steps(resolution, &duration, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_add_timecode(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "timecode", "mp4")?;
    let steps = recipes::add_timecode_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_proxy(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "proxy", "mp4")?;
    let steps = recipes::proxy_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_convert_colorspace(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    target: crate::model::types::Colorspace,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "colorspace", "mp4")?;
    let steps = recipes::convert_colorspace_steps(input, &out, &target, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

