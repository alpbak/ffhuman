use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::{Opacity, TextPosition, TextStyle, WatermarkPosition, WatermarkSize};
use crate::model::types::{MirrorDirection, SplitScreenOrientation, ColorGradePreset, TextAnimation, TransitionType};
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::Result;
use std::io::Write;
use std::path::Path;

pub fn handle_watermark(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    logo: impl AsRef<Path>,
    position: WatermarkPosition,
    opacity: Opacity,
    size: Option<WatermarkSize>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();
    let logo = logo.as_ref();

    // Validate that logo file exists
    if !logo.exists() {
        anyhow::bail!("Logo file not found: {}", logo.display());
    }

    let out = default_out(config, input, "watermarked", "mp4")?;
    let steps = recipes::watermark_steps(input, logo, &out, &position, &opacity, &size, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_add_text(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    text: &str,
    position: TextPosition,
    style: TextStyle,
    timestamp: bool,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "text", "mp4")?;
    let steps = recipes::add_text_steps(input, &out, text, &position, &style, timestamp, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_burn_subtitle(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    subtitle: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();
    let subtitle = subtitle.as_ref();

    // Validate that subtitle file exists
    if !subtitle.exists() {
        anyhow::bail!("Subtitle file not found: {}", subtitle.display());
    }

    let out = default_out(config, input, "subtitled", "mp4")?;
    let steps = recipes::burn_subtitle_steps(input, subtitle, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_pip(
    config: &AppConfig,
    runner: &dyn Runner,
    overlay_video: impl AsRef<Path>,
    base_video: impl AsRef<Path>,
    position: crate::model::types::PipPosition,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let overlay_video = overlay_video.as_ref();
    let base_video = base_video.as_ref();

    // Validate that both videos exist
    if !overlay_video.exists() {
        anyhow::bail!("Overlay video not found: {}", overlay_video.display());
    }
    if !base_video.exists() {
        anyhow::bail!("Base video not found: {}", base_video.display());
    }

    let out = default_out(config, base_video, "pip", "mp4")?;
    let steps = recipes::pip_steps(overlay_video, base_video, &out, &position, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_remove_background(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    color: crate::model::types::ChromaKeyColor,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "no-background", "mp4")?;
    let steps = recipes::remove_background_steps(input, &out, &color, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_overlay(
    config: &AppConfig,
    runner: &dyn Runner,
    overlay_video: impl AsRef<Path>,
    base_video: impl AsRef<Path>,
    position: crate::model::types::WatermarkPosition,
    opacity: crate::model::types::Opacity,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let overlay_video = overlay_video.as_ref();
    let base_video = base_video.as_ref();

    // Validate that both videos exist
    if !overlay_video.exists() {
        anyhow::bail!("Overlay video not found: {}", overlay_video.display());
    }
    if !base_video.exists() {
        anyhow::bail!("Base video not found: {}", base_video.display());
    }

    let out = default_out(config, base_video, "overlaid", "mp4")?;
    let steps = recipes::overlay_steps(overlay_video, base_video, &out, &position, &opacity, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_split_screen(
    config: &AppConfig,
    runner: &dyn Runner,
    video1: impl AsRef<Path>,
    video2: impl AsRef<Path>,
    orientation: SplitScreenOrientation,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let video1 = video1.as_ref();
    let video2 = video2.as_ref();

    // Validate that both videos exist
    if !video1.exists() {
        anyhow::bail!("First video not found: {}", video1.display());
    }
    if !video2.exists() {
        anyhow::bail!("Second video not found: {}", video2.display());
    }

    let out = default_out(config, video1, "split-screen", "mp4")?;
    let steps = recipes::split_screen_steps(video1, video2, &out, orientation, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_mirror(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    direction: MirrorDirection,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let suffix = match direction {
        MirrorDirection::Horizontal => "mirror-h",
        MirrorDirection::Vertical => "mirror-v",
    };
    let out = default_out(config, input, suffix, "mp4")?;
    let steps = recipes::mirror_steps(input, &out, direction, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_color_grade(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    preset: ColorGradePreset,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let suffix = match preset {
        ColorGradePreset::Cinematic => "cinematic",
        ColorGradePreset::Warm => "warm",
        ColorGradePreset::Cool => "cool",
        ColorGradePreset::Dramatic => "dramatic",
    };
    let out = default_out(config, input, suffix, "mp4")?;
    let steps = recipes::color_grade_steps(input, &out, preset, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_animated_text(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    text: &str,
    position: TextPosition,
    animation: TextAnimation,
    style: &TextStyle,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "animated-text", "mp4")?;
    
    // For typewriter effect, create ASS subtitle file with karaoke timing
    let ass_file_path = if matches!(animation, crate::model::types::TextAnimation::Typewriter) {
        let ass_file = if let Some(output_dir) = &config.output_dir {
            std::fs::create_dir_all(output_dir)?;
            output_dir.join("typewriter.ass")
        } else {
            let parent = out.parent().unwrap_or_else(|| Path::new("."));
            std::fs::create_dir_all(parent)?;
            parent.join("typewriter.ass")
        };
        
        // Create ASS file with karaoke effect for typewriter
        let mut file = std::fs::File::create(&ass_file)?;
        writeln!(file, "[Script Info]")?;
        writeln!(file, "Title: Typewriter Effect")?;
        writeln!(file, "")?;
        writeln!(file, "[V4+ Styles]")?;
        writeln!(file, "Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding")?;
        writeln!(file, "Style: Default,Arial,{},&H00FFFFFF,&H000000FF,&H00000000,&H80000000,0,0,0,0,100,100,0,0,1,2,0,5,10,10,10,1", style.font_size.unwrap_or(24))?;
        writeln!(file, "")?;
        writeln!(file, "[Events]")?;
        writeln!(file, "Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text")?;
        
        // Calculate position
        let (align, _margin_v) = match position {
            TextPosition::TopLeft | TextPosition::TopCenter | TextPosition::TopRight => (7, 10),
            TextPosition::BottomLeft | TextPosition::BottomCenter | TextPosition::BottomRight => (9, 10),
            TextPosition::Center => (5, 0),
            TextPosition::Custom { .. } => (5, 0),
        };
        
        // Create typewriter effect: multiple dialogue lines, each showing one more character
        // 5 characters per second = 200ms per character
        let chars_per_sec = 5.0;
        let char_duration = 1.0 / chars_per_sec;
        
        for i in 0..=text.chars().count() {
            let partial_text: String = text.chars().take(i).collect();
            if partial_text.is_empty() {
                continue; // Skip empty text
            }
            
            let start_time = (i as f64) * char_duration;
            let end_time = if i < text.chars().count() {
                ((i + 1) as f64) * char_duration
            } else {
                // Show final text for rest of video (use a long duration)
                999.0
            };
            
            // Format time as HH:MM:SS.mm
            let start_min = (start_time as u32) / 60;
            let start_sec = start_time - (start_min as f64 * 60.0);
            let end_min = (end_time as u32) / 60;
            let end_sec = end_time - (end_min as f64 * 60.0);
            
            writeln!(file, "Dialogue: 0,0:{:02}:{:05.2},0:{:02}:{:05.2},Default,,0,0,0,,{{\\an{}}}{}", 
                start_min, start_sec, end_min, end_sec, align, partial_text)?;
        }
        
        file.sync_all()?;
        
        // Convert to absolute path
        Some(ass_file.canonicalize()
            .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(&ass_file)))?)
    } else {
        None
    };
    
    let steps = recipes::animated_text_steps(input, &out, text, &position, animation, style, config.overwrite, ass_file_path.as_deref())?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_transition(
    config: &AppConfig,
    runner: &dyn Runner,
    video1: impl AsRef<Path>,
    video2: impl AsRef<Path>,
    transition_type: TransitionType,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let video1 = video1.as_ref();
    let video2 = video2.as_ref();

    // Validate that both videos exist
    if !video1.exists() {
        anyhow::bail!("First video not found: {}", video1.display());
    }
    if !video2.exists() {
        anyhow::bail!("Second video not found: {}", video2.display());
    }

    let suffix = match transition_type {
        TransitionType::Fade => "fade",
        TransitionType::Wipe => "wipe",
        TransitionType::Slide => "slide",
    };
    let out = default_out(config, video1, &format!("transition-{}", suffix), "mp4")?;
    let steps = recipes::transition_steps(video1, video2, &out, transition_type, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

