use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::{ConvertFormat, QualityPreset, VideoCodec};
use crate::util::{base_stem, default_out, system::ensure_ffmpeg_exists};
use crate::commands::video;
use anyhow::Result;
use std::path::Path;

pub fn handle_convert(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    format: ConvertFormat,
    quality: Option<QualityPreset>,
    codec: Option<VideoCodec>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    match format {
        ConvertFormat::Gif => {
            let out = default_out(config, input, "gif", "gif")?;

            // Defaults
            let fps = 15;
            let width = 480;

            // Create palette file in output directory to ensure it persists
            let palette_path = if let Some(output_dir) = &config.output_dir {
                std::fs::create_dir_all(output_dir)?;
                output_dir.join("palette.png")
            } else {
                let parent = out.parent().unwrap_or_else(|| Path::new("."));
                std::fs::create_dir_all(parent)?;
                parent.join("palette.png")
            };

            if config.explain {
                eprintln!("[explain] GIF uses palettegen + paletteuse for quality & smaller size.");
                if let Some(q) = quality {
                    eprintln!("[explain] Quality preset: {} (affects fps and resolution)", q);
                }
            }

            let steps = recipes::gif_steps(input, &out, &palette_path, fps, width, quality);
            for step in steps {
                runner.run(&step)?;
            }

            // _palette_file is dropped here, but that's OK - both steps have completed
            eprintln!("Output: {}", out.display());
            Ok(())
        }
        ConvertFormat::Mp4 => {
            let out = default_out(config, input, "convert", "mp4")?;
            let steps = recipes::convert_generic_steps(input, &out, config.overwrite, quality, codec);
            for step in steps {
                runner.run(&step)?;
            }
            eprintln!("Output: {}", out.display());
            Ok(())
        }
        ConvertFormat::Webm => {
            let out = default_out(config, input, "convert", "webm")?;
            // For WebM, default to VP9 codec if not specified
            let webm_codec = codec.or(Some(VideoCodec::Vp9));
            let steps = recipes::convert_generic_steps(input, &out, config.overwrite, quality, webm_codec);
            for step in steps {
                runner.run(&step)?;
            }
            eprintln!("Output: {}", out.display());
            Ok(())
        }
        ConvertFormat::Mp3 | ConvertFormat::Wav => {
            let ext = match format {
                ConvertFormat::Mp3 => "mp3",
                ConvertFormat::Wav => "wav",
                _ => unreachable!(),
            };
            let out = default_out(config, input, "audio", ext)?;
            let steps = recipes::extract_audio_steps(input, &out, ext, config.overwrite);
            for step in steps {
                runner.run(&step)?;
            }
            eprintln!("Output: {}", out.display());
            Ok(())
        }
        ConvertFormat::Iphone => {
            let out = default_out(config, input, "iphone", "mp4")?;
            let steps = recipes::convert_device_steps(input, &out, recipes::DevicePreset::Iphone, quality, config.overwrite);
            for step in steps {
                runner.run(&step)?;
            }
            eprintln!("Output: {}", out.display());
            Ok(())
        }
        ConvertFormat::Android => {
            let out = default_out(config, input, "android", "mp4")?;
            let steps = recipes::convert_device_steps(input, &out, recipes::DevicePreset::Android, quality, config.overwrite);
            for step in steps {
                runner.run(&step)?;
            }
            eprintln!("Output: {}", out.display());
            Ok(())
        }
        ConvertFormat::Hls => {
            let out_dir = if let Some(output_dir) = &config.output_dir {
                std::fs::create_dir_all(output_dir)?;
                output_dir.clone()
            } else {
                let dir = input.parent().unwrap_or_else(|| Path::new("."));
                let dir_name = format!("{}_hls", base_stem(input)?);
                let hls_dir = dir.join(dir_name);
                std::fs::create_dir_all(&hls_dir)?;
                hls_dir
            };
            let steps = recipes::convert_hls_steps(input, &out_dir, quality, config.overwrite)?;
            for step in steps {
                runner.run(&step)?;
            }
            eprintln!("Output directory: {}", out_dir.display());
            Ok(())
        }
        ConvertFormat::Dash => {
            let out_dir = if let Some(output_dir) = &config.output_dir {
                std::fs::create_dir_all(output_dir)?;
                output_dir.clone()
            } else {
                let dir = input.parent().unwrap_or_else(|| Path::new("."));
                let dir_name = format!("{}_dash", base_stem(input)?);
                let dash_dir = dir.join(dir_name);
                std::fs::create_dir_all(&dash_dir)?;
                dash_dir
            };
            let steps = recipes::convert_dash_steps(input, &out_dir, quality, config.overwrite)?;
            for step in steps {
                runner.run(&step)?;
            }
            eprintln!("Output directory: {}", out_dir.display());
            Ok(())
        }
        ConvertFormat::Video360 => {
            // Video360 format is handled by the Convert360 intent, but we support it here
            // for batch operations and edge cases
            video::handle_convert_360(config, runner, input)
        }
    }
}

pub fn handle_animated_gif(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    loop_video: bool,
    optimize: bool,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "animated", "gif")?;

    // Defaults
    let fps = 15;
    let width = 480;

    // Create palette file in output directory to ensure it persists
    let palette_path = if let Some(output_dir) = &config.output_dir {
        std::fs::create_dir_all(output_dir)?;
        output_dir.join("palette.png")
    } else {
        let parent = out.parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(parent)?;
        parent.join("palette.png")
    };

    if config.explain {
        eprintln!("[explain] Animated GIF uses palettegen + paletteuse for quality & smaller size.");
        if loop_video {
            eprintln!("[explain] GIF will loop infinitely.");
        }
        if optimize {
            eprintln!("[explain] GIF will be optimized for smaller file size.");
        }
    }

    let steps = recipes::animated_gif_steps(input, &out, &palette_path, fps, width, loop_video, optimize);
    for step in steps {
        runner.run(&step)?;
    }

    // _palette_file is dropped here, but that's OK - both steps have completed
    eprintln!("Output: {}", out.display());
    Ok(())
}

