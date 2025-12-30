use crate::app::App;
use crate::config::AppConfig;
use crate::ffmpeg::runner::Runner;
use crate::model::Intent;
use crate::util::system::ensure_ffmpeg_exists;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};
use serde_yaml::Value;

pub fn handle_pipeline(
    config: &AppConfig,
    _runner: &dyn Runner,
    input: impl AsRef<Path>,
    steps_file: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();
    let steps_file = steps_file.as_ref();

    if !input.exists() {
        anyhow::bail!("Input file not found: {}", input.display());
    }

    if !steps_file.exists() {
        anyhow::bail!("Steps file not found: {}", steps_file.display());
    }

    // Read and parse steps file
    let content = fs::read_to_string(steps_file)?;
    let steps: Value = serde_yaml::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse steps YAML: {}", e))?;

    eprintln!("Processing pipeline: {}", steps_file.display());
    eprintln!("Input: {}\n", input.display());

    // Create app to execute intents
    let app = App::new(config.clone());

    // Process pipeline steps sequentially
    if let Some(steps_array) = steps.get("steps").and_then(|v| v.as_sequence()) {
        let mut current_input = PathBuf::from(input);

        for (idx, step) in steps_array.iter().enumerate() {
            eprintln!("[{}/{}] Executing step...", idx + 1, steps_array.len());

            let intent = build_intent_from_step(&current_input, step)?;
            
            match app.execute(intent.clone()) {
                Ok(_) => {
                    eprintln!("Step {} completed", idx + 1);
                    
                    // Update current_input to the output of this step
                    // Compute output path using the same logic as the handlers
                    if let Some(output_path) = get_output_path_from_intent(&intent, &current_input, config) {
                        current_input = output_path;
                        eprintln!("Output: {}", current_input.display());
                    } else {
                        anyhow::bail!("Could not determine output path for step {}", idx + 1);
                    }
                }
                Err(e) => {
                    anyhow::bail!("Step {} failed: {}", idx + 1, e);
                }
            }
        }
    } else {
        anyhow::bail!("Steps file must contain 'steps' array");
    }

    eprintln!("\n Pipeline completed successfully!");
    Ok(())
}

fn build_intent_from_step(input: &Path, step: &Value) -> Result<Intent> {
    use crate::model::*;

    let step_type = step.get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Step must have 'type' field"))?;

    match step_type {
        "convert" => {
            let format_str = step.get("format")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("convert step requires 'format' parameter"))?;
            
            let format = match format_str {
                "gif" => ConvertFormat::Gif,
                "mp4" => ConvertFormat::Mp4,
                "webm" => ConvertFormat::Webm,
                "mp3" => ConvertFormat::Mp3,
                "wav" => ConvertFormat::Wav,
                _ => anyhow::bail!("Unsupported format: {}", format_str),
            };

            Ok(Intent::Convert {
                input: PathBuf::from(input),
                format,
                quality: None,
                codec: None,
            })
        }
        "trim" => {
            let start_str = step.get("start")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("trim step requires 'start' parameter"))?;
            let end_str = step.get("end")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("trim step requires 'end' parameter"))?;
            
            Ok(Intent::Trim {
                input: PathBuf::from(input),
                start: Time::parse(start_str)?,
                end: Time::parse(end_str)?,
            })
        }
        "resize" => {
            let target_str = step.get("target")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("resize step requires 'target' parameter"))?;
            
            Ok(Intent::Resize {
                input: PathBuf::from(input),
                target: ResizeTarget::parse(target_str)?,
            })
        }
        "compress" => {
            let target_str = step.get("target")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("compress step requires 'target' parameter"))?;
            
            let two_pass = step.get("two_pass")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            let target_lower = target_str.trim().to_lowercase();
            let target = if target_lower.ends_with("-quality") {
                let quality_str = target_lower.strip_suffix("-quality").unwrap().trim();
                CompressTarget::Quality(QualityPreset::parse(quality_str)?)
            } else {
                CompressTarget::Size(TargetSize::parse(target_str)?)
            };
            
            Ok(Intent::Compress {
                input: PathBuf::from(input),
                target,
                two_pass,
            })
        }
        _ => anyhow::bail!("Unsupported step type: {}", step_type),
    }
}

fn get_output_path_from_intent(intent: &Intent, _input: &Path, config: &AppConfig) -> Option<PathBuf> {
    use crate::util::default_out;
    
    // Use the same output path calculation as the handlers
    match intent {
        Intent::Convert { input: intent_input, format, .. } => {
            let suffix = match format {
                crate::model::ConvertFormat::Gif => "gif",
                crate::model::ConvertFormat::Mp4 => "convert",
                crate::model::ConvertFormat::Webm => "convert",
                crate::model::ConvertFormat::Mp3 => "audio",
                crate::model::ConvertFormat::Wav => "audio",
                crate::model::ConvertFormat::Dash => "dash",
                _ => return None,
            };
            let ext = match format {
                crate::model::ConvertFormat::Gif => "gif",
                crate::model::ConvertFormat::Mp4 => "mp4",
                crate::model::ConvertFormat::Webm => "webm",
                crate::model::ConvertFormat::Mp3 => "mp3",
                crate::model::ConvertFormat::Wav => "wav",
                _ => return None,
            };
            default_out(config, intent_input, suffix, ext).ok()
        }
        Intent::Trim { input: intent_input, .. } => {
            default_out(config, intent_input, "trim", "mp4").ok()
        }
        Intent::Resize { input: intent_input, .. } => {
            default_out(config, intent_input, "resize", "mp4").ok()
        }
        Intent::Compress { input: intent_input, .. } => {
            default_out(config, intent_input, "compressed", "mp4").ok()
        }
        _ => None,
    }
}

