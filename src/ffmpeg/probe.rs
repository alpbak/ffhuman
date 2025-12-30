use crate::util::system::ensure_ffprobe_exists;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Video information structure
pub struct VideoInfo {
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub video_codec: String,
    pub video_bitrate: String,
    pub audio_codec: String,
    pub audio_bitrate: String,
    pub total_bitrate: String,
    pub file_size: String,
}

/// Get the duration of a media file in seconds using ffprobe
pub fn duration_seconds(input: &Path) -> Result<f64> {
    ensure_ffprobe_exists()?;
    let out = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(input)
        .output()
        .context("ffprobe failed")?;

    if !out.status.success() {
        anyhow::bail!("ffprobe could not read duration");
    }
    let s = String::from_utf8(out.stdout)?.trim().to_string();
    let dur = s.parse::<f64>().context("failed to parse duration")?;
    Ok(dur.max(0.01))
}

/// Get comprehensive video information using ffprobe
pub fn get_video_info(input: &Path) -> Result<VideoInfo> {
    ensure_ffprobe_exists()?;
    
    // Get format info (duration, bitrate, size)
    let format_out = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-show_entries", "format=duration,bit_rate,size",
            "-of", "json",
        ])
        .arg(input)
        .output()
        .context("ffprobe format failed")?;
    
    if !format_out.status.success() {
        anyhow::bail!("ffprobe could not read format info");
    }
    
    let format_json: serde_json::Value = serde_json::from_slice(&format_out.stdout)
        .context("failed to parse format JSON")?;
    
    let format = format_json.get("format").ok_or_else(|| anyhow::anyhow!("No format in JSON"))?;
    let duration: f64 = format.get("duration")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.0);
    let total_bitrate: u64 = format.get("bit_rate")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let file_size: u64 = format.get("size")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    // Get video stream info
    let video_out = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "v:0",
            "-show_entries", "stream=codec_name,width,height,r_frame_rate,bit_rate",
            "-of", "json",
        ])
        .arg(input)
        .output()
        .context("ffprobe video failed")?;
    
    if !video_out.status.success() {
        anyhow::bail!("ffprobe could not read video stream");
    }
    
    let video_json: serde_json::Value = serde_json::from_slice(&video_out.stdout)
        .context("failed to parse video JSON")?;
    
    let streams = video_json.get("streams")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("No streams in JSON"))?;
    
    let video_stream = streams.get(0)
        .ok_or_else(|| anyhow::anyhow!("No video stream found"))?;
    
    let width: u32 = video_stream.get("width")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(0);
    let height: u32 = video_stream.get("height")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(0);
    let video_codec = video_stream.get("codec_name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let video_bitrate: u64 = video_stream.get("bit_rate")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    // Parse frame rate (format: "30/1" or "30000/1001")
    let fps = video_stream.get("r_frame_rate")
        .and_then(|v| v.as_str())
        .and_then(|s| {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() == 2 {
                let num: f64 = parts[0].parse().ok()?;
                let den: f64 = parts[1].parse().ok()?;
                if den > 0.0 {
                    Some(num / den)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(0.0);
    
    // Get audio stream info
    let audio_out = Command::new("ffprobe")
        .args([
            "-v", "error",
            "-select_streams", "a:0",
            "-show_entries", "stream=codec_name,bit_rate",
            "-of", "json",
        ])
        .arg(input)
        .output()
        .context("ffprobe audio failed")?;
    
    let audio_codec = if audio_out.status.success() {
        if let Ok(audio_json) = serde_json::from_slice::<serde_json::Value>(&audio_out.stdout) {
            audio_json.get("streams")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.get(0))
                .and_then(|s| s.get("codec_name"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "none".to_string())
        } else {
            "none".to_string()
        }
    } else {
        "none".to_string()
    };
    
    let audio_bitrate: u64 = if audio_out.status.success() {
        if let Ok(audio_json) = serde_json::from_slice::<serde_json::Value>(&audio_out.stdout) {
            audio_json.get("streams")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.get(0))
                .and_then(|s| s.get("bit_rate"))
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok())
                .unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    };
    
    // Format bitrates and file size
    let format_bitrate = |bits: u64| -> String {
        if bits >= 1_000_000 {
            format!("{:.1} Mbps", bits as f64 / 1_000_000.0)
        } else if bits >= 1_000 {
            format!("{:.1} kbps", bits as f64 / 1_000.0)
        } else {
            format!("{} bps", bits)
        }
    };
    
    let format_size = |bytes: u64| -> String {
        if bytes >= 1_000_000_000 {
            format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
        } else if bytes >= 1_000_000 {
            format!("{:.2} MB", bytes as f64 / 1_000_000.0)
        } else if bytes >= 1_000 {
            format!("{:.2} KB", bytes as f64 / 1_000.0)
        } else {
            format!("{} B", bytes)
        }
    };
    
    Ok(VideoInfo {
        duration,
        width,
        height,
        fps,
        video_codec,
        video_bitrate: format_bitrate(video_bitrate),
        audio_codec,
        audio_bitrate: format_bitrate(audio_bitrate),
        total_bitrate: format_bitrate(total_bitrate),
        file_size: format_size(file_size),
    })
}

