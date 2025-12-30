use crate::app::App;
use crate::config::AppConfig;
use crate::ffmpeg::runner::Runner;
use crate::model::Intent;
use crate::util::system::ensure_ffmpeg_exists;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;
use serde_yaml::Value;

pub fn handle_apply_template(
    config: &AppConfig,
    _runner: &dyn Runner,
    input: impl AsRef<Path>,
    template_file: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();
    let template_file = template_file.as_ref();

    if !input.exists() {
        anyhow::bail!("Input file not found: {}", input.display());
    }

    if !template_file.exists() {
        anyhow::bail!("Template file not found: {}", template_file.display());
    }

    // Read and parse template file
    let content = fs::read_to_string(template_file)?;
    let template: Value = serde_yaml::from_str(&content)
        .map_err(|e| anyhow!("Failed to parse template YAML: {}", e))?;

    eprintln!("Applying template: {}", template_file.display());
    eprintln!("Input: {}\n", input.display());

    // Create app to execute intents
    let app = App::new(config.clone());

    // Process template operations
    if let Some(operations) = template.get("operations").and_then(|v| v.as_sequence()) {
        for (idx, op) in operations.iter().enumerate() {
            eprintln!("[{}/{}] Processing operation...", idx + 1, operations.len());

            let intent = build_intent_from_template_op(input, op)?;
            
            match app.execute(intent) {
                Ok(_) => {
                    eprintln!("Operation {} completed\n", idx + 1);
                }
                Err(e) => {
                    anyhow::bail!("Operation {} failed: {}", idx + 1, e);
                }
            }
        }
    } else {
        anyhow::bail!("Template must contain 'operations' array");
    }

    eprintln!("Template applied successfully!");
    Ok(())
}

fn build_intent_from_template_op(input: &Path, op: &Value) -> Result<Intent> {
    use crate::model::*;
    use std::path::PathBuf;

    let op_type = op.get("type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Operation must have 'type' field"))?;

    // For now, use the input path. In a full implementation, we'd track intermediate outputs
    let current_input = PathBuf::from(input);

    match op_type {
        "convert" => {
            let format_str = op.get("format")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("convert operation requires 'format' parameter"))?;
            
            let format = match format_str {
                "gif" => ConvertFormat::Gif,
                "mp4" => ConvertFormat::Mp4,
                "webm" => ConvertFormat::Webm,
                "mp3" => ConvertFormat::Mp3,
                "wav" => ConvertFormat::Wav,
                _ => anyhow::bail!("Unsupported format: {}", format_str),
            };

            Ok(Intent::Convert {
                input: current_input,
                format,
                quality: None,
                codec: None,
            })
        }
        "trim" => {
            let start_str = op.get("start")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("trim operation requires 'start' parameter"))?;
            let end_str = op.get("end")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("trim operation requires 'end' parameter"))?;
            
            Ok(Intent::Trim {
                input: current_input,
                start: Time::parse(start_str)?,
                end: Time::parse(end_str)?,
            })
        }
        "resize" => {
            let target_str = op.get("target")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("resize operation requires 'target' parameter"))?;
            
            Ok(Intent::Resize {
                input: current_input,
                target: ResizeTarget::parse(target_str)?,
            })
        }
        "compress" => {
            let target_str = op.get("target")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("compress operation requires 'target' parameter"))?;
            
            let two_pass = op.get("two_pass")
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
                input: current_input,
                target,
                two_pass,
            })
        }
        _ => anyhow::bail!("Unsupported operation type: {}", op_type),
    }
}

