use crate::config::AppConfig;
use crate::ffmpeg::probe::duration_seconds;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::types::CompressTarget;
use crate::util::{default_out, system::{ensure_ffmpeg_exists, ensure_ffprobe_exists}};
use anyhow::Result;
use std::path::Path;

pub fn handle_compress(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    target: CompressTarget,
    two_pass: bool,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "compressed", "mp4")?;

    match target {
        CompressTarget::Size(target_size) => {
            ensure_ffprobe_exists()?;
            let duration = duration_seconds(input)?;

            if config.explain {
                let total_bps = (target_size.bytes as f64 * 8.0 / duration).max(50_000.0);
                let audio_bps = (total_bps * 0.08).clamp(96_000.0, 160_000.0);
                let video_bps = (total_bps - audio_bps).max(50_000.0);
                let v_kbps = (video_bps / 1000.0).floor() as u64;
                let a_kbps = (audio_bps / 1000.0).floor() as u64;
                eprintln!(
                    "[explain] Target size={} bytes, duration={:.2}s => total≈{} kbps, video≈{} kbps, audio≈{} kbps",
                    target_size.bytes,
                    duration,
                    (total_bps / 1000.0).round(),
                    v_kbps,
                    a_kbps
                );
                if two_pass {
                    eprintln!("[explain] Using 2-pass libx264 for more accurate size targeting.");
                } else {
                    eprintln!("[explain] Using single-pass encoding.");
                }
            }

            let steps = recipes::compress_steps(input, &out, target_size.bytes, duration, config.overwrite, two_pass);
            for step in steps {
                runner.run(&step)?;
            }
        }
        CompressTarget::Bitrate(target_bitrate) => {
            if config.explain {
                let v_kbps = (target_bitrate.bps / 1000).max(50);
                let audio_bps = (target_bitrate.bps as f64 * 0.08).clamp(96_000.0, 160_000.0);
                let a_kbps = (audio_bps / 1000.0).floor() as u64;
                eprintln!(
                    "[explain] Target bitrate={} => video≈{} kbps, audio≈{} kbps",
                    target_bitrate,
                    v_kbps,
                    a_kbps
                );
                if two_pass {
                    eprintln!("[explain] Using 2-pass libx264 for more accurate bitrate targeting.");
                } else {
                    eprintln!("[explain] Using single-pass encoding.");
                }
            }

            let steps = recipes::compress_bitrate_steps(input, &out, target_bitrate.bps, config.overwrite, two_pass);
            for step in steps {
                runner.run(&step)?;
            }
        }
        CompressTarget::Quality(quality) => {
            if config.explain {
                if two_pass {
                    eprintln!("[explain] Using 2-pass encoding with CRF={} for {} quality preset.", quality.crf_value(), quality);
                } else {
                    eprintln!("[explain] Using CRF={} for {} quality preset.", quality.crf_value(), quality);
                    eprintln!("[explain] CRF encoding provides consistent quality with variable bitrate.");
                }
            }

            let steps = recipes::compress_quality_steps(input, &out, quality, config.overwrite, two_pass);
            for step in steps {
                runner.run(&step)?;
            }
        }
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

