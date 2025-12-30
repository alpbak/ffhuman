use crate::ffmpeg::step::Step;
use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Instant;

/// Trait for executing FFmpeg steps
pub trait Runner {
    fn run(&self, step: &Step) -> Result<()>;
}

/// CLI-based runner that executes steps using std::process::Command
pub struct CliRunner {
    pub dry_run: bool,
    pub overwrite: bool,
    pub explain: bool,
    pub show_progress: bool,
}

impl CliRunner {
    pub fn new(dry_run: bool, overwrite: bool, explain: bool) -> Self {
        Self {
            dry_run,
            overwrite,
            explain,
            show_progress: true,
        }
    }
}

impl Runner for CliRunner {
    fn run(&self, step: &Step) -> Result<()> {
        if self.explain {
            eprintln!("\n[explain] Running: {} {}", step.program, step.args.join(" "));
        } else {
            eprintln!("Running: {} {}", step.program, step.args.join(" "));
        }

        if self.dry_run {
            return Ok(());
        }

        // For FFmpeg commands, show progress if enabled
        if self.show_progress && step.program == "ffmpeg" {
            self.run_with_progress(step)?;
        } else {
            let mut cmd = Command::new(&step.program);
            cmd.args(&step.args);

            let status = cmd.status().context("failed to execute command")?;
            if !status.success() {
                anyhow::bail!("{} failed with status: {}", step.program, status);
            }
        }
        Ok(())
    }
}

impl CliRunner {
    fn run_with_progress(&self, step: &Step) -> Result<()> {
        let mut cmd = Command::new(&step.program);
        cmd.args(&step.args);
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::null()); // Discard stdout to prevent deadlock

        let mut child = cmd.spawn().context("failed to execute ffmpeg")?;
        let stderr = child.stderr.take().ok_or_else(|| anyhow!("failed to capture stderr"))?;
        
        let reader = BufReader::new(stderr);
        let progress_re = Regex::new(r"time=(\d{2}):(\d{2}):(\d{2})\.(\d{2})").unwrap();
        let frame_re = Regex::new(r"frame=\s*(\d+)").unwrap();
        let fps_re = Regex::new(r"fps=\s*([\d.]+)").unwrap();
        let speed_re = Regex::new(r"speed=\s*([\d.]+)x").unwrap();
        let bitrate_re = Regex::new(r"bitrate=\s*([\d.]+)\s*(\w+)bits/s").unwrap();
        
        let mut last_update = Instant::now();
        let update_interval = std::time::Duration::from_millis(200); // Update every 200ms
        
        let lines = reader.lines();
        for line_result in lines {
            let line = line_result.context("failed to read stderr")?;
            
            // Skip FFmpeg header and metadata lines
            if line.starts_with("ffmpeg version") 
                || line.starts_with("  built with")
                || line.starts_with("  configuration:")
                || line.starts_with("  lib")
                || line.starts_with("Input #")
                || line.starts_with("  Metadata:")
                || line.starts_with("  Duration:")
                || line.starts_with("    Stream #")
                || line.starts_with("Stream mapping:")
                || line.starts_with("Press [q]")
                || line.starts_with("Output #")
                || line.starts_with("[")
                || line.trim().is_empty() {
                continue;
            }
            
            // Parse progress information
            if let Some(caps) = progress_re.captures(&line) {
                let hours: u32 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
                let minutes: u32 = caps.get(2).unwrap().as_str().parse().unwrap_or(0);
                let seconds: u32 = caps.get(3).unwrap().as_str().parse().unwrap_or(0);
                let centiseconds: u32 = caps.get(4).unwrap().as_str().parse().unwrap_or(0);
                
                // Extract other info
                let frame = frame_re.captures(&line)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().parse::<u64>().ok());
                let fps = fps_re.captures(&line)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().parse::<f64>().ok());
                let speed = speed_re.captures(&line)
                    .and_then(|c| c.get(1))
                    .and_then(|m| m.as_str().parse::<f64>().ok());
                let bitrate = bitrate_re.captures(&line).map(|c| {
                    let val: f64 = c.get(1).unwrap().as_str().parse().unwrap_or(0.0);
                    let unit = c.get(2).unwrap().as_str();
                    format!("{:.1}{}", val, unit)
                });
                
                // Only update display periodically to avoid flickering
                if last_update.elapsed() >= update_interval {
                    let time_str = format!("{:02}:{:02}:{:02}.{:02}", hours, minutes, seconds, centiseconds);
                    
                    let mut progress_parts = vec![format!("time={}", time_str)];
                    
                    if let Some(f) = frame {
                        progress_parts.push(format!("frame={}", f));
                    }
                    if let Some(f) = fps {
                        progress_parts.push(format!("fps={:.1}", f));
                    }
                    if let Some(s) = speed {
                        progress_parts.push(format!("speed={:.2}x", s));
                    }
                    if let Some(ref br) = bitrate {
                        progress_parts.push(format!("bitrate={}", br));
                    }
                    
                    // Clear previous line and print progress
                    eprint!("\r\x1B[K"); // Clear line
                    eprint!("  {}", progress_parts.join(" "));
                    std::io::stderr().flush().ok(); // Ensure progress is visible immediately
                    
                    last_update = Instant::now();
                }
            } else if line.contains("error") || line.contains("Error") || line.contains("failed") {
                // Show errors immediately
                eprintln!("\n{}", line);
            }
        }
        
        eprintln!(); // New line after progress
        
        let status = child.wait().context("failed to wait for ffmpeg")?;
        if !status.success() {
            anyhow::bail!("ffmpeg failed with status: {}", status);
        }
        Ok(())
    }
}

