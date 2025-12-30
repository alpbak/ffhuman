use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::util::system::ensure_ffmpeg_exists;
use crate::util::default_out;
use anyhow::Result;
use std::path::Path;

pub fn handle_stabilize(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "stabilized", "mp4")?;
    let steps = recipes::stabilize_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_denoise(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "denoised", "mp4")?;
    let steps = recipes::denoise_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_motion_blur(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    radius: Option<u32>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "motion-blur", "mp4")?;
    let steps = recipes::motion_blur_steps(input, &out, radius, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_vignette(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    intensity: Option<f32>,
    size: Option<f32>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "vignette", "mp4")?;
    let steps = recipes::vignette_steps(input, &out, intensity, size, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_lens_correct(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "lens-correct", "mp4")?;
    let steps = recipes::lens_correct_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_interpolate(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    fps: u32,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, &format!("{}fps", fps), "mp4")?;
    let steps = recipes::interpolate_steps(input, &out, fps, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_glitch(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    shift: Option<u32>,
    noise: Option<u32>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "glitch", "mp4")?;
    let steps = recipes::glitch_steps(input, &out, shift, noise, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_vintage_film(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    era: Option<String>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "vintage", "mp4")?;
    let steps = recipes::vintage_film_steps(input, &out, era, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

