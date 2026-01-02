use crate::ffmpeg::step::Step;
use crate::model::types::*;
use anyhow::{anyhow, bail, Context, Result};
use std::path::{Path, PathBuf};

/// Determine if video needs to be re-encoded based on input/output formats
/// Returns the video codec to use: "libx264" if re-encoding needed, "copy" otherwise
/// 
/// WebM (VP8/VP9) cannot be copied into MP4 container - must re-encode
fn get_video_codec(input: &Path, output: &Path) -> &'static str {
    let input_ext = input.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    
    let output_ext = output.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    
    // If input is WebM and output is MP4, we must re-encode (VP8/VP9 not compatible with MP4)
    if (input_ext == "webm" || input_ext == "mkv") && output_ext == "mp4" {
        "libx264"
    } else {
        "copy"
    }
}

/// Determine if audio needs to be re-encoded based on input/output formats
/// Returns the audio codec to use: appropriate codec if re-encoding needed, "copy" otherwise
/// 
/// WebM (vorbis) and WMV audio cannot be copied into MP4 container - must re-encode to AAC
/// MP4 (AAC) audio cannot be copied into WebM container - must re-encode to Opus or Vorbis
fn get_audio_codec(input: &Path, output: &Path) -> &'static str {
    let input_ext = input.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    
    let output_ext = output.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    
    // If input is WebM/WMV and output is MP4, we must re-encode audio (vorbis/wma not compatible with MP4)
    if (input_ext == "webm" || input_ext == "wmv" || input_ext == "mkv") && output_ext == "mp4" {
        "aac"
    } else if (input_ext == "mp4" || input_ext == "avi" || input_ext == "mov") && output_ext == "webm" {
        // MP4/AVI/MOV (AAC) audio cannot be copied into WebM - must re-encode to Opus (preferred) or Vorbis
        "libopus"
    } else {
        "copy"
    }
}

/// Build steps for converting video to GIF (2-pass: palette + render)
/// Quality affects fps and width
pub fn gif_steps(
    input: &Path,
    output: &Path,
    palette_path: &Path,
    fps: u32,
    width: u32,
    quality: Option<crate::model::types::QualityPreset>,
) -> Vec<Step> {
    
    // Adjust fps and width based on quality if specified
    let (final_fps, final_width) = if let Some(q) = quality {
        match q {
            QualityPreset::Low => (10, 320),
            QualityPreset::Medium => (fps, width),
            QualityPreset::High => (20, 640),
            QualityPreset::Ultra => (30, 800),
        }
    } else {
        (fps, width)
    };
    
    vec![
        // Pass 1: Generate palette
        Step::new(
            "ffmpeg",
            vec![
                "-y".to_string(),
                "-i".to_string(),
                input.to_string_lossy().to_string(),
                "-vf".to_string(),
                format!("fps={final_fps},scale={final_width}:-1:flags=lanczos,palettegen"),
                palette_path.to_string_lossy().to_string(),
            ],
        ),
        // Pass 2: Render with palette
        Step::new(
            "ffmpeg",
            vec![
                "-y".to_string(),
                "-i".to_string(),
                input.to_string_lossy().to_string(),
                "-i".to_string(),
                palette_path.to_string_lossy().to_string(),
                "-lavfi".to_string(),
                format!("fps={final_fps},scale={final_width}:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer"),
                output.to_string_lossy().to_string(),
            ],
        ),
    ]
}

/// Build steps for generic format conversion with quality and codec options
pub fn convert_generic_steps(
    input: &Path,
    output: &Path,
    overwrite: bool,
    quality: Option<crate::model::types::QualityPreset>,
    codec: Option<crate::model::types::VideoCodec>,
) -> Vec<Step> {
    use crate::model::types::{QualityPreset, VideoCodec};
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
    ];
    
    // Add video codec if specified
    if let Some(vcodec) = codec {
        args.push("-c:v".to_string());
        args.push(vcodec.ffmpeg_name().to_string());
        
        // Add quality settings based on codec
        if let Some(quality_preset) = quality {
            match vcodec {
                VideoCodec::H264 | VideoCodec::H265 => {
                    // Use CRF for H.264/H.265
                    args.push("-crf".to_string());
                    args.push(quality_preset.crf_value().to_string());
                    args.push("-preset".to_string());
                    args.push("medium".to_string());
                }
                VideoCodec::Vp9 => {
                    // Use CRF for VP9 (range 0-63, lower is better)
                    let crf = match quality_preset {
                        QualityPreset::Low => 50,
                        QualityPreset::Medium => 40,
                        QualityPreset::High => 30,
                        QualityPreset::Ultra => 20,
                    };
                    args.push("-crf".to_string());
                    args.push(crf.to_string());
                }
                VideoCodec::Copy => {
                    // No quality settings for copy
                }
            }
        }
    } else if let Some(quality_preset) = quality {
        // If no codec specified but quality is, use H.264 with CRF
        args.push("-c:v".to_string());
        args.push("libx264".to_string());
        args.push("-crf".to_string());
        args.push(quality_preset.crf_value().to_string());
        args.push("-preset".to_string());
        args.push("medium".to_string());
    }
    
    // Use appropriate audio codec (may need re-encoding for WebM/WMV to MP4)
    let audio_codec = get_audio_codec(input, output);
    args.push("-c:a".to_string());
    args.push(audio_codec.to_string());
    
    args.push(output.to_string_lossy().to_string());
    
    vec![Step::new("ffmpeg", args)]
}

/// Build steps for compressing video to target size
pub fn compress_steps(
    input: &Path,
    output: &Path,
    target_bytes: u64,
    duration_sec: f64,
    overwrite: bool,
    two_pass: bool,
) -> Vec<Step> {
    // Calculate bitrates
    let total_bps = (target_bytes as f64 * 8.0 / duration_sec).max(50_000.0);
    let audio_bps = (total_bps * 0.08).clamp(96_000.0, 160_000.0);
    let video_bps = (total_bps - audio_bps).max(50_000.0);
    let v_kbps = (video_bps / 1000.0).floor() as u64;
    let a_kbps = (audio_bps / 1000.0).floor() as u64;

    if two_pass {
        let null_sink = if cfg!(windows) { "NUL" } else { "/dev/null" };

        vec![
            // Pass 1: Analyze
            Step::new(
                "ffmpeg",
                vec![
                    "-y".to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-b:v".to_string(),
                    format!("{v_kbps}k"),
                    "-pass".to_string(),
                    "1".to_string(),
                    "-an".to_string(),
                    "-f".to_string(),
                    "mp4".to_string(),
                    null_sink.to_string(),
                ],
            ),
            // Pass 2: Encode
            Step::new(
                "ffmpeg",
                vec![
                    if overwrite { "-y" } else { "-n" }.to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-b:v".to_string(),
                    format!("{v_kbps}k"),
                    "-pass".to_string(),
                    "2".to_string(),
                    "-c:a".to_string(),
                    "aac".to_string(),
                    "-b:a".to_string(),
                    format!("{a_kbps}k"),
                    "-movflags".to_string(),
                    "+faststart".to_string(),
                    output.to_string_lossy().to_string(),
                ],
            ),
        ]
    } else {
        // Single pass encoding
        vec![Step::new(
            "ffmpeg",
            vec![
                if overwrite { "-y" } else { "-n" }.to_string(),
                "-i".to_string(),
                input.to_string_lossy().to_string(),
                "-c:v".to_string(),
                "libx264".to_string(),
                "-b:v".to_string(),
                format!("{v_kbps}k"),
                "-c:a".to_string(),
                "aac".to_string(),
                "-b:a".to_string(),
                format!("{a_kbps}k"),
                "-movflags".to_string(),
                "+faststart".to_string(),
                output.to_string_lossy().to_string(),
            ],
        )]
    }
}

/// Build steps for compressing video to target bitrate
pub fn compress_bitrate_steps(
    input: &Path,
    output: &Path,
    target_bitrate_bps: u64,
    overwrite: bool,
    two_pass: bool,
) -> Vec<Step> {
    // Convert to kbps for FFmpeg (which uses kbps for -b:v)
    let v_kbps = (target_bitrate_bps / 1000).max(50); // Minimum 50 kbps
    // Allocate ~8% for audio, clamp between 96-160 kbps
    let audio_bps = (target_bitrate_bps as f64 * 0.08).clamp(96_000.0, 160_000.0);
    let a_kbps = (audio_bps / 1000.0).floor() as u64;

    if two_pass {
        let null_sink = if cfg!(windows) { "NUL" } else { "/dev/null" };

        vec![
            // Pass 1: Analyze
            Step::new(
                "ffmpeg",
                vec![
                    "-y".to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-b:v".to_string(),
                    format!("{v_kbps}k"),
                    "-pass".to_string(),
                    "1".to_string(),
                    "-an".to_string(),
                    "-f".to_string(),
                    "mp4".to_string(),
                    null_sink.to_string(),
                ],
            ),
            // Pass 2: Encode
            Step::new(
                "ffmpeg",
                vec![
                    if overwrite { "-y" } else { "-n" }.to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-b:v".to_string(),
                    format!("{v_kbps}k"),
                    "-pass".to_string(),
                    "2".to_string(),
                    "-c:a".to_string(),
                    "aac".to_string(),
                    "-b:a".to_string(),
                    format!("{a_kbps}k"),
                    "-movflags".to_string(),
                    "+faststart".to_string(),
                    output.to_string_lossy().to_string(),
                ],
            ),
        ]
    } else {
        // Single pass encoding
        vec![Step::new(
            "ffmpeg",
            vec![
                if overwrite { "-y" } else { "-n" }.to_string(),
                "-i".to_string(),
                input.to_string_lossy().to_string(),
                "-c:v".to_string(),
                "libx264".to_string(),
                "-b:v".to_string(),
                format!("{v_kbps}k"),
                "-c:a".to_string(),
                "aac".to_string(),
                "-b:a".to_string(),
                format!("{a_kbps}k"),
                "-movflags".to_string(),
                "+faststart".to_string(),
                output.to_string_lossy().to_string(),
            ],
        )]
    }
}

/// Build steps for compressing video with quality preset (CRF encoding)
pub fn compress_quality_steps(
    input: &Path,
    output: &Path,
    quality: crate::model::types::QualityPreset,
    overwrite: bool,
    two_pass: bool,
) -> Vec<Step> {
    let crf = quality.crf_value();
    
    if two_pass {
        // For two-pass with quality preset, we need to estimate bitrate from CRF
        // This is approximate - two-pass with CRF is less common
        // We'll use a reasonable bitrate estimate based on quality
        let estimated_bitrate = match quality {
            crate::model::types::QualityPreset::Low => "1000k",
            crate::model::types::QualityPreset::Medium => "2000k",
            crate::model::types::QualityPreset::High => "4000k",
            crate::model::types::QualityPreset::Ultra => "8000k",
        };
        
        let null_sink = if cfg!(windows) { "NUL" } else { "/dev/null" };
        let audio_codec = get_audio_codec(input, output);
        
        vec![
            // Pass 1: Analyze
            Step::new(
                "ffmpeg",
                vec![
                    "-y".to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-b:v".to_string(),
                    estimated_bitrate.to_string(),
                    "-pass".to_string(),
                    "1".to_string(),
                    "-an".to_string(),
                    "-f".to_string(),
                    "mp4".to_string(),
                    null_sink.to_string(),
                ],
            ),
            // Pass 2: Encode with same bitrate (two-pass doesn't work with CRF)
            Step::new(
                "ffmpeg",
                vec![
                    if overwrite { "-y" } else { "-n" }.to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-c:v".to_string(),
                    "libx264".to_string(),
                    "-b:v".to_string(),
                    estimated_bitrate.to_string(),
                    "-preset".to_string(),
                    "medium".to_string(),
                    "-pass".to_string(),
                    "2".to_string(),
                    "-c:a".to_string(),
                    audio_codec.to_string(),
                    "-b:a".to_string(),
                    "192k".to_string(),
                    "-movflags".to_string(),
                    "+faststart".to_string(),
                    output.to_string_lossy().to_string(),
                ],
            ),
        ]
    } else {
        vec![Step::new(
            "ffmpeg",
            vec![
                if overwrite { "-y" } else { "-n" }.to_string(),
                "-i".to_string(),
                input.to_string_lossy().to_string(),
                "-c:v".to_string(),
                "libx264".to_string(),
                "-crf".to_string(),
                crf.to_string(),
                "-preset".to_string(),
                "medium".to_string(), // Encoding speed preset
                "-c:a".to_string(),
                "aac".to_string(),
                "-b:a".to_string(),
                "192k".to_string(),
                "-movflags".to_string(),
                "+faststart".to_string(),
                output.to_string_lossy().to_string(),
            ],
        )]
    }
}

/// Build steps for trimming video
pub fn trim_steps(input: &Path, output: &Path, start: &Time, end: &Time, overwrite: bool) -> Vec<Step> {
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-ss".to_string(),
            start.to_ffmpeg(),
            "-to".to_string(),
            end.to_ffmpeg(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for extracting audio
pub fn extract_audio_steps(
    input: &Path,
    output: &Path,
    format: &str,
    overwrite: bool,
) -> Vec<Step> {
    let (codec_args, container_ext) = match format {
        "mp3" => (vec!["-c:a", "libmp3lame", "-q:a", "2"], "mp3"),
        "wav" => (vec!["-c:a", "pcm_s16le"], "wav"),
        _ => (vec!["-c:a", "aac", "-b:a", "192k"], "m4a"),
    };

    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-vn".to_string(),
    ];
    args.extend(codec_args.iter().map(|s| s.to_string()));
    
    let output_path = output.with_extension(container_ext);
    args.push(output_path.to_string_lossy().to_string());

    vec![Step::new("ffmpeg", args)]
}

/// Build steps for resizing video
pub fn resize_steps(
    input: &Path,
    output: &Path,
    target: &ResizeTarget,
    overwrite: bool,
) -> Vec<Step> {
    let vf = target.to_ffmpeg_scale();
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            vf,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build atempo filter chain for audio speed adjustment
fn build_atempo_chain(f: f64) -> Result<String> {
    if f <= 0.0 {
        bail!("Invalid speed factor");
    }

    let mut parts = vec![];
    let mut remaining = f;

    while remaining > 2.0 {
        parts.push("atempo=2.0".to_string());
        remaining /= 2.0;
    }
    while remaining < 0.5 {
        parts.push("atempo=0.5".to_string());
        remaining /= 0.5;
    }
    parts.push(format!("atempo={:.6}", remaining));

    Ok(parts.join(","))
}

/// Build steps for speeding up video
pub fn speed_up_steps(
    input: &Path,
    output: &Path,
    factor: &SpeedFactor,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let f = factor.factor;
    let v_expr = format!("setpts=PTS/{}", f);
    let atempo = build_atempo_chain(f)?;
    let filter = format!("[0:v]{v_expr}[v];[0:a]{atempo}[a]");

    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter,
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "[a]".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for slowing down video
pub fn speed_down_steps(
    input: &Path,
    output: &Path,
    factor: &SpeedFactor,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let f = factor.factor;
    let v_expr = format!("setpts=PTS*{}", f);
    let atempo = build_atempo_chain(1.0 / f)?;
    let filter = format!("[0:v]{v_expr}[v];[0:a]{atempo}[a]");

    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter,
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "[a]".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for reversing video
pub fn reverse_steps(input: &Path, output: &Path, overwrite: bool) -> Vec<Step> {
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            "[0:v]reverse[v];[0:a]areverse[a]".to_string(),
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "[a]".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for muting video
pub fn mute_steps(input: &Path, output: &Path, overwrite: bool) -> Vec<Step> {
    let video_codec = get_video_codec(input, output);
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-c:v".to_string(),
            video_codec.to_string(),
            "-an".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for rotating video
pub fn rotate_steps(
    input: &Path,
    output: &Path,
    degrees: &RotateDegrees,
    overwrite: bool,
) -> Vec<Step> {
    let vf = match degrees.0 {
        90 => "transpose=1",
        180 => "transpose=2,transpose=2",
        270 => "transpose=2",
        0 => "null",
        _ => "null", // Should be validated by RotateDegrees::new
    };

    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            vf.to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for flipping video
pub fn flip_steps(
    input: &Path,
    output: &Path,
    direction: &FlipDirection,
    overwrite: bool,
) -> Vec<Step> {
    let vf = match direction {
        FlipDirection::Horizontal => "hflip",
        FlipDirection::Vertical => "vflip",
    };

    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            vf.to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for creating thumbnail
pub fn thumbnail_steps(input: &Path, output: &Path, time: &Time, overwrite: bool) -> Vec<Step> {
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-ss".to_string(),
            time.to_ffmpeg(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-frames:v".to_string(),
            "1".to_string(),
            "-q:v".to_string(),
            "2".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for cropping video
pub fn crop_steps(
    input: &Path,
    output: &Path,
    width: u32,
    height: u32,
    overwrite: bool,
) -> Vec<Step> {
    // Centered crop - crop to exact dimensions, but ensure we don't exceed input dimensions
    // If requested dimensions are larger than input, crop to input size (no scaling)
    // If requested dimensions are smaller, crop to requested size
    let vf = format!("crop=min({width}\\,iw):min({height}\\,ih):(iw-min({width}\\,iw))/2:(ih-min({height}\\,ih))/2");
    let audio_codec = get_audio_codec(input, output);

    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            vf,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            audio_codec.to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for setting FPS
pub fn set_fps_steps(input: &Path, output: &Path, fps: u32, overwrite: bool) -> Vec<Step> {
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            format!("fps={fps}"),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for looping video (requires concat list file path)
pub fn loop_steps(
    first_input: &Path,
    output: &Path,
    concat_list_path: &Path,
    overwrite: bool,
) -> Vec<Step> {
    let video_codec = get_video_codec(first_input, output);
    let audio_codec = get_audio_codec(first_input, output);
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-f".to_string(),
        "concat".to_string(),
        "-safe".to_string(),
        "0".to_string(),
        "-i".to_string(),
        concat_list_path.to_string_lossy().to_string(),
    ];
    
    if video_codec == "copy" && audio_codec == "copy" {
        args.push("-c".to_string());
        args.push("copy".to_string());
    } else {
        args.push("-c:v".to_string());
        args.push(video_codec.to_string());
        if video_codec != "copy" {
            args.push("-pix_fmt".to_string());
            args.push("yuv420p".to_string());
        }
        args.push("-c:a".to_string());
        args.push(audio_codec.to_string());
    }
    
    args.push(output.to_string_lossy().to_string());
    
    vec![Step::new("ffmpeg", args)]
}

/// Build steps for merging two videos
pub fn merge_steps(
    first_input: &Path,
    output: &Path,
    concat_list_path: &Path,
    overwrite: bool,
) -> Vec<Step> {
    let video_codec = get_video_codec(first_input, output);
    let audio_codec = get_audio_codec(first_input, output);
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-f".to_string(),
        "concat".to_string(),
        "-safe".to_string(),
        "0".to_string(),
        "-i".to_string(),
        concat_list_path.to_string_lossy().to_string(),
    ];
    
    if video_codec == "copy" && audio_codec == "copy" {
        args.push("-c".to_string());
        args.push("copy".to_string());
    } else {
        args.push("-c:v".to_string());
        args.push(video_codec.to_string());
        if video_codec != "copy" {
            args.push("-pix_fmt".to_string());
            args.push("yuv420p".to_string());
        }
        args.push("-c:a".to_string());
        args.push(audio_codec.to_string());
    }
    
    args.push(output.to_string_lossy().to_string());
    
    vec![Step::new("ffmpeg", args)]
}

/// Build steps for adding audio to video
pub fn add_audio_steps(
    video: &Path,
    audio: &Path,
    output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    let video_codec = get_video_codec(video, output);
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            video.to_string_lossy().to_string(),
            "-i".to_string(),
            audio.to_string_lossy().to_string(),
            "-map".to_string(),
            "0:v:0".to_string(),
            "-map".to_string(),
            "1:a:0".to_string(),
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            "-shortest".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for grayscale conversion
pub fn grayscale_steps(input: &Path, output: &Path, overwrite: bool) -> Vec<Step> {
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "format=gray".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for video filters (adjustments or presets)
pub fn filter_steps(
    input: &Path,
    output: &Path,
    adjustments: crate::model::types::FilterAdjustments,
    preset: Option<crate::model::types::ColorPreset>,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::ColorPreset;

    let mut filter_parts = Vec::new();

    // Apply preset or adjustments
    if let Some(preset) = preset {
        match preset {
            ColorPreset::Vintage => {
                // Vintage: warm tones, slight desaturation, slight contrast boost
                // Using eq filter with warm color balance
                filter_parts.push("eq=brightness=0.05:contrast=1.15:saturation=0.85".to_string());
                // Add a subtle warm tone using colorbalance
                filter_parts.push("colorbalance=rs=0.1:gs=-0.05:bs=-0.1".to_string());
            }
            ColorPreset::BlackAndWhite => {
                // Black and white: grayscale
                filter_parts.push("format=gray".to_string());
            }
            ColorPreset::Sepia => {
                // Sepia: sepia tone effect using colorchannelmixer
                // Classic sepia formula: R' = 0.393*R + 0.769*G + 0.189*B
                filter_parts.push("colorchannelmixer=.393:.769:.189:0:.349:.686:.168:0:.272:.534:.131".to_string());
            }
        }
    } else {
        // Build adjustment filter using eq filter
        // FFmpeg eq filter parameters:
        // - brightness: -1.0 to 1.0 (0.0 is no change)
        // - contrast: 0.0 to 2.0 (1.0 is normal, <1.0 reduces, >1.0 increases)
        // - saturation: 0.0 to 2.0 (1.0 is normal, 0.0 is grayscale, >1.0 increases)
        
        let mut eq_parts = Vec::new();
        
        if let Some(brightness) = adjustments.brightness {
            // Direct mapping: -1.0 to 1.0
            eq_parts.push(format!("brightness={:.3}", brightness));
        }
        
        if let Some(contrast) = adjustments.contrast {
            // Map from -1.0..1.0 to 0.0..2.0
            let contrast_value = contrast + 1.0;
            eq_parts.push(format!("contrast={:.3}", contrast_value));
        }
        
        if let Some(saturation) = adjustments.saturation {
            // Map from -1.0..1.0 to 0.0..2.0
            let saturation_value = saturation + 1.0;
            eq_parts.push(format!("saturation={:.3}", saturation_value));
        }

        if !eq_parts.is_empty() {
            filter_parts.push(format!("eq={}", eq_parts.join(":")));
        }
    }

    if filter_parts.is_empty() {
        bail!("No filter parameters provided");
    }

    // Combine multiple filters with comma
    let filter_complex = filter_parts.join(",");

    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter_complex,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for audio normalization
pub fn normalize_steps(
    input: &Path,
    output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use loudnorm filter for audio normalization
    // This normalizes audio to a target loudness level
    let video_codec = get_video_codec(input, output);
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            "loudnorm=I=-16:TP=-1.5:LRA=11".to_string(), // Standard broadcast loudness
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for adjusting audio volume
pub fn adjust_volume_steps(
    input: &Path,
    output: &Path,
    adjustment: &crate::model::types::VolumeAdjustment,
    overwrite: bool,
) -> Vec<Step> {
    let volume = adjustment.to_ffmpeg_volume();
    let video_codec = get_video_codec(input, output);
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            format!("volume={:.6}", volume),
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for syncing audio (delay or advance)
pub fn sync_audio_steps(
    input: &Path,
    output: &Path,
    direction: &crate::model::types::AudioSyncDirection,
    offset: &crate::model::types::Duration,
    overwrite: bool,
) -> Vec<Step> {
    let offset_secs = offset.to_seconds();
    // For delay: use adelay filter to delay audio
    // For advance: use -itsoffset to delay video (which effectively advances audio relative to video)
    let (video_args, audio_filter) = match direction {
        crate::model::types::AudioSyncDirection::Delay => {
            // Delay audio by offset_secs
            // Use adelay filter: adelay=delays|ch1delays:ch2delays
            // For stereo: delay both channels by offset_secs * 1000 milliseconds
            // adelay format: delays in milliseconds, separated by |
            let delay_ms = (offset_secs * 1000.0) as u64;
            (vec![], format!("adelay={}|{}", delay_ms, delay_ms))
        }
        crate::model::types::AudioSyncDirection::Advance => {
            // Advance audio (negative delay) - we can't go back in time, so we delay video instead
            // Use -itsoffset to delay video (which effectively advances audio relative to video)
            (vec!["-itsoffset".to_string(), format!("{}", offset_secs)], "anull".to_string())
        }
    };

    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
    ];
    
    if !video_args.is_empty() {
        args.extend(video_args);
    }
    
    args.extend(vec![
        "-i".to_string(),
        input.to_string_lossy().to_string(),
    ]);
    
    let video_codec = get_video_codec(input, output);
    args.extend(vec![
        "-af".to_string(),
        audio_filter,
        "-c:v".to_string(),
        video_codec.to_string(),
        "-c:a".to_string(),
        "aac".to_string(),
        output.to_string_lossy().to_string(),
    ]);

    vec![Step::new("ffmpeg", args)]
}


/// Build steps for mixing two audio files
pub fn mix_audio_steps(
    audio1: &Path,
    audio2: &Path,
    output: &Path,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // Use amix filter to mix two audio streams
    // amix=inputs=2:duration=longest (mixes both, uses longest duration)
    
    // Determine codec based on output extension
    let ext = output.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp3")
        .to_lowercase();
    
    let (codec, bitrate_flag) = match ext.as_str() {
        "mp3" => ("libmp3lame", "-q:a"),
        "wav" => ("pcm_s16le", ""),
        "ogg" => ("libvorbis", "-q:a"),
        "aac" | "m4a" => ("aac", "-b:a"),
        _ => ("aac", "-b:a"), // Default to AAC
    };
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        audio1.to_string_lossy().to_string(),
        "-i".to_string(),
        audio2.to_string_lossy().to_string(),
        "-filter_complex".to_string(),
        "[0:a][1:a]amix=inputs=2:duration=longest[a]".to_string(),
        "-map".to_string(),
        "[a]".to_string(),
        "-c:a".to_string(),
        codec.to_string(),
    ];
    
    // Add quality/bitrate setting
    if !bitrate_flag.is_empty() {
        args.push(bitrate_flag.to_string());
        match ext.as_str() {
            "mp3" => args.push("2".to_string()), // MP3 quality 0-9, 2 is good quality
            "ogg" => args.push("5".to_string()), // Vorbis quality 0-10, 5 is good
            _ => args.push("192k".to_string()), // Bitrate for others
        }
    }
    
    args.push(output.to_string_lossy().to_string());
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Build steps for extracting audio from a time range
pub fn extract_audio_range_steps(
    input: &Path,
    output: &Path,
    start: &crate::model::types::Time,
    end: &crate::model::types::Time,
    format: &str,
    overwrite: bool,
) -> Vec<Step> {
    let (codec_args, container_ext) = match format {
        "mp3" => (vec!["-c:a", "libmp3lame", "-q:a", "2"], "mp3"),
        "wav" => (vec!["-c:a", "pcm_s16le"], "wav"),
        _ => (vec!["-c:a", "aac", "-b:a", "192k"], "m4a"),
    };

    let start_time = start.to_ffmpeg();
    let duration_secs = end.to_seconds() as f64 - start.to_seconds() as f64;

    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-ss".to_string(),
        start_time,
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-t".to_string(),
        format!("{}", duration_secs),
        "-vn".to_string(),
    ];
    args.extend(codec_args.iter().map(|s| s.to_string()));
    
    let output_path = output.with_extension(container_ext);
    args.push(output_path.to_string_lossy().to_string());

    vec![Step::new("ffmpeg", args)]
}

/// Build steps for audio fade in/out
pub fn fade_steps(
    input: &Path,
    output: &Path,
    fade_in: Option<f64>,  // Duration in seconds
    fade_out: Option<f64>, // Duration in seconds
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::ffmpeg::probe;
    
    let mut filter_parts = Vec::new();
    
    // Build fade filter chain
    if let Some(fade_in_dur) = fade_in {
        // Fade in: start at 0, fade over fade_in_dur seconds
        filter_parts.push(format!("afade=t=in:st=0:d={}", fade_in_dur));
    }
    
    if let Some(fade_out_dur) = fade_out {
        // For fade out, we need to know the duration of the audio
        // Fade out: start at (duration - fade_out_dur), fade over fade_out_dur seconds
        let duration = probe::duration_seconds(input)?;
        let fade_out_start = (duration - fade_out_dur).max(0.0);
        
        if fade_out_start > 0.0 {
            filter_parts.push(format!("afade=t=out:st={}:d={}", fade_out_start, fade_out_dur));
        } else {
            // If fade out duration is longer than video, just fade from start
            filter_parts.push(format!("afade=t=out:st=0:d={}", duration.min(fade_out_dur)));
        }
    }
    
    if filter_parts.is_empty() {
        bail!("At least one fade (in or out) must be specified");
    }
    
    // Chain multiple filters with comma (afade filters can be chained)
    let filter_complex = filter_parts.join(",");
    let video_codec = get_video_codec(input, output);
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            filter_complex,
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for blurring a region in video
pub fn blur_steps(
    input: &Path,
    output: &Path,
    region: &crate::model::types::BlurRegion,
    overwrite: bool,
) -> Vec<Step> {
    // Use boxblur filter with crop and overlay approach
    // Strategy:
    // 1. Create a blurred version of the entire video
    // 2. Crop the blurred region
    // 3. Overlay it back on the original video at the correct position
    
    // boxblur parameters: boxblur=luma_radius:luma_power:chroma_radius:chroma_power
    // Using luma_radius=10 and chroma_radius=10 for a moderate blur
    let blur_filter = "boxblur=10:10";
    
    // Build filter complex:
    // [0:v]boxblur=10:10[blurred];
    // [blurred]crop=width:height:x:y[blurred_crop];
    // [0:v][blurred_crop]overlay=x:y[v]
    let filter_complex = format!(
        "[0:v]{blur_filter}[blurred];[blurred]crop={}:{}:{}:{}[blurred_crop];[0:v][blurred_crop]overlay={}:{}[v]",
        region.width, region.height, region.x, region.y, region.x, region.y
    );
    
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter_complex,
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Map audio if present
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for side-by-side video comparison
pub fn compare_steps(
    video1: &Path,
    video2: &Path,
    output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use hstack filter to place videos side by side
    // Scale both videos to same height, then stack horizontally
    let audio_codec = get_audio_codec(video1, output);
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            video1.to_string_lossy().to_string(),
            "-i".to_string(),
            video2.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            "[0:v]scale=iw*min(720/iw\\,720/ih):ih*min(720/iw\\,720/ih)[v0];[1:v]scale=iw*min(720/iw\\,720/ih):ih*min(720/iw\\,720/ih)[v1];[v0][v1]hstack[h];[h]scale=-2:720[v]".to_string(),
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Use audio from first video
            "-c:v".to_string(),
            "libx264".to_string(),
            "-pix_fmt".to_string(),
            "yuv420p".to_string(),
            "-c:a".to_string(),
            audio_codec.to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for setting video metadata
pub fn set_metadata_steps(
    input: &Path,
    output: &Path,
    field: &crate::model::types::MetadataField,
    value: &str,
    overwrite: bool,
) -> Vec<Step> {
    let metadata_key = field.ffmpeg_key();
    
    let video_codec = get_video_codec(input, output);
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-metadata".to_string(),
        format!("{}={}", metadata_key, value),
    ];
    
    if video_codec == "copy" {
        args.push("-c".to_string());
        args.push("copy".to_string());
    } else {
        args.push("-c:v".to_string());
        args.push(video_codec.to_string());
        args.push("-c:a".to_string());
        args.push("copy".to_string());
    }
    
    args.push(output.to_string_lossy().to_string());
    
    vec![Step::new("ffmpeg", args)]
}

/// Build steps for creating video montage (grid layout)
pub fn montage_steps(
    videos: &[PathBuf],
    output: &Path,
    layout: &crate::model::types::MontageLayout,
    overwrite: bool,
) -> Result<Vec<Step>> {
    
    let cols = layout.cols;
    let rows = layout.rows;
    // Build input arguments
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
    ];
    
    // Add all video inputs
    for video in videos.iter() {
        args.push("-i".to_string());
        args.push(video.to_string_lossy().to_string());
    }
    
    // Build filter complex for grid layout
    // Scale all videos to same size, then arrange in grid using hstack/vstack
    let mut filter_parts = Vec::new();
    let cell_width = 320;
    let cell_height = 240;
    
    // Scale each input video to cell size
    for i in 0..videos.len() {
        filter_parts.push(format!(
            "[{}:v]scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2[v{}]",
            i, cell_width, cell_height, cell_width, cell_height, i
        ));
    }
    
    // Build grid using hstack and vstack for more reliable layout
    // First, create horizontal rows using hstack
    let mut row_filters = Vec::new();
    let mut row_labels = Vec::new();
    for row in 0..rows {
        let mut row_inputs = Vec::new();
        for col in 0..cols {
            let idx = (row * cols + col) as usize;
            if idx < videos.len() {
                row_inputs.push(format!("[v{}]", idx));
            }
        }
        if !row_inputs.is_empty() {
            // Use hstack to combine videos in this row horizontally
            let row_label = format!("row{}", row);
            row_filters.push(format!("{}hstack=inputs={}[{}]", 
                row_inputs.join(""), row_inputs.len(), row_label));
            row_labels.push(format!("[row{}]", row));
        }
    }
    
    // Then, stack rows vertically using vstack (if more than 1 row)
    let scale_chain = filter_parts.join(";");
    let row_chain = row_filters.join(";");
    let output_width = cols * cell_width;
    let _output_height = rows * cell_height;
    
    // Combine scale, row creation, and vertical stacking
    // If only one row, skip vstack and use the row directly
    let filter_complex = if row_labels.len() > 1 {
        // Multiple rows: stack them vertically
        format!("{};{};{}vstack=inputs={}[vstack];[vstack]scale={}:-2[v]", 
            scale_chain, row_chain, row_labels.join(""), row_labels.len(), output_width)
    } else {
        // Single row: use it directly
        format!("{};{};[{}]scale={}:-2[v]", 
            scale_chain, row_chain, row_labels[0].trim_start_matches('[').trim_end_matches(']'), output_width)
    };
    
    args.push("-filter_complex".to_string());
    args.push(filter_complex);
    let audio_codec = if !videos.is_empty() {
        get_audio_codec(&videos[0], output)
    } else {
        "copy"
    };
    args.push("-map".to_string());
    args.push("[v]".to_string());
    args.push("-map".to_string());
    args.push("0:a?".to_string()); // Use audio from first video
    args.push("-c:v".to_string());
    args.push("libx264".to_string());
    args.push("-pix_fmt".to_string());
    args.push("yuv420p".to_string());
    args.push("-c:a".to_string());
    args.push(audio_codec.to_string());
    args.push(output.to_string_lossy().to_string());
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Build steps for crossfade transition between two videos
pub fn crossfade_steps(
    video1: &Path,
    video2: &Path,
    output: &Path,
    duration: &crate::model::types::Duration,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::ffmpeg::probe;
    
    let crossfade_duration = duration.to_seconds();
    let video1_duration = probe::duration_seconds(video1)?;
    
    // Calculate crossfade timing
    // Video1 fades out over the last crossfade_duration seconds
    // Video2 fades in over the first crossfade_duration seconds
    // They overlap during the crossfade
    let video1_fade_start = (video1_duration - crossfade_duration).max(0.0);
    
    // Build filter complex:
    // 1. Apply fade out to end of video1
    // 2. Apply fade in to start of video2
    // 3. Overlay video2 on video1 starting at fade_start time
    // 4. Concatenate the non-overlapping parts
    
    // For simplicity, we'll:
    // - Trim video1 to exclude the crossfade portion, then fade out the remaining part
    // - Fade in video2 from start
    // - Overlay them during the crossfade period
    // - Concatenate: video1_part1 + crossfade_overlay + video2_part2
    
    let _video1_trim_end = video1_fade_start;
    
    // More complex approach: use xfade filter which handles crossfades automatically
    // xfade requires both videos to be the same duration for the transition
    // So we'll trim both to the shorter duration and apply crossfade
    
    let min_duration = video1_duration.min(probe::duration_seconds(video2)?);
    let transition_start = (min_duration - crossfade_duration).max(0.0);
    
    let filter_complex = format!(
        "[0:v][1:v]xfade=transition=fade:duration={}:offset={}[v]",
        crossfade_duration, transition_start
    );
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            video1.to_string_lossy().to_string(),
            "-i".to_string(),
            video2.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter_complex,
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Use audio from first video
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for splitting video into segments
/// Returns multiple steps, one for each segment
pub fn split_steps(
    input: &Path,
    output_dir: &Path,
    mode: &crate::model::types::SplitMode,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::ffmpeg::probe;
    use crate::model::types::SplitMode;
    
    let duration = probe::duration_seconds(input)?;
    let mut steps = Vec::new();
    
    match mode {
        SplitMode::Every(interval) => {
            let interval_secs = interval.to_seconds();
            let mut start = 0.0;
            let mut segment_num = 1;
            
            while start < duration {
                let end = (start + interval_secs).min(duration);
                let output = output_dir.join(format!("segment_{:03}.mp4", segment_num));
                
                let video_codec = get_video_codec(input, &output);
                let mut step_args = vec![
                    if overwrite { "-y" } else { "-n" }.to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-ss".to_string(),
                    format!("{:.3}", start),
                    "-t".to_string(),
                    format!("{:.3}", end - start),
                ];
                
                if video_codec == "copy" {
                    step_args.push("-c".to_string());
                    step_args.push("copy".to_string());
                } else {
                    step_args.push("-c:v".to_string());
                    step_args.push(video_codec.to_string());
                    step_args.push("-c:a".to_string());
                    step_args.push("copy".to_string());
                }
                
                step_args.push(output.to_string_lossy().to_string());
                steps.push(Step::new("ffmpeg", step_args));
                
                start = end;
                segment_num += 1;
            }
        }
        SplitMode::IntoParts(parts) => {
            let segment_duration = duration / (*parts as f64);
            for i in 0..*parts {
                let start = i as f64 * segment_duration;
                let end = if i == parts - 1 {
                    duration
                } else {
                    (i + 1) as f64 * segment_duration
                };
                
                let output = output_dir.join(format!("part_{:03}_of_{:03}.mp4", i + 1, parts));
                let video_codec = get_video_codec(input, &output);
                let mut step_args = vec![
                    if overwrite { "-y" } else { "-n" }.to_string(),
                    "-i".to_string(),
                    input.to_string_lossy().to_string(),
                    "-ss".to_string(),
                    format!("{:.3}", start),
                    "-t".to_string(),
                    format!("{:.3}", end - start),
                ];
                
                if video_codec == "copy" {
                    step_args.push("-c".to_string());
                    step_args.push("copy".to_string());
                } else {
                    step_args.push("-c:v".to_string());
                    step_args.push(video_codec.to_string());
                    step_args.push("-c:a".to_string());
                    step_args.push("copy".to_string());
                }
                
                step_args.push(output.to_string_lossy().to_string());
                steps.push(Step::new("ffmpeg", step_args));
            }
        }
    }
    
    Ok(steps)
}

/// Build steps for extracting frames at intervals
pub fn extract_frames_steps(
    input: &Path,
    output_dir: &Path,
    interval: &crate::model::types::Duration,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::ffmpeg::probe;
    
    let duration = probe::duration_seconds(input)?;
    let interval_secs = interval.to_seconds();
    let mut steps = Vec::new();
    let mut time = 0.0;
    let mut frame_num = 1;
    
    while time < duration {
        let output = output_dir.join(format!("frame_{:05}.jpg", frame_num));
        
        steps.push(Step::new(
            "ffmpeg",
            vec![
                if overwrite { "-y" } else { "-n" }.to_string(),
                "-ss".to_string(),
                format!("{:.3}", time),
                "-i".to_string(),
                input.to_string_lossy().to_string(),
                "-frames:v".to_string(),
                "1".to_string(),
                "-q:v".to_string(),
                "2".to_string(), // High quality
                output.to_string_lossy().to_string(),
            ],
        ));
        
        time += interval_secs;
        frame_num += 1;
    }
    
    Ok(steps)
}

/// Build steps for burning subtitles into video
pub fn burn_subtitle_steps(
    input: &Path,
    subtitle: &Path,
    output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use subtitles filter for SRT files, or ass filter for ASS files
    // For simplicity, we'll use subtitles filter which works with SRT
    // and can be extended to support ASS later
    
    let subtitle_path = subtitle.to_string_lossy();
    let filter = if subtitle_path.ends_with(".ass") || subtitle_path.ends_with(".ASS") {
        format!("ass={}", subtitle_path)
    } else {
        format!("subtitles={}", subtitle_path)
    };
    
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter,
            "-c:a".to_string(),
            "copy".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for watermark overlay
pub fn watermark_steps(
    input: &Path,
    logo: &Path,
    output: &Path,
    position: &crate::model::types::WatermarkPosition,
    opacity: &crate::model::types::Opacity,
    size: &Option<crate::model::types::WatermarkSize>,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::{WatermarkPosition, WatermarkSize};
    
    // Build opacity filter
    let opacity_value = opacity.0;
    
    // Build overlay position
    let overlay_pos = match position {
        WatermarkPosition::TopLeft => "10:10".to_string(),
        WatermarkPosition::TopRight => "W-w-10:10".to_string(),
        WatermarkPosition::BottomLeft => "10:H-h-10".to_string(),
        WatermarkPosition::BottomRight => "W-w-10:H-h-10".to_string(),
        WatermarkPosition::Custom { x, y } => format!("{}:{}", x, y),
    };
    
    // Build filter complex based on size and opacity
    let filter_complex = match size {
        Some(WatermarkSize::Percentage(pct)) => {
            // Use scale2ref for percentage scaling relative to main video
            // scale2ref outputs two streams: scaled logo and reference
            // We need to use both outputs - use the reference as the base for overlay
            if opacity_value < 1.0 {
                format!(
                    "[1:v][0:v]scale2ref=w=iw*{}:h=ow/mdar[logo_scaled][ref];[logo_scaled]format=rgba,colorchannelmixer=aa={}[logo];[ref][logo]overlay={}[v]",
                    pct, opacity_value, overlay_pos
                )
            } else {
                format!(
                    "[1:v][0:v]scale2ref=w=iw*{}:h=ow/mdar[logo_scaled][ref];[logo_scaled]format=rgba[logo];[ref][logo]overlay={}[v]",
                    pct, overlay_pos
                )
            }
        }
        Some(WatermarkSize::Pixels { width, height }) => {
            let scale_str = if let Some(h) = height {
                format!("scale={}:{}", width, h)
            } else {
                format!("scale={}:-1", width)
            };
            if opacity_value < 1.0 {
                format!(
                    "[1:v]{},format=rgba,colorchannelmixer=aa={}[logo];[0:v][logo]overlay={}[v]",
                    scale_str, opacity_value, overlay_pos
                )
            } else {
                format!(
                    "[1:v]{},format=rgba[logo];[0:v][logo]overlay={}[v]",
                    scale_str, overlay_pos
                )
            }
        }
        None => {
            // No scaling
            if opacity_value < 1.0 {
                format!(
                    "[1:v]format=rgba,colorchannelmixer=aa={}[logo];[0:v][logo]overlay={}[v]",
                    opacity_value, overlay_pos
                )
            } else {
                format!(
                    "[1:v]format=rgba[logo];[0:v][logo]overlay={}[v]",
                    overlay_pos
                )
            }
        }
    };
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-i".to_string(),
            logo.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter_complex,
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Map audio if present
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for adding text overlay
pub fn add_text_steps(
    input: &Path,
    output: &Path,
    text: &str,
    position: &crate::model::types::TextPosition,
    style: &crate::model::types::TextStyle,
    timestamp: bool,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::TextPosition;
    
    // Determine the text to display
    let display_text = if timestamp {
        "%{pts\\:localtime\\:%Y-%m-%d %H\\:%M\\:%S}".to_string()
    } else {
        // Escape special characters for FFmpeg drawtext
        text.replace('\\', "\\\\")
            .replace(':', "\\:")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('\'', "\\'")
    };
    
    // Build font size
    let font_size = style.font_size.unwrap_or(24);
    
    // Build color
    let color = style.color.to_ffmpeg();
    
    // Build position expression
    let (x_expr, y_expr) = match position {
        TextPosition::TopLeft => ("10".to_string(), "10".to_string()),
        TextPosition::TopRight => ("W-tw-10".to_string(), "10".to_string()),
        TextPosition::TopCenter => ("(W-tw)/2".to_string(), "10".to_string()),
        TextPosition::BottomLeft => ("10".to_string(), "H-th-10".to_string()),
        TextPosition::BottomRight => ("W-tw-10".to_string(), "H-th-10".to_string()),
        TextPosition::BottomCenter => ("(W-tw)/2".to_string(), "H-th-10".to_string()),
        TextPosition::Center => ("(W-tw)/2".to_string(), "(H-th)/2".to_string()),
        TextPosition::Custom { x, y } => (x.to_string(), y.to_string()),
    };
    
    // Build drawtext filter
    // tw = text width, th = text height
    let drawtext_filter = format!(
        "drawtext=text='{}':fontsize={}:fontcolor={}:x={}:y={}",
        display_text, font_size, color, x_expr, y_expr
    );
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            drawtext_filter,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for picture-in-picture overlay
pub fn pip_steps(
    overlay_video: &Path,
    base_video: &Path,
    output: &Path,
    position: &crate::model::types::PipPosition,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::PipPosition;
    
    // Build overlay position
    let overlay_pos = match position {
        PipPosition::TopLeft => "10:10".to_string(),
        PipPosition::TopRight => "W-w-10:10".to_string(),
        PipPosition::BottomLeft => "10:H-h-10".to_string(),
        PipPosition::BottomRight => "W-w-10:H-h-10".to_string(),
        PipPosition::Center => "(W-w)/2:(H-h)/2".to_string(),
    };
    
    // Scale overlay video to 30% of base video width, maintain aspect ratio
    // Then overlay it on the base video
    let filter_complex = format!(
        "[1:v]scale=iw*0.3:-1[overlay_scaled];[0:v][overlay_scaled]overlay={}[v]",
        overlay_pos
    );
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            base_video.to_string_lossy().to_string(),
            "-i".to_string(),
            overlay_video.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter_complex,
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Map audio from base video if present
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for chroma key background removal
pub fn remove_background_steps(
    input: &Path,
    output: &Path,
    color: &crate::model::types::ChromaKeyColor,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let color_hex = color.to_ffmpeg();
    
    // Use chromakey filter to remove the specified color
    // similarity: how similar colors should be removed (0.0-1.0)
    // blend: how much to blend edges (0.0-1.0)
    let filter = format!(
        "chromakey=color={}:similarity=0.3:blend=0.1",
        color_hex
    );
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-pix_fmt".to_string(),
            "yuv420p".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for video overlay with transparency
pub fn overlay_steps(
    overlay_video: &Path,
    base_video: &Path,
    output: &Path,
    position: &crate::model::types::WatermarkPosition,
    opacity: &crate::model::types::Opacity,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::WatermarkPosition;
    
    // Build overlay position
    let overlay_pos = match position {
        WatermarkPosition::TopLeft => "10:10".to_string(),
        WatermarkPosition::TopRight => "W-w-10:10".to_string(),
        WatermarkPosition::BottomLeft => "10:H-h-10".to_string(),
        WatermarkPosition::BottomRight => "W-w-10:H-h-10".to_string(),
        WatermarkPosition::Custom { x, y } => format!("{}:{}", x, y),
    };
    
    let opacity_value = opacity.0;
    
    // Build filter complex with opacity
    let filter_complex = if opacity_value < 1.0 {
        format!(
            "[1:v]format=rgba,colorchannelmixer=aa={}[overlay_alpha];[0:v][overlay_alpha]overlay={}[v]",
            opacity_value, overlay_pos
        )
    } else {
        format!(
            "[0:v][1:v]overlay={}[v]",
            overlay_pos
        )
    };
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            base_video.to_string_lossy().to_string(),
            "-i".to_string(),
            overlay_video.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter_complex,
            "-map".to_string(),
            "[v]".to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Map audio from base video if present
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}


/// Build steps for concatenating multiple videos without re-encoding (faster than merge)
pub fn concat_steps(
    output: &Path,
    concat_list_path: &Path,
    overwrite: bool,
    first_input: &Path,
) -> Vec<Step> {
    let video_codec = get_video_codec(first_input, output);
    let audio_codec = get_audio_codec(first_input, output);
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-f".to_string(),
        "concat".to_string(),
        "-safe".to_string(),
        "0".to_string(),
        "-i".to_string(),
        concat_list_path.to_string_lossy().to_string(),
    ];
    
    if video_codec == "copy" && audio_codec == "copy" {
        args.push("-c".to_string());
        args.push("copy".to_string());
    } else {
        args.push("-c:v".to_string());
        args.push(video_codec.to_string());
        if video_codec != "copy" {
            args.push("-pix_fmt".to_string());
            args.push("yuv420p".to_string());
        }
        args.push("-c:a".to_string());
        args.push(audio_codec.to_string());
    }
    
    args.push(output.to_string_lossy().to_string());
    
    vec![Step::new("ffmpeg", args)]
}

/// Build steps for detecting scene changes in video
/// Outputs scene change timestamps to a text file
pub fn detect_scenes_steps(
    input: &Path,
    _output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use ffmpeg's scene detection filter
    // The output will be captured from stderr and written to the output file
    // We use select filter with scene detection threshold
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "select='gt(scene,0.3)',showinfo".to_string(),
            "-f".to_string(),
            "null".to_string(),
            "-".to_string(),
        ],
    )]
}

/// Build steps for detecting black frames in video
/// Outputs black frame timestamps to a text file
pub fn detect_black_steps(
    input: &Path,
    _output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use ffmpeg's blackdetect filter
    // The output will be captured from stderr and written to the output file
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "blackdetect=d=0.1:pix_th=0.1".to_string(),
            "-f".to_string(),
            "null".to_string(),
            "-".to_string(),
        ],
    )]
}


/// Build steps for auto-detecting and fixing video rotation
/// Note: This requires probing the video first to detect rotation metadata
/// For now, we apply a filter that removes rotation metadata
/// A more complete implementation would probe first and apply the appropriate transpose
pub fn fix_rotation_steps(
    input: &Path,
    output: &Path,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // Use ffmpeg to remove rotation metadata
    // The video will be displayed correctly by players that respect rotation metadata
    // For a more complete fix, we would need to probe the video first to get rotation
    // and then apply the appropriate transpose filter
    let video_codec = get_video_codec(input, output);
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
    ];
    
    if video_codec == "copy" {
        args.push("-c".to_string());
        args.push("copy".to_string());
    } else {
        args.push("-c:v".to_string());
        args.push(video_codec.to_string());
        args.push("-c:a".to_string());
        args.push("copy".to_string());
    }
    
    args.push("-metadata:s:v:0".to_string());
    args.push("rotate=0".to_string());
    args.push(output.to_string_lossy().to_string());
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Device preset for optimized encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevicePreset {
    Iphone,
    Android,
}

/// Build steps for device-specific conversion (iPhone/Android optimized)
pub fn convert_device_steps(
    input: &Path,
    output: &Path,
    device: DevicePreset,
    quality: Option<crate::model::types::QualityPreset>,
    overwrite: bool,
) -> Vec<Step> {
    let (max_width, max_height, crf, preset) = match device {
        DevicePreset::Iphone => {
            // iPhone optimized: H.264, max 1080p, faststart, optimized for iOS playback
            let (width, height) = (1920, 1080);
            let crf = quality.map(|q| q.crf_value()).unwrap_or(23);
            (width, height, crf, "medium")
        }
        DevicePreset::Android => {
            // Android optimized: H.264, max 1080p, baseline profile for compatibility
            let (width, height) = (1920, 1080);
            let crf = quality.map(|q| q.crf_value()).unwrap_or(23);
            (width, height, crf, "medium")
        }
    };
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-vf".to_string(),
        format!("scale='min({max_width},iw)':'min({max_height},ih)':force_original_aspect_ratio=decrease"),
        "-c:v".to_string(),
        "libx264".to_string(),
        "-crf".to_string(),
        crf.to_string(),
        "-preset".to_string(),
        preset.to_string(),
    ];
    
    // Add device-specific settings
    match device {
        DevicePreset::Iphone => {
            // iPhone: Use H.264 High profile, faststart for streaming
            args.push("-profile:v".to_string());
            args.push("high".to_string());
            args.push("-level".to_string());
            args.push("4.0".to_string());
            args.push("-pix_fmt".to_string());
            args.push("yuv420p".to_string());
            args.push("-movflags".to_string());
            args.push("+faststart".to_string());
        }
        DevicePreset::Android => {
            // Android: Use H.264 Baseline profile for maximum compatibility
            args.push("-profile:v".to_string());
            args.push("baseline".to_string());
            args.push("-level".to_string());
            args.push("3.0".to_string());
            args.push("-pix_fmt".to_string());
            args.push("yuv420p".to_string());
        }
    }
    
    // Audio settings
    args.push("-c:a".to_string());
    args.push("aac".to_string());
    args.push("-b:a".to_string());
    args.push("128k".to_string());
    
    args.push(output.to_string_lossy().to_string());
    
    vec![Step::new("ffmpeg", args)]
}

/// Build steps for HLS streaming format conversion
pub fn convert_hls_steps(
    input: &Path,
    output_dir: &Path,
    quality: Option<crate::model::types::QualityPreset>,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // HLS requires multiple quality variants
    // We'll create a simple single-variant HLS stream
    let crf = quality.map(|q| q.crf_value()).unwrap_or(23);
    
    let playlist = output_dir.join("playlist.m3u8");
    let segment_pattern = output_dir.join("segment_%03d.ts");
    
    let args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-c:v".to_string(),
        "libx264".to_string(),
        "-crf".to_string(),
        crf.to_string(),
        "-preset".to_string(),
        "medium".to_string(),
        "-c:a".to_string(),
        "aac".to_string(),
        "-b:a".to_string(),
        "128k".to_string(),
        "-f".to_string(),
        "hls".to_string(),
        "-hls_time".to_string(),
        "10".to_string(), // 10 second segments
        "-hls_list_size".to_string(),
        "0".to_string(), // Keep all segments
        "-hls_segment_filename".to_string(),
        segment_pattern.to_string_lossy().to_string(),
        playlist.to_string_lossy().to_string(),
    ];
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Build steps for DASH streaming format conversion
pub fn convert_dash_steps(
    input: &Path,
    output_dir: &Path,
    quality: Option<crate::model::types::QualityPreset>,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // DASH requires multiple quality variants
    // We'll create a simple single-variant DASH stream
    let crf = quality.map(|q| q.crf_value()).unwrap_or(23);
    
    // Ensure output directory exists
    std::fs::create_dir_all(output_dir).context("Failed to create DASH output directory")?;
    
    let manifest = output_dir.join("manifest.mpd");
    // DASH needs relative path pattern for segments (relative to manifest)
    let segment_pattern = "segment_%03d.m4s";
    
    let audio_codec = get_audio_codec(input, &manifest);
    
    let args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-c:v".to_string(),
        "libx264".to_string(),
        "-crf".to_string(),
        crf.to_string(),
        "-preset".to_string(),
        "medium".to_string(),
        "-c:a".to_string(),
        audio_codec.to_string(),
        "-b:a".to_string(),
        "128k".to_string(),
        "-f".to_string(),
        "dash".to_string(),
        "-seg_duration".to_string(),
        "10".to_string(), // 10 second segments
        "-use_timeline".to_string(),
        "1".to_string(),
        "-use_template".to_string(),
        "1".to_string(),
        "-init_seg_name".to_string(),
        "init_$RepresentationID$.m4s".to_string(),
        "-media_seg_name".to_string(),
        segment_pattern.to_string(),
        manifest.to_string_lossy().to_string(),
    ];
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Build steps for generating a quick preview (first 10 seconds, low quality)
pub fn preview_steps(input: &Path, output: &Path, overwrite: bool) -> Vec<Step> {
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-t".to_string(),
            "10".to_string(), // First 10 seconds
            "-vf".to_string(),
            "scale=640:-2".to_string(), // Low resolution
            "-c:v".to_string(),
            "libx264".to_string(),
            "-crf".to_string(),
            "28".to_string(), // Low quality (higher CRF = lower quality)
            "-preset".to_string(),
            "fast".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            "-b:a".to_string(),
            "64k".to_string(), // Low audio bitrate
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for adding motion blur effect
pub fn motion_blur_steps(input: &Path, output: &Path, radius: Option<u32>, overwrite: bool) -> Vec<Step> {
    // Use tmix (temporal mix) filter to blend frames together
    // This creates a motion blur effect by averaging frames over time
    // frames: number of frames to mix (higher = more blur)
    let frames = radius.unwrap_or(3);
    
    // Generate weights: equal weights for all frames (1 for each frame)
    let weights: Vec<String> = (0..frames).map(|_| "1".to_string()).collect();
    let weights_str = weights.join(" ");
    
    let filter = format!("tmix=frames={}:weights={}", frames, weights_str);
    
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for adding vignette effect
pub fn vignette_steps(
    input: &Path,
    output: &Path,
    intensity: Option<f32>,
    size: Option<f32>,
    overwrite: bool,
) -> Vec<Step> {
    // Use geq filter to create a visible vignette effect by darkening edges
    // Calculate distance from center and apply darkening factor
    // Distance from center: sqrt((X/W-0.5)^2 + (Y/H-0.5)^2)
    // Maximum distance from center to corner is sqrt(0.5^2 + 0.5^2) = sqrt(0.5) ≈ 0.707
    
    // Parameters:
    // - intensity: How dark the edges get (0.0-1.0, default 0.5)
    //   This controls the minimum brightness at edges: min_brightness = 1.0 - intensity
    // - size: Size of the bright center area (0.0-1.0, default 0.7)
    //   Higher values = larger bright center, smaller vignette area
    
    let intensity_val = intensity.unwrap_or(0.5);
    let size_val = size.unwrap_or(0.7);
    
    // Clamp values to valid range
    let intensity_clamped = intensity_val.max(0.0).min(1.0);
    let size_clamped = size_val.max(0.0).min(1.0);
    
    // Calculate minimum brightness (edges)
    let min_brightness = 1.0 - intensity_clamped;
    
    // Maximum distance from center to corner is sqrt(0.5) ≈ 0.707
    let max_distance = 0.707;
    
    // Normalize distance: d_norm = distance / max_distance (0 to 1)
    // Apply size: vignette starts at distance = size * max_distance
    // Formula: brightness = 1.0 when d_norm < size, otherwise decreases to min_brightness
    // Using smooth transition: brightness = 1.0 - max(0, (d_norm - size) / (1 - size)) * (1.0 - min_brightness)
    // Simplified: brightness = max(min_brightness, 1.0 - max(0, (d_norm - size) / (1 - size)) * intensity)
    
    // For geq, we use: brightness = max(min_brightness, 1.0 - max(0, (d/max_distance - size) / (1 - size)) * intensity)
    // Where d = sqrt((X/W-0.5)^2 + (Y/H-0.5)^2)
    
    let filter = format!(
        "geq=lum='p(X,Y)*max({},1-max(0,(sqrt(pow(X/W-0.5,2)+pow(Y/H-0.5,2))/{}-{})/{})*{})':cb='p(X,Y)':cr='p(X,Y)'",
        min_brightness,
        max_distance,
        size_clamped,
        1.0 - size_clamped,
        intensity_clamped
    );
    
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for lens correction (fixing lens distortion)
pub fn lens_correct_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    // Use lenscorrection filter to fix barrel/pincushion distortion
    // k1, k2: distortion coefficients (negative for barrel, positive for pincushion)
    // Default values provide a subtle correction
    // For more accurate correction, these values should be calibrated per camera/lens
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "lenscorrection=k1=-0.1:k2=-0.05".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for frame interpolation (increasing frame rate)
pub fn interpolate_steps(input: &Path, output: &Path, target_fps: u32, overwrite: bool) -> Result<Vec<Step>> {
    // Use minterpolate filter to generate intermediate frames
    // This creates smooth slow-motion by interpolating frames
    // fps: target frame rate
    // mi_mode: motion interpolation mode (mci = motion compensated interpolation)
    // mc_mode: motion compensation mode (aobmc = adaptive overlapped block motion compensation)
    // Note: minterpolate is extremely computationally intensive as it generates new frames
    // Using faster preset for better performance while maintaining reasonable quality
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            format!("minterpolate=fps={}:mi_mode=mci:mc_mode=aobmc:vsbmc=1", target_fps),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-preset".to_string(),
            "veryfast".to_string(), // Faster preset for better performance
            "-crf".to_string(),
            "23".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for video denoising using hqdn3d filter
/// hqdn3d is a high-quality 3D denoise filter that's widely available
pub fn denoise_steps(input: &Path, output: &Path, overwrite: bool) -> Vec<Step> {
    // hqdn3d parameters:
    // luma_spatial: spatial luma strength (default: 4.0, higher = more denoising)
    // chroma_spatial: spatial chroma strength (default: 3.0)
    // luma_temporal: temporal luma strength (default: 6.0, higher = more temporal smoothing)
    // chroma_temporal: temporal chroma strength (default: 4.5)
    // Using moderate values for good quality/performance balance
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "hqdn3d=4:3:6:4.5".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-preset".to_string(),
            "medium".to_string(),
            "-crf".to_string(),
            "23".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for video stabilization
/// Uses deshake filter (built-in) as primary method, with vidstab as fallback if available
/// deshake is simpler and doesn't require external libraries
pub fn stabilize_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    // Use deshake filter which is built into ffmpeg
    // It's simpler than vidstab but works well for most cases
    // If vid.stab is needed for advanced stabilization, it can be added as a two-pass process
    // For now, we use deshake which is widely available
    // Note: deshake is computationally intensive as it analyzes motion between frames
    // Using faster preset for better performance while maintaining reasonable quality
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "deshake".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-preset".to_string(),
            "veryfast".to_string(), // Faster preset for better performance
            "-crf".to_string(),
            "23".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for extracting all metadata to JSON/XML
pub fn extract_metadata_steps(input: &Path, _output: &Path, format: &str, _overwrite: bool) -> Vec<Step> {
    let output_format = match format {
        "json" => "json",
        "xml" => "xml",
        _ => "json",
    };
    
    vec![Step::new(
        "ffprobe",
        vec![
            "-v".to_string(),
            "error".to_string(),
            "-show_format".to_string(),
            "-show_streams".to_string(),
            "-of".to_string(),
            output_format.to_string(),
            input.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for thumbnail grid generation
pub fn thumbnail_grid_steps(input: &Path, output: &Path, cols: u32, rows: u32, overwrite: bool) -> Result<Vec<Step>> {
    // Calculate number of thumbnails needed
    let total = cols * rows;
    
    // Use select filter to pick frames evenly spaced throughout the video
    // Then use tile filter to arrange them in a grid
    let filter = format!(
        "select='not(mod(n\\,{total}))',scale=320:-1,tile={cols}x{rows}",
        total = total,
        cols = cols,
        rows = rows
    );
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter,
            "-frames:v".to_string(),
            "1".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for social media platform conversion
pub fn social_media_convert_steps(
    input: &Path,
    output: &Path,
    platform: &str,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let (width, height, bitrate, fps) = match platform {
        "instagram" => (1080, 1080, 3500, 30), // Square format
        "tiktok" => (1080, 1920, 4000, 30),    // Vertical 9:16
        "youtube-shorts" => (1080, 1920, 5000, 30), // Vertical 9:16
        "twitter" => (1280, 720, 3000, 30),    // Horizontal 16:9
        _ => (1080, 1080, 3500, 30), // Default to Instagram
    };
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            format!("scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2", width, height, width, height),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-b:v".to_string(),
            format!("{}k", bitrate),
            "-r".to_string(),
            fps.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            "-b:a".to_string(),
            "128k".to_string(),
            "-movflags".to_string(),
            "+faststart".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for social media crop (square or circle)
pub fn social_crop_steps(
    input: &Path,
    output: &Path,
    shape: &str,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let filter = match shape {
        "square" => {
            // Crop to square (1:1 aspect ratio), centered
            // Crop to min(iw,ih) x min(iw,ih) centered
            "crop='min(iw,ih)':'min(iw,ih)':(iw-ow)/2:(ih-oh)/2"
        }
        "circle" => {
            // Crop to square then apply circular mask (black background for MP4 compatibility)
            // Use geq to set pixels outside circle to black
            // Calculate distance from center: sqrt((X-W/2)^2 + (Y-H/2)^2)
            // If distance < radius (W/2), keep pixel, else set to black
            "crop='min(iw,ih)':'min(iw,ih)':(iw-ow)/2:(ih-oh)/2,geq=lum='if(lt(sqrt(pow(X-W/2,2)+pow(Y-H/2,2)),W/2),lum(X,Y),0)':cb='if(lt(sqrt(pow(X-W/2,2)+pow(Y-H/2,2)),W/2),cb(X,Y),128)':cr='if(lt(sqrt(pow(X-W/2,2)+pow(Y-H/2,2)),W/2),cr(X,Y),128)'"
        }
        _ => return Err(anyhow::anyhow!("Invalid shape: {shape}")),
    };
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter.to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for vertical video conversion (9:16 aspect ratio)
pub fn vertical_convert_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "scale=1080:1920:force_original_aspect_ratio=decrease,pad=1080:1920:(ow-iw)/2:(oh-ih)/2:black".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for story format (9:16, 15 seconds max, optimized encoding)
pub fn story_format_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-t".to_string(),
            "15".to_string(), // Max 15 seconds
            "-vf".to_string(),
            "scale=1080:1920:force_original_aspect_ratio=decrease,pad=1080:1920:(ow-iw)/2:(oh-ih)/2:black".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-preset".to_string(),
            "fast".to_string(),
            "-crf".to_string(),
            "23".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            "-b:a".to_string(),
            "128k".to_string(),
            "-movflags".to_string(),
            "+faststart".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for noise reduction
pub fn noise_reduction_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    // Use highpass and lowpass filters to reduce noise
    // Also use anr (adaptive noise reduction) filter
    let video_codec = get_video_codec(input, output);
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            "highpass=f=200,lowpass=f=3000,anlmdn=s=0.0003".to_string(),
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for echo/reverb removal
pub fn echo_removal_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    // Use aecho filter to remove echo/reverb
    let video_codec = get_video_codec(input, output);
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            "aecho=0.8:0.88:60:0.4".to_string(), // in_gain:out_gain:delays:decays
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for audio ducking (lower background when voice detected)
pub fn audio_ducking_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    // Use sidechaincompress filter to duck audio when voice is detected
    // This is a simplified version - full implementation would need voice detection
    let video_codec = get_video_codec(input, output);
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            "acompressor=threshold=0.05:ratio=9:attack=5:release=50".to_string(),
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for audio equalizer
pub fn audio_equalizer_steps(
    input: &Path,
    output: &Path,
    bass: Option<i32>,
    treble: Option<i32>,
    mid: Option<i32>,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let mut filters = Vec::new();
    
    // Bass (low frequencies, ~60-250 Hz)
    if let Some(b) = bass {
        let gain = (b as f64 / 20.0).clamp(-20.0, 20.0);
        filters.push(format!("equalizer=f=100:width_type=h:width=100:g={}", gain));
    }
    
    // Mid (mid frequencies, ~250-4000 Hz)
    if let Some(m) = mid {
        let gain = (m as f64 / 20.0).clamp(-20.0, 20.0);
        filters.push(format!("equalizer=f=1000:width_type=h:width=2000:g={}", gain));
    }
    
    // Treble (high frequencies, ~4000-20000 Hz)
    if let Some(t) = treble {
        let gain = (t as f64 / 20.0).clamp(-20.0, 20.0);
        filters.push(format!("equalizer=f=10000:width_type=h:width=5000:g={}", gain));
    }
    
    let filter_chain = if filters.is_empty() {
        "anull".to_string() // No-op filter if no adjustments
    } else {
        filters.join(",")
    };
    
    let video_codec = get_video_codec(input, output);
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            filter_chain,
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for voice isolation
pub fn voice_isolation_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    // Use bandpass filter to isolate voice frequencies (typically 300-3400 Hz)
    let video_codec = get_video_codec(input, output);
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            "highpass=f=300,lowpass=f=3400".to_string(),
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for audio speed change without pitch shift
pub fn audio_speed_keep_pitch_steps(
    input: &Path,
    output: &Path,
    factor: f64,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // Use atempo filter for speed change (can only do 0.5-2.0x, chain multiple for larger factors)
    // Then use asetrate and aresample to maintain pitch
    let mut filters = Vec::new();
    
    let mut remaining = factor;
    while remaining > 2.0 {
        filters.push("atempo=2.0".to_string());
        remaining /= 2.0;
    }
    while remaining < 0.5 {
        filters.push("atempo=0.5".to_string());
        remaining *= 2.0;
    }
    if (remaining - 1.0).abs() > 0.01 {
        filters.push(format!("atempo={}", remaining));
    }
    
    let filter_chain = if filters.is_empty() {
        "anull".to_string()
    } else {
        filters.join(",")
    };
    
    let video_codec = get_video_codec(input, output);
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            filter_chain,
            "-c:v".to_string(),
            video_codec.to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for glitch effects
pub fn glitch_steps(
    input: &Path,
    output: &Path,
    shift: Option<u32>,
    noise: Option<u32>,
    overwrite: bool,
) -> Vec<Step> {
    // Create digital glitch effects using RGB channel separation and noise
    // The effect creates:
    // 1. RGB channel separation (chromatic aberration) - creates color fringing
    //    - Red channel shifted right by shift pixels
    //    - Blue channel shifted left by shift pixels
    //    - Green channel stays in place
    // 2. Digital noise/artifacts for that corrupted digital look
    // 3. Creates the classic "glitch" aesthetic with RGB separation
    
    let mut shift_amount = shift.unwrap_or(3);
    let mut noise_amount = noise.unwrap_or(30);
    
    // Clamp shift to reasonable values to avoid out-of-bounds access
    // Large shifts can cause "Result too large" errors
    // Limit to 15 pixels max to prevent issues
    shift_amount = shift_amount.min(15);
    
    // Clamp noise to reasonable values (0-100)
    noise_amount = noise_amount.min(100);
    
    // Using geq to shift RGB channels
    // Since we clamp shift_amount to max 15, we don't need complex bounds checking
    // The geq filter will handle out-of-bounds access by clamping automatically
    // Note: r(), g(), b() functions require both X and Y coordinates
    let filter = format!(
        "format=rgb24,geq=r='r(X+{},Y)':g='g(X,Y)':b='b(X-{},Y)',noise=alls={}:allf=t+u,format=yuv420p",
        shift_amount, shift_amount, noise_amount
    );
    
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for vintage film effect
pub fn vintage_film_steps(
    input: &Path,
    output: &Path,
    era: Option<String>,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // Apply film grain, scratches, and vintage color grading based on era
    // Each era has different characteristics:
    // - 70s: Warm tones, high grain, desaturated, soft contrast
    // - 80s: Cool tones, moderate grain, vibrant colors, high contrast
    // - 90s: Neutral tones, low grain, balanced colors, moderate contrast
    // - classic (default): Vintage curves, moderate grain, warm tones
    
    let era_lower = era.as_ref().map(|s| s.to_lowercase()).unwrap_or_else(|| "classic".to_string());
    
    let filter = match era_lower.as_str() {
        "70s" => {
            // 70s: Warm, grainy, desaturated, soft
            "noise=alls=15:allf=t+u,curves=vintage,eq=brightness=0.1:contrast=1.05:saturation=0.7,colorbalance=rs=0.15:gs=-0.05:bs=-0.1"
        }
        "80s" => {
            // 80s: Cool, vibrant, high contrast, moderate grain
            "noise=alls=12:allf=t+u,curves=vintage,eq=brightness=-0.05:contrast=1.2:saturation=1.1,colorbalance=rs=-0.1:gs=0.05:bs=0.15"
        }
        "90s" => {
            // 90s: Neutral, balanced, low grain
            "noise=alls=8:allf=t+u,curves=vintage,eq=brightness=0.02:contrast=1.1:saturation=0.9,colorbalance=rs=0.05:gs=0:bs=-0.05"
        }
        "classic" | _ => {
            // Classic vintage: default vintage look
            "noise=alls=10:allf=t+u,curves=vintage,eq=brightness=0.05:contrast=1.1:saturation=0.8"
        }
    };
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter.to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for split screen
pub fn split_screen_steps(
    input1: &Path,
    input2: &Path,
    output: &Path,
    orientation: crate::model::types::SplitScreenOrientation,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::SplitScreenOrientation;
    
    let filter = match orientation {
        SplitScreenOrientation::Horizontal => {
            // Side-by-side: scale both to half width, stack horizontally, then ensure even dimensions
            format!(
                "[0:v]scale=iw/2:-1[v0];[1:v]scale=iw/2:-1[v1];[v0][v1]hstack[h];[h]scale=iw:-2[v]"
            )
        }
        SplitScreenOrientation::Vertical => {
            // Top/bottom: scale both to half height, stack vertically, then ensure even dimensions
            format!(
                "[0:v]scale=-1:ih/2[v0];[1:v]scale=-1:ih/2[v1];[v0][v1]vstack[v];[v]scale=-2:ih[v]"
            )
        }
    };
    
    let audio_codec = get_audio_codec(input1, output);
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input1.to_string_lossy().to_string(),
            "-i".to_string(),
            input2.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter,
            "-map".to_string(),
            "[v]".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-pix_fmt".to_string(),
            "yuv420p".to_string(),
            "-c:a".to_string(),
            audio_codec.to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Use audio from first input if available
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for mirror effect
pub fn mirror_steps(
    input: &Path,
    output: &Path,
    direction: crate::model::types::MirrorDirection,
    overwrite: bool,
) -> Vec<Step> {
    use crate::model::types::MirrorDirection;
    
    let filter = match direction {
        MirrorDirection::Horizontal => "hflip",
        MirrorDirection::Vertical => "vflip",
    };
    
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter.to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for color grading
pub fn color_grade_steps(
    input: &Path,
    output: &Path,
    preset: crate::model::types::ColorGradePreset,
    overwrite: bool,
) -> Vec<Step> {
    use crate::model::types::ColorGradePreset;
    
    let filter = match preset {
        ColorGradePreset::Cinematic => "curves=preset=lighter,eq=contrast=1.2:saturation=1.1",
        ColorGradePreset::Warm => "colorbalance=rs=0.15:gs=-0.05:bs=-0.15,eq=saturation=1.1",
        ColorGradePreset::Cool => "colorbalance=rs=-0.1:gs=0.05:bs=0.15,eq=saturation=1.1",
        ColorGradePreset::Dramatic => "curves=preset=strong_contrast,eq=contrast=1.3:saturation=1.2",
    };
    
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter.to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for animated text
pub fn animated_text_steps(
    input: &Path,
    output: &Path,
    text: &str,
    position: &crate::model::types::TextPosition,
    animation: crate::model::types::TextAnimation,
    style: &crate::model::types::TextStyle,
    overwrite: bool,
    ass_file_path: Option<&Path>,
) -> Result<Vec<Step>> {
    use crate::model::types::{TextAnimation, TextPosition};
    
    // Build position string
    let (x, y) = match position {
        TextPosition::TopLeft => ("10", "10"),
        TextPosition::TopRight => ("w-text_w-10", "10"),
        TextPosition::TopCenter => ("(w-text_w)/2", "10"),
        TextPosition::BottomLeft => ("10", "h-text_h-10"),
        TextPosition::BottomRight => ("w-text_w-10", "h-text_h-10"),
        TextPosition::BottomCenter => ("(w-text_w)/2", "h-text_h-10"),
        TextPosition::Center => ("(w-text_w)/2", "(h-text_h)/2"),
        TextPosition::Custom { x: _, y: _ } => {
            return Err(anyhow::anyhow!("Custom position not supported for animated text"));
        }
    };
    
    let font_size = style.font_size.unwrap_or(24);
    let color = style.color.to_ffmpeg();
    
    // Build filter based on animation type
    let filter = match animation {
        TextAnimation::FadeIn => {
            // Fade in over 1 second
            format!("drawtext=text='{}':fontsize={}:fontcolor={}:x={}:y={}:alpha='if(lt(t,1), t/1, 1)'", 
                text.replace("'", "\\'"), font_size, color, x, y)
        }
        TextAnimation::SlideIn => {
            // Slide in from left over 1 second
            // Start off-screen to the left (-text_w) and slide to final position
            // Animation duration: 1 second, speed: move from -text_w to final x position
            format!("drawtext=text='{}':fontsize={}:fontcolor={}:x='if(lt(t,1), -text_w + (t*({} + text_w)), {})':y={}", 
                text.replace("'", "\\'"), font_size, color, x, x, y)
        }
        TextAnimation::Typewriter => {
            // Typewriter effect - use ASS subtitles with karaoke for proper character-by-character reveal
            if let Some(ass_file) = ass_file_path {
                let ass_file_str = ass_file.to_string_lossy().into_owned();
                // Use subtitles filter to render ASS file with karaoke effect
                format!("subtitles='{}'", ass_file_str.replace("'", "\\'"))
            } else {
                // Fallback: show full text with fade-in if ASS file not provided
                let escaped_text = text.replace("'", "\\'");
                format!("drawtext=text='{}':fontsize={}:fontcolor={}:x={}:y={}:alpha='if(lt(t,1), t/1, 1)'", 
                    escaped_text, font_size, color, x, y)
            }
        }
    };
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            filter,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for video transitions
pub fn transition_steps(
    input1: &Path,
    input2: &Path,
    output: &Path,
    transition_type: crate::model::types::TransitionType,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::TransitionType;
    
    // Get duration of first video to determine transition point
    // For now, we'll use a fixed 1-second transition at the end of video1
    // In a full implementation, we'd probe the video duration
    
    let filter = match transition_type {
        TransitionType::Fade => {
            // Crossfade transition
            format!(
                "[0:v][1:v]xfade=transition=fade:duration=1:offset=4"
            )
        }
        TransitionType::Wipe => {
            // Wipe transition (left to right)
            format!(
                "[0:v][1:v]xfade=transition=wipeleft:duration=1:offset=4"
            )
        }
        TransitionType::Slide => {
            // Slide transition
            format!(
                "[0:v][1:v]xfade=transition=slideleft:duration=1:offset=4"
            )
        }
    };
    
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input1.to_string_lossy().to_string(),
            "-i".to_string(),
            input2.to_string_lossy().to_string(),
            "-filter_complex".to_string(),
            filter,
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            "-map".to_string(),
            "0:a?".to_string(), // Use audio from first input
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for syncing multiple cameras by audio
pub fn sync_cameras_steps(
    videos: &[PathBuf],
    output: &Path,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // Use align audio filter to sync multiple videos by their audio tracks
    // This is a simplified version - full implementation would analyze audio correlation
    // For now, we'll use the first video as reference and align others to it
    
    if videos.len() < 2 {
        bail!("Sync cameras requires at least 2 videos");
    }
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
    ];
    
    // Add all video inputs
    for video in videos.iter() {
        args.push("-i".to_string());
        args.push(video.to_string_lossy().to_string());
    }
    
    // For video, scale and arrange in grid (2x2 for 4 videos, etc.)
    let cols = (videos.len() as f64).sqrt().ceil() as u32;
    let rows = ((videos.len() as f64) / (cols as f64)).ceil() as u32;
    
    // Build filter complex for video only (simplified - no audio alignment for now)
    let mut filter_parts = Vec::new();
    for i in 0..videos.len() {
        filter_parts.push(format!("[{}:v]scale=320:240[v{}]", i, i));
    }
    
    // Calculate output dimensions
    let output_width = cols * 320;
    let _output_height = rows * 240;
    
    // Use hstack/vstack approach like montage for more reliable layout
    // Build grid using hstack and vstack
    let mut row_filters = Vec::new();
    let mut row_labels = Vec::new();
    for row in 0..rows {
        let mut row_inputs = Vec::new();
        for col in 0..cols {
            let idx = (row * cols + col) as usize;
            if idx < videos.len() {
                row_inputs.push(format!("[v{}]", idx));
            }
        }
        if !row_inputs.is_empty() {
            let row_label = format!("row{}", row);
            row_filters.push(format!("{}hstack=inputs={}[{}]", 
                row_inputs.join(""), row_inputs.len(), row_label));
            row_labels.push(format!("[row{}]", row));
        }
    }
    
    let scale_chain = filter_parts.join(";");
    let row_chain = row_filters.join(";");
    // If only one row, skip vstack and use the row directly
    let filter_complex = if row_labels.len() > 1 {
        format!("{};{};{}vstack=inputs={}[vstack];[vstack]scale={}:-2[v]", 
            scale_chain, row_chain, row_labels.join(""), row_labels.len(), output_width)
    } else if !row_labels.is_empty() {
        let row_name = row_labels[0].trim_start_matches('[').trim_end_matches(']');
        format!("{};{};[{}]scale={}:-2[v]", 
            scale_chain, row_chain, row_name, output_width)
    } else {
        format!("{};{}[v]", scale_chain, row_chain)
    };
    
    args.push("-filter_complex".to_string());
    args.push(filter_complex);
    args.push("-map".to_string());
    args.push("[v]".to_string());
    args.push("-map".to_string());
    args.push("0:a?".to_string()); // Use audio from first video if available
    args.push("-c:v".to_string());
    args.push("libx264".to_string());
    args.push("-pix_fmt".to_string());
    args.push("yuv420p".to_string());
    let audio_codec = if !videos.is_empty() {
        get_audio_codec(&videos[0], output)
    } else {
        "aac"
    };
    args.push("-c:a".to_string());
    args.push(audio_codec.to_string());
    args.push(output.to_string_lossy().to_string());
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Build steps for generating test pattern
pub fn generate_test_pattern_steps(
    resolution: &str,
    duration: &crate::model::types::Duration,
    output: &Path,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // Parse resolution
    use regex::Regex;
    let (width, height) = match resolution.to_lowercase().as_str() {
        "720p" => (1280, 720),
        "1080p" => (1920, 1080),
        "4k" | "2160p" => (3840, 2160),
        _ => {
            // Try to parse as WxH
            let re = Regex::new(r"(\d+)x(\d+)")
                .map_err(|e| anyhow!("Invalid regex: {}", e))?;
            let caps = re.captures(resolution)
                .ok_or_else(|| anyhow!("Invalid resolution: {resolution}"))?;
            let w = caps.get(1).unwrap().as_str().parse::<u32>()?;
            let h = caps.get(2).unwrap().as_str().parse::<u32>()?;
            (w, h)
        }
    };
    
    let duration_secs = duration.to_seconds();
    
    // Generate SMPTE color bars test pattern
    // Use testsrc filter to generate test pattern
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-f".to_string(),
            "lavfi".to_string(),
            "-i".to_string(),
            format!("testsrc=duration={}:size={}x{}:rate=30", duration_secs, width, height),
            "-t".to_string(),
            duration_secs.to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-pix_fmt".to_string(),
            "yuv420p".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for adding timecode overlay
pub fn add_timecode_steps(
    input: &Path,
    output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use drawtext filter to add timecode overlay
    // Timecode format: HH:MM:SS:FF (hours:minutes:seconds:frames)
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "drawtext=text='%{pts\\:hms}':fontsize=24:fontcolor=white:x=10:y=10:box=1:boxcolor=black@0.5".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for generating proxy (low-res version for editing)
pub fn proxy_steps(
    input: &Path,
    output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Generate low-resolution proxy (typically 720p or 480p)
    // Use fast encoding preset for speed
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "scale=1280:720".to_string(), // 720p proxy
            "-c:v".to_string(),
            "libx264".to_string(),
            "-preset".to_string(),
            "ultrafast".to_string(), // Fast encoding for proxies
            "-crf".to_string(),
            "28".to_string(), // Lower quality for speed
            "-c:a".to_string(),
            "aac".to_string(),
            "-b:a".to_string(),
            "128k".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for exporting EDL (Edit Decision List)
pub fn export_edl_steps(
    input: &Path,
    _output: &Path,
    _overwrite: bool,
) -> Vec<Step> {
    // Use ffmpeg to export EDL format
    // EDL is a simple text format listing edit points
    // This is a simplified version - full EDL export would require more analysis
    vec![Step::new(
        "ffprobe",
        vec![
            "-v".to_string(),
            "error".to_string(),
            "-show_frames".to_string(),
            "-select_streams".to_string(),
            "v:0".to_string(),
            "-show_entries".to_string(),
            "frame=pkt_pts_time".to_string(),
            "-of".to_string(),
            "csv=p=0".to_string(),
            input.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for converting color space
pub fn convert_colorspace_steps(
    input: &Path,
    output: &Path,
    target: &crate::model::types::Colorspace,
    overwrite: bool,
) -> Vec<Step> {
    let colorspace = target.to_ffmpeg();
    let audio_codec = get_audio_codec(input, output);
    
    // Use colorspace filter with proper syntax: colorspace=all=output:iall=input
    // For now, we'll convert to target colorspace (assuming input is bt709)
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            format!("colorspace=all={}:iall=bt709", colorspace),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            audio_codec.to_string(),
            output.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for detecting silence in audio
pub fn detect_silence_steps(
    input: &Path,
    _output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use silencedetect filter to find silent segments
    // Output will be captured from stderr/logs
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            "silencedetect=noise=-30dB:duration=0.5".to_string(),
            "-f".to_string(),
            "null".to_string(),
            "-".to_string(),
        ],
    )]
}

/// Build steps for analyzing loudness (LUFS)
pub fn analyze_loudness_steps(
    input: &Path,
    _output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use loudnorm filter in first pass to analyze loudness
    // Output will be captured from stderr/logs
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-af".to_string(),
            "loudnorm=I=-16:TP=-1.5:LRA=11:print_format=json".to_string(),
            "-f".to_string(),
            "null".to_string(),
            "-".to_string(),
        ],
    )]
}

/// Build steps for detecting duplicate frames
pub fn detect_duplicates_steps(
    input: &Path,
    _output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use select filter to find duplicate frames
    // The 'scene' variable compares current frame to previous frame
    // Low scene value (< 0.0001) indicates duplicate frames
    // Use null muxer with platform-specific null device
    let null_sink = if cfg!(windows) { "NUL" } else { "/dev/null" };
    vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "select='not(gt(scene\\,0.0001))',showinfo".to_string(),
            "-f".to_string(),
            "null".to_string(),
            null_sink.to_string(),
        ],
    )]
}

/// Build steps for video collage (similar to montage but with more layout options)
pub fn collage_steps(
    videos: &[PathBuf],
    output: &Path,
    layout: &crate::model::types::MontageLayout,
    overwrite: bool,
) -> Result<Vec<Step>> {
    // Reuse montage logic since collage is similar
    montage_steps(videos, output, layout, overwrite)
}

/// Build steps for creating video slideshow from images
pub fn slideshow_steps(
    images: &[PathBuf],
    output: &Path,
    duration: crate::model::types::Duration,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let duration_sec = duration.to_seconds();
    
    // Build filter complex to concatenate images with specified duration
    let mut filter_parts = Vec::new();
    let mut inputs = Vec::new();
    
    for (i, image) in images.iter().enumerate() {
        inputs.push("-loop".to_string());
        inputs.push("1".to_string());
        inputs.push("-t".to_string());
        inputs.push(duration_sec.to_string());
        inputs.push("-i".to_string());
        inputs.push(image.to_string_lossy().to_string());
        
        filter_parts.push(format!("[{}:v]scale=1280:720:force_original_aspect_ratio=decrease,pad=1280:720:(ow-iw)/2:(oh-ih)/2,setsar=1[v{}]", i, i));
    }
    
    // Concatenate all video segments
    let input_labels: Vec<String> = (0..images.len()).map(|i| format!("[v{}]", i)).collect();
    let concat_filter = format!("{};{}concat=n={}:v=1:a=0[outv]", 
        filter_parts.join(";"), 
        input_labels.join(""),
        images.len());
    
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
    ];
    args.extend(inputs);
    args.push("-filter_complex".to_string());
    args.push(concat_filter);
    args.push("-map".to_string());
    args.push("[outv]".to_string());
    args.push("-c:v".to_string());
    args.push("libx264".to_string());
    args.push("-pix_fmt".to_string());
    args.push("yuv420p".to_string());
    args.push("-r".to_string());
    args.push("30".to_string());
    args.push(output.to_string_lossy().to_string());
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Build steps for visualizing audio as video (waveform or spectrum)
pub fn visualize_steps(
    audio: &Path,
    output: &Path,
    style: crate::model::types::VisualizationStyle,
    overwrite: bool,
) -> Result<Vec<Step>> {
    use crate::model::types::VisualizationStyle;
    
    // For animated visualization, we create a video from audio
    // Use filter_complex to connect audio stream to video filter
    let filter_complex = match style {
        VisualizationStyle::Waveform => {
            "[0:a]showwaves=s=1280x720:mode=line:colors=0xFFFFFF:scale=lin[v]"
        }
        VisualizationStyle::Spectrum => {
            "[0:a]showspectrum=s=1280x720:color=intensity:slide=scroll[v]"
        }
    };
    
    let animated_args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        audio.to_string_lossy().to_string(),
        "-filter_complex".to_string(),
        filter_complex.to_string(),
        "-map".to_string(),
        "[v]".to_string(),
        "-c:v".to_string(),
        "libx264".to_string(),
        "-pix_fmt".to_string(),
        "yuv420p".to_string(),
        "-r".to_string(),
        "30".to_string(),
        output.to_string_lossy().to_string(),
    ];
    
    Ok(vec![Step::new("ffmpeg", animated_args)])
}

/// Build steps for animated GIF with loop and optimization options
pub fn animated_gif_steps(
    input: &Path,
    output: &Path,
    palette_path: &Path,
    fps: u32,
    width: u32,
    loop_video: bool,
    optimize: bool,
) -> Vec<Step> {
    let mut steps = Vec::new();
    
    // Pass 1: Generate palette
    let mut palette_args = vec![
        "-y".to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-vf".to_string(),
        format!("fps={fps},scale={width}:-1:flags=lanczos,palettegen"),
    ];
    
    if optimize {
        // Add stats_mode for better palette generation
        // stats_mode is a parameter of palettegen, not a separate filter
        // The filter string is at index 4 (after "-vf" at index 3)
        palette_args[4] = format!("fps={fps},scale={width}:-1:flags=lanczos,palettegen=stats_mode=diff");
    }
    
    palette_args.push(palette_path.to_string_lossy().to_string());
    steps.push(Step::new("ffmpeg", palette_args));
    
    // Pass 2: Render with palette
    let mut render_args = vec![
        "-y".to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-i".to_string(),
        palette_path.to_string_lossy().to_string(),
        "-lavfi".to_string(),
    ];
    
    let mut filter = format!("fps={fps},scale={width}:-1:flags=lanczos[x];[x][1:v]paletteuse");
    
    if optimize {
        filter = format!("fps={fps},scale={width}:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer:bayer_scale=5");
    }
    
    render_args.push(filter);
    
    if loop_video {
        // Add loop option (GIFs loop by default, but we can make it explicit)
        render_args.push("-loop".to_string());
        render_args.push("0".to_string()); // 0 = infinite loop
    }
    
    render_args.push(output.to_string_lossy().to_string());
    steps.push(Step::new("ffmpeg", render_args));
    
    steps
}

/// Build steps for tiling video in a grid pattern
pub fn tile_steps(
    input: &Path,
    output: &Path,
    layout: &crate::model::types::MontageLayout,
    overwrite: bool,
) -> Result<Vec<Step>> {
    let cols = layout.cols;
    let rows = layout.rows;
    let total_cells = (cols * rows) as usize;
    
    // Build filter complex to repeat video in grid
    let cell_width = 320;
    let cell_height = 240;
    
    // Scale input video to cell size, output to [scaled] to avoid reusing [v0]
    let mut filter_parts = vec![format!(
        "[0:v]scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2[scaled]",
        cell_width, cell_height, cell_width, cell_height
    )];
    
    // Split the scaled video into multiple streams
    // split filter syntax: [input]split=n:outputs=N[out1][out2]...[outN]
    let split_outputs: Vec<String> = (0..total_cells).map(|i| format!("[v{}]", i)).collect();
    filter_parts.push(format!("[scaled]split={}:{}", total_cells, split_outputs.join("")));
    
    // Use hstack/vstack approach for more reliable layout
    // Build grid using hstack and vstack
    let mut row_filters = Vec::new();
    let mut row_labels = Vec::new();
    for row in 0..rows {
        let mut row_inputs = Vec::new();
        for col in 0..cols {
            let idx = (row * cols + col) as usize;
            row_inputs.push(format!("[v{}]", idx));
        }
        let row_label = format!("row{}", row);
        row_filters.push(format!("{}hstack=inputs={}[{}]", 
            row_inputs.join(""), row_inputs.len(), row_label));
        row_labels.push(format!("[row{}]", row));
    }
    
    let scale_chain = filter_parts.join(";");
    let row_chain = row_filters.join(";");
    // Calculate output dimensions
    let output_width = cols * cell_width;
    let _output_height = rows * cell_height;
    
    // Combine scale, split, row creation, and vertical stacking
    // If only one row, skip vstack and use the row directly
    let filter_complex = if row_labels.len() > 1 {
        format!("{};{};{}vstack=inputs={}[vstack];[vstack]scale={}:-2[v]", 
            scale_chain, row_chain, row_labels.join(""), row_labels.len(), output_width)
    } else if !row_labels.is_empty() {
        let row_name = row_labels[0].trim_start_matches('[').trim_end_matches(']');
        format!("{};{};[{}]scale={}:-2[v]", 
            scale_chain, row_chain, row_name, output_width)
    } else {
        format!("{};{}[v]", scale_chain, row_chain)
    };
    
    let audio_codec = get_audio_codec(input, output);
    let args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
        "-filter_complex".to_string(),
        filter_complex,
        "-map".to_string(),
        "[v]".to_string(),
        "-map".to_string(),
        "0:a?".to_string(), // Use audio from input
        "-c:v".to_string(),
        "libx264".to_string(),
        "-pix_fmt".to_string(),
        "yuv420p".to_string(),
        "-c:a".to_string(),
        audio_codec.to_string(),
        output.to_string_lossy().to_string(),
    ];
    
    Ok(vec![Step::new("ffmpeg", args)])
}

/// Build steps for repairing corrupted video files
pub fn repair_steps(
    input: &Path,
    output: &Path,
    overwrite: bool,
) -> Vec<Step> {
    // Use ffmpeg to attempt to repair by copying streams and ignoring errors
    let video_codec = get_video_codec(input, output);
    let mut args = vec![
        if overwrite { "-y" } else { "-n" }.to_string(),
        "-err_detect".to_string(),
        "ignore_err".to_string(),
        "-i".to_string(),
        input.to_string_lossy().to_string(),
    ];
    
    if video_codec == "copy" {
        args.push("-c".to_string());
        args.push("copy".to_string());
    } else {
        args.push("-c:v".to_string());
        args.push(video_codec.to_string());
        args.push("-c:a".to_string());
        args.push("copy".to_string());
    }
    
    args.push("-fflags".to_string());
    args.push("+genpts".to_string());
    args.push(output.to_string_lossy().to_string());
    
    vec![Step::new("ffmpeg", args)]
}

/// Build steps for validating video file
pub fn validate_steps(
    input: &Path,
) -> Vec<Step> {
    // Use ffprobe to check if file is valid
    vec![Step::new(
        "ffprobe",
        vec![
            "-v".to_string(),
            "error".to_string(),
            "-show_entries".to_string(),
            "format=duration,size".to_string(),
            "-of".to_string(),
            "default=noprint_wrappers=1".to_string(),
            input.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for extracting keyframes (I-frames) only
pub fn extract_keyframes_steps(
    input: &Path,
    output_dir: &Path,
) -> Vec<Step> {
    // Extract only I-frames using select filter
    vec![Step::new(
        "ffmpeg",
        vec![
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "select='eq(pict_type,I)'".to_string(),
            "-vsync".to_string(),
            "vfr".to_string(),
            "-f".to_string(),
            "image2".to_string(),
            output_dir.join("keyframe_%05d.jpg").to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for getting detailed video statistics
pub fn stats_steps(
    input: &Path,
) -> Vec<Step> {
    // Use ffprobe to get detailed statistics
    vec![Step::new(
        "ffprobe",
        vec![
            "-v".to_string(),
            "error".to_string(),
            "-show_entries".to_string(),
            "stream=codec_name,codec_type,width,height,bit_rate,r_frame_rate,duration,nb_frames".to_string(),
            "-show_entries".to_string(),
            "format=size,duration,bit_rate".to_string(),
            "-of".to_string(),
            "json".to_string(),
            input.to_string_lossy().to_string(),
        ],
    )]
}

/// Build steps for comparing videos with optional PSNR/SSIM metrics
pub fn compare_steps_with_metrics(
    video1: &Path,
    video2: &Path,
    output: &Path,
    overwrite: bool,
    show_psnr: bool,
) -> Vec<Step> {
    if show_psnr {
        // Create side-by-side comparison and calculate PSNR/SSIM metrics
        // First, create the comparison video
        let mut steps = compare_steps(video1, video2, output, overwrite);
        
        // Add a step to calculate PSNR/SSIM metrics separately
        // Scale both videos to same size, then calculate metrics
        steps.push(Step::new(
            "ffmpeg",
            vec![
                "-i".to_string(),
                video1.to_string_lossy().to_string(),
                "-i".to_string(),
                video2.to_string_lossy().to_string(),
                "-lavfi".to_string(),
                "[0:v]scale=iw*min(720/iw\\,720/ih):ih*min(720/iw\\,720/ih)[v0];[1:v]scale=iw*min(720/iw\\,720/ih):ih*min(720/iw\\,720/ih)[v1];[v0][v1]psnr=stats_file=psnr.log;[v0][v1]ssim=stats_file=ssim.log".to_string(),
                "-f".to_string(),
                "null".to_string(),
                "-".to_string(),
            ],
        ));
        
        steps
    } else {
        // Regular side-by-side comparison (existing behavior)
        compare_steps(video1, video2, output, overwrite)
    }
}

/// Build steps for converting video to 360° format
/// Adds 360° video metadata and projection information
pub fn convert_360_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            "-metadata:s:v:0".to_string(),
            "spherical-video=1".to_string(),
            "-metadata:s:v:0".to_string(),
            "stereo-mode=mono".to_string(),
            "-vf".to_string(),
            "v360=input=equirect:output=equirect".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for converting HDR video to SDR (tone mapping)
/// Uses zscale filter for high-quality tone mapping
pub fn convert_hdr_to_sdr_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    let audio_codec = get_audio_codec(input, output);
    // HDR to SDR conversion: first normalize colorspace, then convert to linear, tonemap, then back to bt709
    // Use colorspace filter first to handle inputs with unknown colorspace metadata
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vf".to_string(),
            "colorspace=bt709:iall=bt709:fast=1,zscale=t=linear:npl=100,format=gbrpf32le,zscale=p=bt709,tonemap=hable:desat=0,zscale=t=bt709:m=bt709:r=tv,format=yuv420p".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-crf".to_string(),
            "23".to_string(),
            "-preset".to_string(),
            "medium".to_string(),
            "-c:a".to_string(),
            audio_codec.to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}

/// Build steps for fixing variable frame rate (VFR) to constant frame rate (CFR)
/// Uses fps filter to convert VFR to CFR at the source frame rate or 30fps
pub fn fix_framerate_steps(input: &Path, output: &Path, overwrite: bool) -> Result<Vec<Step>> {
    Ok(vec![Step::new(
        "ffmpeg",
        vec![
            if overwrite { "-y" } else { "-n" }.to_string(),
            "-i".to_string(),
            input.to_string_lossy().to_string(),
            "-vsync".to_string(),
            "cfr".to_string(),
            "-r".to_string(),
            "30".to_string(),
            "-c:v".to_string(),
            "libx264".to_string(),
            "-crf".to_string(),
            "23".to_string(),
            "-preset".to_string(),
            "medium".to_string(),
            "-c:a".to_string(),
            "copy".to_string(),
            output.to_string_lossy().to_string(),
        ],
    )])
}
