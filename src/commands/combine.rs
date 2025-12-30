use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::types::Duration;
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::{Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn handle_loop(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    times: u32,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "loop", "mp4")?;

    // Create concat list file in output directory to ensure it persists
    let list_path = if let Some(output_dir) = &config.output_dir {
        std::fs::create_dir_all(output_dir)?;
        output_dir.join("concat_list.txt")
    } else {
        let parent = out.parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(parent)?;
        parent.join("concat_list.txt")
    };
    
    // Convert to absolute path for FFmpeg
    let list_path = list_path.canonicalize()
        .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(&list_path)))?;
    
    let mut list = std::fs::File::create(&list_path).context("failed to create concat list")?;
    for _ in 0..times {
        // Use absolute paths in concat list file
        let input_path = input.canonicalize()
            .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(input)))?;
        writeln!(list, "file '{}'", input_path.to_string_lossy().replace('\'', "\\'"))
            .context("write concat list")?;
    }
    list.sync_all().context("sync concat list")?;

    let steps = recipes::loop_steps(input, &out, &list_path, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    // _list_file is dropped here, but that's OK - step has completed
    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_merge(
    config: &AppConfig,
    runner: &dyn Runner,
    a: impl AsRef<Path>,
    b: impl AsRef<Path>,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let a = a.as_ref();
    let b = b.as_ref();

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
            a.parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = a.file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_merged.mp4"))
    };

    // Create concat list file in output directory to ensure it persists
    let list_path = if let Some(output_dir) = &config.output_dir {
        std::fs::create_dir_all(output_dir)?;
        output_dir.join("concat_list.txt")
    } else {
        let parent = out.parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(parent)?;
        parent.join("concat_list.txt")
    };
    
    // Convert to absolute path for FFmpeg
    let list_path = list_path.canonicalize()
        .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(&list_path)))?;
    
    let mut list = std::fs::File::create(&list_path).context("failed to create concat list")?;
    // Use absolute paths in concat list file
    let a_path = a.canonicalize()
        .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(a)))?;
    let b_path = b.canonicalize()
        .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(b)))?;
    writeln!(list, "file '{}'", a_path.to_string_lossy().replace('\'', "\\'"))?;
    writeln!(list, "file '{}'", b_path.to_string_lossy().replace('\'', "\\'"))?;
    list.sync_all().context("sync concat list")?;

    let steps = recipes::merge_steps(a, &out, &list_path, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    // _list_file is dropped here, but that's OK - step has completed
    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_compare(
    config: &AppConfig,
    runner: &dyn Runner,
    video1: impl AsRef<Path>,
    video2: impl AsRef<Path>,
    show_psnr: bool,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let video1 = video1.as_ref();
    let video2 = video2.as_ref();

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
            video1.parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = video1.file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_compared.mp4"))
    };

    let steps = recipes::compare_steps_with_metrics(video1, video2, &out, config.overwrite, show_psnr);
    for step in steps {
        runner.run(&step)?;
    }

    if show_psnr {
        eprintln!("Quality metrics saved to psnr.log and ssim.log");
    }
    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_montage(
    config: &AppConfig,
    runner: &dyn Runner,
    videos: &[PathBuf],
    layout: crate::model::types::MontageLayout,
) -> Result<()> {
    ensure_ffmpeg_exists()?;

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
            videos[0].parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = videos[0].file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_montage.mp4"))
    };

    let steps = recipes::montage_steps(videos, &out, &layout, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_crossfade(
    config: &AppConfig,
    runner: &dyn Runner,
    video1: impl AsRef<Path>,
    video2: impl AsRef<Path>,
    duration: Duration,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let video1 = video1.as_ref();
    let video2 = video2.as_ref();

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
            video1.parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = video1.file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_crossfade.mp4"))
    };

    let steps = recipes::crossfade_steps(video1, video2, &out, &duration, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_concat(
    config: &AppConfig,
    runner: &dyn Runner,
    videos: &[PathBuf],
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    if videos.is_empty() {
        anyhow::bail!("Concat requires at least one video file");
    }

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
            videos[0].parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = videos[0].file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_concat.mp4"))
    };

    // Create concat list file in output directory to ensure it persists
    let list_path = if let Some(output_dir) = &config.output_dir {
        std::fs::create_dir_all(output_dir)?;
        output_dir.join("concat_list.txt")
    } else {
        let parent = out.parent().unwrap_or_else(|| Path::new("."));
        std::fs::create_dir_all(parent)?;
        parent.join("concat_list.txt")
    };
    
    // Convert to absolute path for FFmpeg
    let list_path = list_path.canonicalize()
        .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(&list_path)))?;
    
    let mut list = std::fs::File::create(&list_path).context("failed to create concat list")?;
    for video in videos {
        // Use absolute paths in concat list file
        let video_path = video.canonicalize()
            .or_else(|_| std::env::current_dir().map(|cwd| cwd.join(video)))?;
        writeln!(list, "file '{}'", video_path.to_string_lossy().replace('\'', "\\'"))
            .context("write concat list")?;
    }
    list.sync_all().context("sync concat list")?;

    let steps = recipes::concat_steps(&out, &list_path, config.overwrite, &videos[0]);
    for step in steps {
        runner.run(&step)?;
    }

    // _list_file is dropped here, but that's OK - step has completed
    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_sync_cameras(
    config: &AppConfig,
    runner: &dyn Runner,
    videos: &[PathBuf],
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    if videos.len() < 2 {
        anyhow::bail!("Sync cameras requires at least 2 video files");
    }

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
            videos[0].parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = videos[0].file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_synced.mp4"))
    };

    let steps = recipes::sync_cameras_steps(videos, &out, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_collage(
    config: &AppConfig,
    runner: &dyn Runner,
    videos: &[PathBuf],
    layout: crate::model::types::MontageLayout,
) -> Result<()> {
    ensure_ffmpeg_exists()?;

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
            videos[0].parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = videos[0].file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_collage.mp4"))
    };

    let steps = recipes::collage_steps(videos, &out, &layout, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

pub fn handle_slideshow(
    config: &AppConfig,
    runner: &dyn Runner,
    images: &[PathBuf],
    duration: crate::model::types::Duration,
) -> Result<()> {
    ensure_ffmpeg_exists()?;

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
            images[0].parent().unwrap_or_else(|| Path::new("."))
        };
        let stem = images[0].file_stem().unwrap().to_string_lossy();
        dir.join(format!("{stem}_slideshow.mp4"))
    };

    let steps = recipes::slideshow_steps(images, &out, duration, config.overwrite)?;
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

