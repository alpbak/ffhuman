use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::{AudioFormat, AudioSyncDirection, Duration, Time, VolumeAdjustment, SpeedFactor};
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::Result;
use std::path::Path;

pub fn handle_extract_audio(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    format: AudioFormat,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let ext = match format {
        AudioFormat::Mp3 => "mp3",
        AudioFormat::Wav => "wav",
    };
    let out = default_out(config, input, "audio", ext)?;
    let steps = recipes::extract_audio_steps(input, &out, ext, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_mute(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "mute", "mp4")?;
    let steps = recipes::mute_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_add_audio(
    config: &AppConfig,
    runner: &dyn Runner,
    audio: impl AsRef<Path>,
    video: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let audio = audio.as_ref();
    let video = video.as_ref();

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
            video.parent().unwrap_or_else(|| Path::new("."))
        };
        dir.join(format!("{}_audio.mp4", video.file_stem().unwrap().to_string_lossy()))
    };

    let steps = recipes::add_audio_steps(video, audio, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_normalize(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "normalized", "mp4")?;
    let steps = recipes::normalize_steps(input, &out, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_fade(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    fade_in: Option<Duration>,
    fade_out: Option<Duration>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "faded", "mp4")?;
    let fade_in_secs = fade_in.map(|d| d.to_seconds());
    let fade_out_secs = fade_out.map(|d| d.to_seconds());
    let steps = recipes::fade_steps(input, &out, fade_in_secs, fade_out_secs, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_adjust_volume(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    adjustment: VolumeAdjustment,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "volume_adjusted", "mp4")?;
    let steps = recipes::adjust_volume_steps(input, &out, &adjustment, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_sync_audio(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    direction: AudioSyncDirection,
    offset: Duration,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "synced", "mp4")?;
    let steps = recipes::sync_audio_steps(input, &out, &direction, &offset, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}


pub fn handle_mix_audio(
    config: &AppConfig,
    runner: &dyn Runner,
    audio1: impl AsRef<Path>,
    audio2: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let audio1 = audio1.as_ref();
    let audio2 = audio2.as_ref();

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
            audio1.parent().unwrap_or_else(|| Path::new("."))
        };
        dir.join(format!("{}_mixed.mp3", audio1.file_stem().unwrap().to_string_lossy()))
    };

    let steps = recipes::mix_audio_steps(audio1, audio2, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_extract_audio_range(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    start: Time,
    end: Time,
    format: AudioFormat,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let ext = match format {
        AudioFormat::Mp3 => "mp3",
        AudioFormat::Wav => "wav",
    };
    let out = default_out(config, input, "audio_range", ext)?;
    let steps = recipes::extract_audio_range_steps(input, &out, &start, &end, ext, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_noise_reduction(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "noise_reduced", "mp4")?;
    let steps = recipes::noise_reduction_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_echo_removal(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "echo_removed", "mp4")?;
    let steps = recipes::echo_removal_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_audio_ducking(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "audio_ducked", "mp4")?;
    let steps = recipes::audio_ducking_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_audio_equalizer(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    bass: Option<i32>,
    treble: Option<i32>,
    mid: Option<i32>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    // Validate that at least one adjustment is provided
    if bass.is_none() && treble.is_none() && mid.is_none() {
        anyhow::bail!("Audio equalizer requires at least one of --bass, --treble, or --mid");
    }

    // Validate ranges
    if let Some(b) = bass {
        if b < -20 || b > 20 {
            anyhow::bail!("Bass adjustment must be between -20 and +20");
        }
    }
    if let Some(t) = treble {
        if t < -20 || t > 20 {
            anyhow::bail!("Treble adjustment must be between -20 and +20");
        }
    }
    if let Some(m) = mid {
        if m < -20 || m > 20 {
            anyhow::bail!("Mid adjustment must be between -20 and +20");
        }
    }

    let out = default_out(config, input, "equalized", "mp4")?;
    let steps = recipes::audio_equalizer_steps(input, &out, bass, treble, mid, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_voice_isolation(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "voice_isolated", "mp4")?;
    let steps = recipes::voice_isolation_steps(input, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_audio_speed_keep_pitch(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    factor: SpeedFactor,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "audio_speed", "mp4")?;
    let steps = recipes::audio_speed_keep_pitch_steps(input, &out, factor.factor, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

