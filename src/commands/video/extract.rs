use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::{Duration, Time};
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::Result;
use std::path::Path;

pub fn handle_thumbnail(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    time: Time,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "thumb", "jpg")?;
    let steps = recipes::thumbnail_steps(input, &out, &time, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_crop(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    width: u32,
    height: u32,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "crop", "mp4")?;
    let steps = recipes::crop_steps(input, &out, width, height, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_extract_frames(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    interval: Duration,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    // Create output directory for frames
    let output_dir = if let Some(output_dir) = &config.output_dir {
        std::fs::create_dir_all(output_dir)?;
        output_dir.clone()
    } else {
        let dir = input.parent().unwrap_or_else(|| Path::new("."));
        let dir_name = format!("{}_frames", input.file_stem().unwrap().to_string_lossy());
        let frames_dir = dir.join(dir_name);
        std::fs::create_dir_all(&frames_dir)?;
        frames_dir
    };

    let steps = recipes::extract_frames_steps(input, &output_dir, &interval, config.overwrite)?;
    
    eprintln!("Extracting {} frames...", steps.len());
    for (idx, step) in steps.iter().enumerate() {
        if idx % 10 == 0 || idx == steps.len() - 1 {
            eprintln!(" Processing frame {}/{}...", idx + 1, steps.len());
        }
        runner.run(step)?;
    }

    eprintln!("Output directory: {}", output_dir.display());
    Ok(())
}

pub fn handle_detect_scenes(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    // Output scene detection file (text file with timestamps)
    let out = if let Some(out) = &config.out {
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        out.clone()
    } else {
        let dir = if let Some(output_dir) = &config.output_dir {
            std::fs::create_dir_all(output_dir)?;
            output_dir
        } else {
            input.parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = input.file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_scenes.txt"))
    };

    let steps = recipes::detect_scenes_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_detect_black(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    // Output black frame detection file (text file with timestamps)
    let out = if let Some(out) = &config.out {
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent)?;
        }
        out.clone()
    } else {
        let dir = if let Some(output_dir) = &config.output_dir {
            std::fs::create_dir_all(output_dir)?;
            output_dir
        } else {
            input.parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = input.file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_black_frames.txt"))
    };

    let steps = recipes::detect_black_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_analyze_quality(
    _config: &AppConfig,
    _runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    use crate::ffmpeg::probe;
    use crate::util::system::ensure_ffprobe_exists;
    
    ensure_ffprobe_exists()?;
    let input = input.as_ref();

    // Use ffprobe to get video information
    let video_info = probe::get_video_info(input)?;
    
    eprintln!("Video Quality Analysis");
    eprintln!("----------------------------------------");
    eprintln!("File: {}", input.display());
    eprintln!("Duration: {:.2}s", video_info.duration);
    eprintln!("Resolution: {}x{}", video_info.width, video_info.height);
    eprintln!("Frame Rate: {:.2} fps", video_info.fps);
    eprintln!("Video Codec: {}", video_info.video_codec);
    eprintln!("Video Bitrate: {}", video_info.video_bitrate);
    eprintln!("Audio Codec: {}", video_info.audio_codec);
    eprintln!("Audio Bitrate: {}", video_info.audio_bitrate);
    eprintln!("Total Bitrate: {}", video_info.total_bitrate);
    eprintln!("File Size: {}", video_info.file_size);
    eprintln!("----------------------------------------");
    
    Ok(())
}

pub fn handle_preview(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "preview", "mp4")?;
    let steps = recipes::preview_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_suggest_format(
    _config: &AppConfig,
    _runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    use crate::ffmpeg::probe;
    use crate::util::system::ensure_ffprobe_exists;
    
    ensure_ffprobe_exists()?;
    let input = input.as_ref();

    // Get video information
    let video_info = probe::get_video_info(input)?;
    
    eprintln!("Format Suggestions for: {}", input.display());
    eprintln!("----------------------------------------");
    
    // Analyze content and suggest formats
    let suggestions = analyze_and_suggest(&video_info);
    
    eprintln!("Recommended formats:");
    for (i, suggestion) in suggestions.iter().enumerate() {
        eprintln!(" {}. {} - {}", i + 1, suggestion.format, suggestion.reason);
    }
    
    eprintln!("----------------------------------------");
    
    Ok(())
}

pub fn handle_thumbnail_grid(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    layout: crate::model::intent::ThumbnailGridLayout,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "thumbnails", "png")?;
    let steps = recipes::thumbnail_grid_steps(input, &out, layout.cols, layout.rows, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_social_media_convert(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    platform: crate::model::intent::SocialPlatform,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let platform_str = match platform {
        crate::model::intent::SocialPlatform::Instagram => "instagram",
        crate::model::intent::SocialPlatform::TikTok => "tiktok",
        crate::model::intent::SocialPlatform::YoutubeShorts => "youtube-shorts",
        crate::model::intent::SocialPlatform::Twitter => "twitter",
    };

    let out = default_out(config, input, platform_str, "mp4")?;
    let steps = recipes::social_media_convert_steps(input, &out, platform_str, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_social_crop(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    shape: crate::model::intent::SocialCropShape,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let shape_str = match shape {
        crate::model::intent::SocialCropShape::Square => "square",
        crate::model::intent::SocialCropShape::Circle => "circle",
    };

    let out = default_out(config, input, shape_str, "mp4")?;
    let steps = recipes::social_crop_steps(input, &out, shape_str, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_vertical_convert(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "vertical", "mp4")?;
    let steps = recipes::vertical_convert_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_story_format(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "story", "mp4")?;
    let steps = recipes::story_format_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

struct FormatSuggestion {
    format: String,
    reason: String,
}

fn analyze_and_suggest(info: &crate::ffmpeg::probe::VideoInfo) -> Vec<FormatSuggestion> {
    let mut suggestions = Vec::new();
    
    // Analyze resolution
    let is_high_res = info.width >= 1920 || info.height >= 1080;
    let _is_low_res = info.width < 640 || info.height < 480;
    
    // Analyze duration
    let is_short = info.duration < 30.0;
    let is_long = info.duration > 300.0; // 5 minutes
    
    // Analyze bitrate
    let _bitrate_mbps: f64 = info.total_bitrate
        .replace(" Mbps", "")
        .replace(" kbps", "")
        .parse()
        .unwrap_or(0.0);
    
    // Suggest based on content characteristics
    if is_short && is_high_res {
        suggestions.push(FormatSuggestion {
            format: "MP4 (H.264)".to_string(),
            reason: "Short high-res video - MP4 provides good quality and compatibility".to_string(),
        });
        suggestions.push(FormatSuggestion {
            format: "WebM (VP9)".to_string(),
            reason: "Good compression for web sharing".to_string(),
        });
    } else if is_short {
        suggestions.push(FormatSuggestion {
            format: "GIF".to_string(),
            reason: "Short video - GIF is perfect for sharing on social media".to_string(),
        });
        suggestions.push(FormatSuggestion {
            format: "MP4 (H.264)".to_string(),
            reason: "Universal compatibility".to_string(),
        });
    } else if is_long {
        suggestions.push(FormatSuggestion {
            format: "MP4 (H.265)".to_string(),
            reason: "Long video - H.265 provides better compression".to_string(),
        });
        suggestions.push(FormatSuggestion {
            format: "MP4 (H.264)".to_string(),
            reason: "Widely compatible format".to_string(),
        });
    } else {
        suggestions.push(FormatSuggestion {
            format: "MP4 (H.264)".to_string(),
            reason: "Best balance of quality and compatibility".to_string(),
        });
        suggestions.push(FormatSuggestion {
            format: "WebM (VP9)".to_string(),
            reason: "Good for web use with smaller file sizes".to_string(),
        });
    }
    
    // Add device-specific suggestions
    if is_high_res {
        suggestions.push(FormatSuggestion {
            format: "iPhone optimized".to_string(),
            reason: "Optimized for iOS devices".to_string(),
        });
    }
    
    // Streaming suggestions for long videos
    if is_long {
        suggestions.push(FormatSuggestion {
            format: "HLS".to_string(),
            reason: "Adaptive streaming for long content".to_string(),
        });
    }
    
    suggestions
}

pub fn handle_visualize(
    config: &AppConfig,
    runner: &dyn Runner,
    audio: impl AsRef<Path>,
    style: crate::model::types::VisualizationStyle,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let audio = audio.as_ref();

    let out = default_out(config, audio, "visualized", "mp4")?;
    let steps = recipes::visualize_steps(audio, &out, style, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_repair(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "repaired", "mp4")?;
    let steps = recipes::repair_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_validate(
    _config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    use crate::util::system::ensure_ffprobe_exists;
    
    ensure_ffprobe_exists()?;
    let input = input.as_ref();

    let steps = recipes::validate_steps(input);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("File validation completed: {}", input.display());
    Ok(())
}

pub fn handle_extract_keyframes(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    // Create output directory for keyframes
    let output_dir = if let Some(output_dir) = &config.output_dir {
        std::fs::create_dir_all(output_dir)?;
        output_dir.clone()
    } else {
        let dir = input.parent().unwrap_or_else(|| Path::new("."));
        let dir_name = format!("{}_keyframes", input.file_stem().unwrap().to_string_lossy());
        let keyframes_dir = dir.join(dir_name);
        std::fs::create_dir_all(&keyframes_dir)?;
        keyframes_dir
    };

    let steps = recipes::extract_keyframes_steps(input, &output_dir);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output directory: {}", output_dir.display());
    Ok(())
}

pub fn handle_stats(
    _config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    use crate::util::system::ensure_ffprobe_exists;
    
    ensure_ffprobe_exists()?;
    let input = input.as_ref();

    let steps = recipes::stats_steps(input);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Statistics generated for: {}", input.display());
    Ok(())
}

