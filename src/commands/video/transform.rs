use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::{FlipDirection, ResizeTarget, RotateDegrees, SpeedFactor};
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::Result;
use std::path::Path;

pub fn handle_resize(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    target: ResizeTarget,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "resized", "mp4")?;
    let steps = recipes::resize_steps(input, &out, &target, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_speed_up(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    factor: SpeedFactor,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "speed", "mp4")?;
    let steps = recipes::speed_up_steps(input, &out, &factor, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_slow_down(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    factor: SpeedFactor,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "speed", "mp4")?;
    let steps = recipes::speed_down_steps(input, &out, &factor, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_timelapse(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    speed: SpeedFactor,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "timelapse", "mp4")?;
    
    if config.explain {
        eprintln!("[explain] Creating time-lapse by speeding up video {}x", speed);
        eprintln!("[explain] Using setpts for video and atempo chain for audio");
    }
    
    // Reuse speed_up_steps recipe - time-lapse is essentially the same operation
    let steps = recipes::speed_up_steps(input, &out, &speed, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_reverse(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "reverse", "mp4")?;
    let steps = recipes::reverse_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_rotate(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    degrees: RotateDegrees,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "rotated", "mp4")?;
    let steps = recipes::rotate_steps(input, &out, &degrees, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_flip(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    direction: FlipDirection,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "flip", "mp4")?;
    let steps = recipes::flip_steps(input, &out, &direction, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}


pub fn handle_fix_rotation(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "fixed_rotation", "mp4")?;
    let steps = recipes::fix_rotation_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_tile(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    layout: crate::model::types::MontageLayout,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "tiled", "mp4")?;
    let steps = recipes::tile_steps(input, &out, &layout, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_convert_360(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "360", "mp4")?;
    let steps = recipes::convert_360_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_convert_hdr_to_sdr(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "sdr", "mp4")?;
    let steps = recipes::convert_hdr_to_sdr_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_fix_framerate(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "fixed_framerate", "mp4")?;
    let steps = recipes::fix_framerate_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

