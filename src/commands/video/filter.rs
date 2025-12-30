use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::types::{BlurType, ColorPreset, FilterAdjustments};
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::Result;
use std::path::Path;

pub fn handle_grayscale(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "gray", "mp4")?;
    let steps = recipes::grayscale_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_filter(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    adjustments: FilterAdjustments,
    preset: Option<ColorPreset>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    // Determine output suffix based on what filter is applied
    let suffix = if let Some(preset) = preset {
        match preset {
            ColorPreset::Vintage => "vintage",
            ColorPreset::BlackAndWhite => "bw",
            ColorPreset::Sepia => "sepia",
        }
    } else {
        "filtered"
    };

    let out = default_out(config, input, suffix, "mp4")?;
    let steps = recipes::filter_steps(input, &out, adjustments, preset, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_blur(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    blur_type: BlurType,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "blurred", "mp4")?;
    
    match blur_type {
        BlurType::Region(region) => {
            let steps = recipes::blur_steps(input, &out, &region, config.overwrite);
            for step in steps {
                runner.run(&step)?;
            }
        }
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

