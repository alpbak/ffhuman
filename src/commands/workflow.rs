use crate::app::App;
use crate::config::AppConfig;
use crate::ffmpeg::runner::Runner;
use crate::model::Intent;
use crate::util::system::ensure_ffmpeg_exists;
use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

/// Workflow step definition
#[derive(Debug, Clone)]
struct WorkflowStep {
    operation: String,
    input: String,
    #[allow(dead_code)]
    output: Option<String>, // Reserved for future use
    params: serde_json::Value,
}

/// Workflow configuration
#[derive(Debug)]
struct WorkflowConfig {
    steps: Vec<WorkflowStep>,
}

pub fn handle_workflow(
    config: &AppConfig,
    _runner: &dyn Runner,
    config_file: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let config_file = config_file.as_ref();

    if !config_file.exists() {
        anyhow::bail!("Workflow file not found: {}", config_file.display());
    }

    // Read and parse workflow file
    let content = fs::read_to_string(config_file)?;
    let workflow = parse_workflow(&content)?;

    eprintln!("Processing workflow: {} steps", workflow.steps.len());

    // Create app to execute intents
    let app = App::new(config.clone());

    for (idx, step) in workflow.steps.iter().enumerate() {
        eprintln!("\n[{}/{}] Executing: {}", idx + 1, workflow.steps.len(), step.operation);

        let intent = build_intent_from_step(step)?;
        
        match app.execute(intent) {
            Ok(_) => {
                eprintln!("Step {} completed", idx + 1);
            }
            Err(e) => {
                anyhow::bail!("Step {} failed: {}", idx + 1, e);
            }
        }
    }

    eprintln!("\n Workflow completed successfully!");
    Ok(())
}

fn parse_workflow(content: &str) -> Result<WorkflowConfig> {
    // Simple YAML-like parser for workflow files
    // Expected format:
    // steps:
    //   - operation: convert
    //     input: video.mp4
    //     output: video.gif
    //     params:
    //       format: gif
    
    let mut steps = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    // Find steps section
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with("steps:") {
            i += 1;
            break;
        }
        i += 1;
    }
    
    // Parse steps
    while i < lines.len() {
        let line = lines[i].trim();
        
        if line.starts_with("- operation:") {
            let operation = line.strip_prefix("- operation:").unwrap_or("").trim().to_string();
            i += 1;
            
            let mut input = String::new();
            let mut output = None;
            let mut params = serde_json::json!({});
            
            while i < lines.len() {
                let step_line = lines[i].trim();
                if step_line.is_empty() || (!step_line.starts_with(" ") && !step_line.starts_with("-")) {
                    break;
                }
                
                if step_line.starts_with("input:") {
                    input = step_line.strip_prefix("input:").unwrap_or("").trim().to_string();
                } else if step_line.starts_with("output:") {
                    output = Some(step_line.strip_prefix("output:").unwrap_or("").trim().to_string());
                } else if step_line.starts_with("params:") {
                    i += 1;
                    while i < lines.len() {
                        let param_line = lines[i].trim();
                        if param_line.is_empty() || !param_line.starts_with(" ") {
                            i -= 1;
                            break;
                        }
                        if param_line.contains(":") {
                            let parts: Vec<&str> = param_line.splitn(2, ':').collect();
                            if parts.len() == 2 {
                                let key = parts[0].trim();
                                let value = parts[1].trim().trim_matches('"');
                                params[key] = serde_json::Value::String(value.to_string());
                            }
                        }
                        i += 1;
                    }
                }
                i += 1;
            }
            
            steps.push(WorkflowStep {
                operation,
                input,
                output,
                params,
            });
        } else {
            i += 1;
        }
    }
    
    if steps.is_empty() {
        anyhow::bail!("No steps found in workflow file");
    }
    
    Ok(WorkflowConfig { steps })
}

fn build_intent_from_step(step: &WorkflowStep) -> Result<Intent> {
    use crate::model::*;
    use std::path::PathBuf;
    
    let input = PathBuf::from(&step.input);
    
    match step.operation.as_str() {
        "convert" => {
            let format_str = step.params.get("format")
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
                input,
                format,
                quality: None,
                codec: None,
            })
        }
        "trim" => {
            let start_str = step.params.get("start")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("trim operation requires 'start' parameter"))?;
            let end_str = step.params.get("end")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("trim operation requires 'end' parameter"))?;
            
            Ok(Intent::Trim {
                input,
                start: Time::parse(start_str)?,
                end: Time::parse(end_str)?,
            })
        }
        "resize" => {
            let target_str = step.params.get("target")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("resize operation requires 'target' parameter"))?;
            
            Ok(Intent::Resize {
                input,
                target: ResizeTarget::parse(target_str)?,
            })
        }
        "compress" => {
            let target_str = step.params.get("target")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("compress operation requires 'target' parameter"))?;
            
            let two_pass = step.params.get("two_pass")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            // Try to parse as quality preset first
            let target_lower = target_str.trim().to_lowercase();
            let target = if target_lower.ends_with("-quality") {
                let quality_str = target_lower.strip_suffix("-quality").unwrap().trim();
                CompressTarget::Quality(QualityPreset::parse(quality_str)?)
            } else {
                // Try to parse as bitrate first
                let target_trimmed = target_str.trim();
                let target_lower_trimmed = target_trimmed.to_lowercase();
                let looks_like_bitrate = target_lower_trimmed.ends_with("bps")
                    || target_lower_trimmed.ends_with("kbps")
                    || target_lower_trimmed.ends_with("mbps")
                    || target_lower_trimmed.ends_with("gbps")
                    || (target_lower_trimmed.ends_with('k') && !target_lower_trimmed.ends_with("mk"))
                    || (target_lower_trimmed.ends_with('m') && !target_lower_trimmed.ends_with("mb") && !target_lower_trimmed.ends_with("gb"))
                    || (target_lower_trimmed.ends_with('g') && !target_lower_trimmed.ends_with("gb"));
                
                if looks_like_bitrate {
                    if let Ok(bitrate) = TargetBitrate::parse(target_str) {
                        CompressTarget::Bitrate(bitrate)
                    } else {
                        CompressTarget::Size(TargetSize::parse(target_str)?)
                    }
                } else {
                    CompressTarget::Size(TargetSize::parse(target_str)?)
                }
            };
            
            Ok(Intent::Compress {
                input,
                target,
                two_pass,
            })
        }
        op => anyhow::bail!("Unsupported operation: {}", op),
    }
}

