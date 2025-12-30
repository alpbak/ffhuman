use crate::model::*;
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ffhuman", about = "FFmpeg for humans")]
pub struct Cli {
    /// Print generated ffmpeg commands, do not execute
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Explain what we do (prints chosen recipe details)
    #[arg(long, global = true)]
    pub explain: bool,

    /// Overwrite output files
    #[arg(long, short = 'y', global = true)]
    pub overwrite: bool,

    /// Output path override (full path to output file)
    #[arg(long, global = true)]
    pub out: Option<PathBuf>,

    /// Output directory (folder where output files will be saved)
    #[arg(long, global = true)]
    pub output_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// convert video.mp4 to gif
    ///
    /// Convert video or audio files to different formats.
    ///
    /// Examples:
    ///   convert video.mp4 to gif
    ///   convert video.mp4 to mp4
    ///   convert video.mp4 to webm quality high
    ///   convert video.mp4 to mp3
    ///   convert video.mp4 to wav
    ///   convert video.mp4 to iphone
    ///   convert video.mp4 to android
    ///   convert video.mp4 to hls
    ///   convert video.mp4 to dash
    Convert {
        #[arg(help = "Input video or audio file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target format: gif, mp4, webm, mp3, wav, iphone, android, hls, dash, or 360")]
        format: ConvertFormatCli,
        #[arg(long, help = "Quality preset: low, medium, high, or ultra")]
        quality: Option<String>,
        #[arg(long, help = "Video codec: h264, h265, vp9, or copy")]
        codec: Option<String>,
    },

    /// compress video.mp4 to 10mb  OR compress video.mp4 to high-quality
    ///
    /// Compress video to a target file size or quality preset.
    ///
    /// Examples:
    ///   compress video.mp4 to 10mb
    ///   compress video.mp4 to 800k
    ///   compress video.mp4 to high-quality
    ///   compress video.mp4 to low-quality
    ///   compress video.mp4 to 10mb --two-pass
    Compress {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target: size (e.g., 10mb, 800k) or quality preset (low-quality, medium-quality, high-quality, ultra-quality)")]
        target: String,
        #[arg(long, help = "Use two-pass encoding for more accurate size targeting")]
        two_pass: bool,
    },

    /// trim video.mp4 from 0:30 to 1:00
    ///
    /// Trim video to a specific time range.
    ///
    /// Examples:
    ///   trim video.mp4 from 30 to 60
    ///   trim video.mp4 from 0:30 to 1:00
    ///   trim video.mp4 from 1:05:30 to 2:10:45
    Trim {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "from")]
        _from: KeywordFrom,
        #[arg(help = "Start time: SS, M:SS, or H:MM:SS format (e.g., 30, 0:30, 1:05:30)")]
        start: String,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "End time: SS, M:SS, or H:MM:SS format (e.g., 60, 1:00, 2:10:45)")]
        end: String,
    },

    /// extract-audio video.mp4
    ///
    ///
    /// Examples:
    ///   extract-audio video.mp4
    #[command(name = "extract-audio")]
    Extract {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// resize video.mp4 to 1280x720  OR resize video.mp4 to 720p
    ///
    /// Resize video to specific dimensions or preset resolution.
    ///
    /// Examples:
    ///   resize video.mp4 to 1280x720
    ///   resize video.mp4 to 720p
    ///   resize video.mp4 to 1080p
    ///   resize video.mp4 to 4k
    Resize {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target size: WxH (e.g., 1280x720) or preset (720p, 1080p, 4k)")]
        target: String,
    },

    /// speed-up video.mp4 by 2x
    ///
    /// Speed up video playback by a factor.
    ///
    /// Examples:
    ///   speed-up video.mp4 by 2x
    ///   speed-up video.mp4 by 1.5x
    #[command(name = "speed-up")]
    SpeedUp {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "by")]
        _by: KeywordBy,
        #[arg(help = "Speed factor: must be positive number followed by 'x' (e.g., 2x, 1.5x)")]
        factor: String,
    },

    /// slow-down video.mp4 by 2x
    ///
    /// Slow down video playback by a factor.
    ///
    /// Examples:
    ///   slow-down video.mp4 by 2x
    ///   slow-down video.mp4 by 0.5x
    #[command(name = "slow-down")]
    SlowDown {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "by")]
        _by: KeywordBy,
        #[arg(help = "Slowdown factor: must be positive number followed by 'x' (e.g., 2x, 0.5x)")]
        factor: String,
    },

    /// reverse video.mp4
    ///
    /// Reverse video playback (play backwards).
    Reverse {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// mute video.mp4
    ///
    /// Remove audio track from video.
    Mute {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// rotate video.mp4 by 90
    ///
    /// Rotate video by 90, 180, or 270 degrees.
    ///
    /// Examples:
    ///   rotate video.mp4 by 90
    ///   rotate video.mp4 by 180
    ///   rotate video.mp4 by 270
    Rotate {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "by")]
        _by: KeywordBy,
        #[arg(help = "Rotation degrees: 90, 180, or 270")]
        degrees: i32,
    },

    /// flip video.mp4 horizontal
    ///
    /// Flip video horizontally or vertically.
    ///
    /// Examples:
    ///   flip video.mp4 horizontal
    ///   flip video.mp4 vertical
    Flip {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Flip direction: horizontal or vertical")]
        direction: FlipDirCli,
    },

    /// thumbnail video.mp4 at 0:05
    ///
    /// Extract a single frame as a thumbnail image.
    ///
    /// Examples:
    ///   thumbnail video.mp4 at 5
    ///   thumbnail video.mp4 at 0:05
    ///   thumbnail video.mp4 at 1:05:30
    Thumbnail {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "at")]
        _at: KeywordAt,
        #[arg(help = "Time position: SS, M:SS, or H:MM:SS format (e.g., 5, 0:05, 1:05:30)")]
        time: String,
    },

    /// crop video.mp4 to 640x480
    ///
    /// Crop video to specific dimensions (centered crop).
    ///
    /// Examples:
    ///   crop video.mp4 to 640x480
    ///   crop video.mp4 to 1920x1080
    Crop {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Crop size: WxH format (e.g., 640x480)")]
        size: String,
    },

    /// fps video.mp4 to 30
    ///
    /// Change video frame rate.
    ///
    /// Examples:
    ///   fps video.mp4 to 30
    ///   fps video.mp4 to 60
    Fps {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target frames per second (e.g., 30, 60)")]
        fps: u32,
    },

    /// loop video.mp4 3 times
    ///
    /// Loop video multiple times.
    ///
    /// Examples:
    ///   loop video.mp4 3 times
    ///   loop video.mp4 5 times
    Loop {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Number of times to loop (must be >= 2)")]
        times: u32,
        #[arg(value_name = "times")]
        _times_kw: KeywordTimes,
    },

    /// merge a.mp4 and b.mp4
    ///
    /// Merge two videos sequentially.
    ///
    /// Examples:
    ///   merge video1.mp4 and video2.mp4
    Merge {
        #[arg(help = "First video file")]
        a: PathBuf,
        #[arg(value_name = "and")]
        _and: KeywordAnd,
        #[arg(help = "Second video file")]
        b: PathBuf,
    },

    /// add audio.mp3 to video.mp4
    ///
    /// Replace or add audio track to video.
    ///
    /// Examples:
    ///   add audio.mp3 to video.mp4
    ///   add music.wav to video.mp4
    AddAudio {
        #[arg(help = "Input audio file")]
        audio: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Input video file")]
        video: PathBuf,
    },

    /// video.mp4 grayscale  (kept as "grayscale video.mp4" for sane parsing)
    ///
    /// Convert video to grayscale.
    Grayscale {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// stabilize video.mp4
    ///
    /// Stabilize shaky video using video stabilization algorithm.
    Stabilize {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// denoise video.mp4
    ///
    /// Reduce video noise using denoising filter.
    Denoise {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// watermark video.mp4 logo.png at top-right --opacity 0.5 --size 20%
    ///
    /// Add a logo/image overlay to a video with configurable position, opacity, and size.
    ///
    /// Examples:
    ///   watermark video.mp4 logo.png at top-right
    ///   watermark video.mp4 logo.png at top-right --opacity 0.5
    ///   watermark video.mp4 logo.png at bottom-left --size 20%
    ///   watermark video.mp4 logo.png at 100,50 --opacity 0.8 --size 200x100
    Watermark {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Logo/image file to overlay (PNG, JPG, etc.)")]
        logo: PathBuf,
        #[arg(value_name = "at")]
        _at: KeywordAt,
        #[arg(help = "Position: top-left, top-right, bottom-left, bottom-right, or x,y coordinates (e.g., 100,50)")]
        position: String,
        #[arg(long, help = "Opacity value (0.0 to 1.0, default: 1.0). 0.0 is transparent, 1.0 is fully opaque")]
        opacity: Option<f64>,
        #[arg(long, help = "Size: percentage (e.g., 20% or 0.2) or pixels (e.g., 200x100 or 200). If only width is specified, aspect ratio is maintained")]
        size: Option<String>,
    },

    /// add-text video.mp4 "My Video" at bottom-center
    ///
    /// Add text overlay to video with configurable position, font, size, and color.
    ///
    /// Examples:
    ///   add-text video.mp4 "My Video" at bottom-center
    ///   add-text video.mp4 "Title" at top-left --font-size 48 --color red
    ///   add-text video.mp4 "Watermark" at top-right --color "#FFFFFF" --font-size 32
    ///   add-text video.mp4 "Timestamp" at bottom-center --timestamp
    AddText {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Text to overlay on video")]
        text: String,
        #[arg(value_name = "at")]
        _at: KeywordAt,
        #[arg(help = "Position: top-left, top-right, top-center, bottom-left, bottom-right, bottom-center, center, or x,y coordinates")]
        position: String,
        #[arg(long, help = "Font size in pixels (default: 24)")]
        font_size: Option<u32>,
        #[arg(long, help = "Text color: named color (white, black, red, etc.) or hex (#FFFFFF or FFFFFF), default: white")]
        color: Option<String>,
        #[arg(long, help = "Show timestamp instead of custom text")]
        timestamp: bool,
    },

    /// filter video.mp4 --brightness 10 --contrast 5
    ///
    /// Apply video filters: brightness/contrast/saturation adjustments or color grading presets.
    ///
    /// Examples:
    ///   filter video.mp4 --brightness 10 --contrast 5
    ///   filter video.mp4 --brightness -5 --saturation 20
    ///   filter video.mp4 --preset vintage
    ///   filter video.mp4 --preset black-and-white
    ///   filter video.mp4 --preset sepia
    Filter {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Brightness adjustment: -100 to 100 (default: 0)")]
        brightness: Option<f64>,
        #[arg(long, help = "Contrast adjustment: -100 to 100 (default: 0)")]
        contrast: Option<f64>,
        #[arg(long, help = "Saturation adjustment: -100 to 100 (default: 0)")]
        saturation: Option<f64>,
        #[arg(long, help = "Color grading preset: vintage, black-and-white, or sepia")]
        preset: Option<String>,
    },

    /// blur video.mp4 region 100,100,200,200
    ///
    /// Blur faces or regions in video for privacy.
    ///
    /// Examples:
    ///   blur video.mp4 region 100,100,200,200
    Blur {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Blur type: region")]
        blur_type: BlurTypeCli,
        #[arg(help = "Region coordinates: x,y,width,height (e.g., 100,100,200,200)")]
        region: String,
    },

    /// normalize video.mp4
    ///
    /// Normalize audio levels in video.
    ///
    /// Examples:
    ///   normalize video.mp4
    Normalize {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// adjust-volume video.mp4 to 50%  OR adjust-volume video.mp4 by +10db
    ///
    /// Adjust audio volume by percentage or decibels.
    ///
    /// Examples:
    ///   adjust-volume video.mp4 to 50%
    ///   adjust-volume video.mp4 by +10db
    ///   adjust-volume video.mp4 by -5db
    #[command(name = "adjust-volume")]
    AdjustVolume {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: Option<KeywordTo>,
        #[arg(value_name = "by")]
        _by: Option<KeywordBy>,
        #[arg(help = "Volume adjustment: percentage (e.g., 50%) or decibels (e.g., +10db, -5db)")]
        adjustment: String,
    },

    /// sync-audio video.mp4 delay 0.5s  OR sync-audio video.mp4 advance 0.3s
    ///
    /// Fix audio/video sync issues by delaying or advancing audio.
    ///
    /// Examples:
    ///   sync-audio video.mp4 delay 0.5s
    ///   sync-audio video.mp4 advance 0.3s
    #[command(name = "sync-audio")]
    SyncAudio {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Sync direction: delay or advance")]
        direction: String,
        #[arg(help = "Time offset (e.g., 0.5s, 0.3s)")]
        offset: String,
    },


    /// mix-audio audio.mp3 and audio2.mp3
    ///
    /// Combine multiple audio files into one.
    ///
    /// Examples:
    ///   mix-audio audio.mp3 and audio2.mp3
    #[command(name = "mix-audio")]
    MixAudio {
        #[arg(help = "First audio file")]
        audio1: PathBuf,
        #[arg(value_name = "and")]
        _and: KeywordAnd,
        #[arg(help = "Second audio file")]
        audio2: PathBuf,
    },

    /// extract-audio-range video.mp4 from 0:30 to 2:00
    ///
    /// Extract audio from a specific time range.
    ///
    /// Examples:
    ///   extract-audio-range video.mp4 from 0:30 to 2:00
    #[command(name = "extract-audio-range")]
    ExtractAudioRange {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "from")]
        _from2: KeywordFrom,
        #[arg(help = "Start time: SS, M:SS, or H:MM:SS format")]
        start: String,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "End time: SS, M:SS, or H:MM:SS format")]
        end: String,
    },

    /// fade video.mp4 in 2s out 2s
    ///
    /// Apply fade in/out effects to audio in video.
    ///
    /// Examples:
    ///   fade video.mp4 in 2s out 2s
    ///   fade video.mp4 in 1.5s
    ///   fade video.mp4 out 3s
    Fade {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Fade in duration (e.g., 2s, 1.5s)")]
        fade_in: Option<String>,
        #[arg(long, help = "Fade out duration (e.g., 2s, 1.5s)")]
        fade_out: Option<String>,
    },

    /// split video.mp4 every 30s  OR split video.mp4 into 3 parts
    ///
    /// Split video into segments by time interval or into equal parts.
    ///
    /// Examples:
    ///   split video.mp4 every 30s
    ///   split video.mp4 into 3 parts
    Split {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Split mode: 'every <duration>' or 'into <N> parts'")]
        mode: String,
    },

    /// extract-frames video.mp4 every 1s
    ///
    /// Extract frames from video at specified intervals.
    ///
    /// Examples:
    ///   extract-frames video.mp4 every 1s
    ///   extract-frames video.mp4 every 0.5s
    ExtractFrames {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "every")]
        _every: KeywordEvery,
        #[arg(help = "Interval duration (e.g., 1s, 0.5s)")]
        interval: String,
    },

    /// burn-subtitle video.mp4 subtitle.srt
    ///
    /// Burn subtitles into video.
    ///
    /// Examples:
    ///   burn-subtitle video.mp4 subtitle.srt
    BurnSubtitle {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Subtitle file (SRT, ASS, etc.)")]
        subtitle: PathBuf,
    },

    /// batch convert *.mp4 to gif
    ///
    /// Process multiple files with the same operation.
    ///
    /// Examples:
    ///   batch convert *.mp4 to gif
    ///   batch convert *.mp4 to gif --if duration < 30s
    Batch {
        #[arg(help = "Operation: convert")]
        operation: BatchOperationCli,
        #[arg(help = "File pattern (e.g., *.mp4)")]
        pattern: String,
        #[arg(value_name = "to")]
        _to: Option<KeywordTo>,
        #[arg(help = "Target format (for convert operation): gif, mp4, webm, mp3, or wav")]
        format: Option<ConvertFormatCli>,
        #[arg(long, help = "Conditional processing: --if duration < 30s")]
        r#if: Option<String>,
    },

    /// compare video1.mp4 and video2.mp4
    ///
    /// Create side-by-side comparison of two videos.
    ///
    /// Examples:
    ///   compare video1.mp4 and video2.mp4
    ///   compare video1.mp4 and video2.mp4 --show-psnr
    Compare {
        #[arg(help = "First video file")]
        video1: PathBuf,
        #[arg(value_name = "and")]
        _and: KeywordAnd,
        #[arg(help = "Second video file")]
        video2: PathBuf,
        #[arg(long, help = "Show quality metrics (PSNR, SSIM)")]
        show_psnr: bool,
    },

    /// set-metadata video.mp4 title "My Video"
    ///
    /// Edit video metadata (title, author, copyright, etc.).
    ///
    /// Examples:
    ///   set-metadata video.mp4 title "My Video"
    ///   set-metadata video.mp4 author "John Doe"
    ///   set-metadata video.mp4 copyright "2024"
    SetMetadata {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Metadata field: title, author, copyright, comment, or description")]
        field: String,
        #[arg(help = "Metadata value")]
        value: String,
    },

    /// montage layout 2x2 video1.mp4 video2.mp4 video3.mp4 video4.mp4
    ///
    /// Create grid/collage of multiple videos.
    ///
    /// Examples:
    ///   montage layout 2x2 video1.mp4 video2.mp4 video3.mp4 video4.mp4
    ///   montage layout 3x1 video1.mp4 video2.mp4 video3.mp4
    Montage {
        #[arg(value_name = "layout")]
        _layout_kw: KeywordLayout,
        #[arg(help = "Grid layout: WxH format (e.g., 2x2, 3x1)")]
        layout: String,
        #[arg(help = "Input video files (2 or more)")]
        videos: Vec<PathBuf>,
    },

    /// crossfade video1.mp4 and video2.mp4 duration 2s
    ///
    /// Create smooth transitions between videos.
    ///
    /// Examples:
    ///   crossfade video1.mp4 and video2.mp4 duration 2s
    ///   crossfade video1.mp4 and video2.mp4 duration 1.5s
    Crossfade {
        #[arg(help = "First video file")]
        video1: PathBuf,
        #[arg(value_name = "and")]
        _and: KeywordAnd,
        #[arg(help = "Second video file")]
        video2: PathBuf,
        #[arg(value_name = "duration")]
        _duration_kw: KeywordDuration,
        #[arg(help = "Crossfade duration (e.g., 2s, 1.5s)")]
        duration: String,
    },

    /// timelapse video.mp4 speed 10x
    ///
    /// Create time-lapse video by speeding up significantly.
    ///
    /// Examples:
    ///   timelapse video.mp4 speed 10x
    ///   timelapse video.mp4 speed 100x
    Timelapse {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "speed")]
        _speed_kw: KeywordSpeed,
        #[arg(help = "Speed factor: must be positive number followed by 'x' (e.g., 10x, 100x)")]
        speed: String,
    },

    /// pip video1.mp4 on video2.mp4 at top-right
    ///
    /// Overlay one video on another (picture-in-picture).
    ///
    /// Examples:
    ///   pip video1.mp4 on video2.mp4 at top-right
    ///   pip video1.mp4 on video2.mp4 at bottom-left
    Pip {
        #[arg(help = "Video to overlay (smaller video)")]
        overlay_video: PathBuf,
        #[arg(value_name = "on")]
        _on: KeywordOn,
        #[arg(help = "Base video (background video)")]
        base_video: PathBuf,
        #[arg(value_name = "at")]
        _at: KeywordAt,
        #[arg(help = "Position: top-left, top-right, bottom-left, bottom-right, or center")]
        position: String,
    },

    /// remove-background video.mp4 color green
    ///
    /// Remove backgrounds using chroma key (green screen / blue screen).
    ///
    /// Examples:
    ///   remove-background video.mp4 color green
    ///   remove-background video.mp4 color blue
    ///   remove-background video.mp4 color #00FF00
    #[command(name = "remove-background")]
    RemoveBackground {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "color")]
        _color_kw: KeywordColor,
        #[arg(help = "Chroma key color: green, blue, or hex color (e.g., #00FF00)")]
        color: String,
    },

    /// overlay video1.mp4 on video2.mp4 at 100,50 opacity 0.7
    ///
    /// Overlay one video on another with transparency.
    ///
    /// Examples:
    ///   overlay video1.mp4 on video2.mp4 at 100,50 opacity 0.7
    ///   overlay video1.mp4 on video2.mp4 at top-right opacity 0.5
    Overlay {
        #[arg(help = "Video to overlay")]
        overlay_video: PathBuf,
        #[arg(value_name = "on")]
        _on: KeywordOn,
        #[arg(help = "Base video (background)")]
        base_video: PathBuf,
        #[arg(value_name = "at")]
        _at: KeywordAt,
        #[arg(help = "Position: top-left, top-right, bottom-left, bottom-right, or x,y coordinates")]
        position: String,
        #[arg(long, help = "Opacity value (0.0 to 1.0, default: 1.0)")]
        opacity: Option<f64>,
    },

    /// concat video1.mp4 video2.mp4 video3.mp4
    ///
    /// Concatenate multiple videos without re-encoding (faster than merge).
    ///
    /// Examples:
    ///   concat video1.mp4 video2.mp4 video3.mp4
    Concat {
        #[arg(help = "Input video files (2 or more)")]
        videos: Vec<PathBuf>,
    },

    /// detect-scenes video.mp4
    ///
    /// Find scene changes automatically in video.
    ///
    /// Examples:
    ///   detect-scenes video.mp4
    #[command(name = "detect-scenes")]
    DetectScenes {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// detect-black video.mp4
    ///
    /// Find and optionally remove black frames.
    ///
    /// Examples:
    ///   detect-black video.mp4
    #[command(name = "detect-black")]
    DetectBlack {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },


    /// fix-rotation video.mp4
    ///
    /// Auto-detect and fix video orientation.
    ///
    /// Examples:
    ///   fix-rotation video.mp4
    #[command(name = "fix-rotation")]
    FixRotation {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// analyze-quality video.mp4
    ///
    /// Analyze video quality metrics (bitrate, resolution, codec, etc.).
    ///
    /// Examples:
    ///   analyze-quality video.mp4
    #[command(name = "analyze-quality")]
    AnalyzeQuality {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// preview video.mp4
    ///
    /// Generate quick preview (first 10 seconds, low quality).
    ///
    /// Examples:
    ///   preview video.mp4
    Preview {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// suggest-format video.mp4
    ///
    /// Recommend best format based on content.
    ///
    /// Examples:
    ///   suggest-format video.mp4
    #[command(name = "suggest-format")]
    Suggest {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// ffhuman workflow process.yaml
    ///
    /// Define multi-step operations in a config file.
    ///
    /// Examples:
    ///   workflow process.yaml
    Workflow {
        #[arg(help = "Workflow configuration file (YAML)")]
        config_file: PathBuf,
    },

    /// motion-blur video.mp4 [--radius N]
    ///
    /// Add motion blur effect to video.
    ///
    /// Examples:
    ///   motion-blur video.mp4
    ///   motion-blur video.mp4 --radius 5
    MotionBlur {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Blur radius (number of frames to mix, default: 3, higher = more blur)")]
        radius: Option<u32>,
    },

    /// vignette video.mp4 [--intensity N] [--size N]
    ///
    /// Add vignette effect to video.
    ///
    /// Examples:
    ///   vignette video.mp4
    ///   vignette video.mp4 --intensity 0.7 --size 0.6
    Vignette {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Vignette intensity (0.0-1.0, default: 0.5, higher = darker edges)")]
        intensity: Option<f32>,
        #[arg(long, help = "Vignette size (0.0-1.0, default: 0.7, higher = larger bright center)")]
        size: Option<f32>,
    },

    /// lens-correct video.mp4
    ///
    /// Fix lens distortion in video.
    ///
    /// Examples:
    ///   lens-correct video.mp4
    LensCorrect {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// interpolate video.mp4 to 60fps
    ///
    /// Generate intermediate frames for smooth slow-motion (frame interpolation).
    ///
    /// Examples:
    ///   interpolate video.mp4 to 60fps
    ///   interpolate video.mp4 to 120fps
    Interpolate {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target frames per second (e.g., 60, 120)")]
        fps: u32,
    },

    /// Check system details and FFmpeg status
    Doctor,

    /// extract-metadata video.mp4
    ///
    /// Extract all metadata from video to JSON or XML.
    ///
    /// Examples:
    ///   extract-metadata video.mp4
    ///   extract-metadata video.mp4 --format json
    ///   extract-metadata video.mp4 --format xml
    #[command(name = "extract-metadata")]
    ExtractMetadata {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Output format: json or xml (default: json)")]
        format: Option<String>,
    },

    /// info video.mp4
    ///
    /// Display human-readable summary of video properties.
    ///
    /// Examples:
    ///   info video.mp4
    Info {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// thumbnails video.mp4 3x3
    ///
    /// Generate a grid of thumbnails from video.
    ///
    /// Examples:
    ///   thumbnails video.mp4 3x3
    ///   thumbnails video.mp4 2x2
    Thumbnails {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Grid layout: WxH format (e.g., 3x3, 2x2)")]
        layout: String,
    },

    /// convert video.mp4 to instagram
    ///
    /// Convert video optimized for social media platforms.
    ///
    /// Examples:
    ///   convert video.mp4 to instagram
    ///   convert video.mp4 to tiktok
    ///   convert video.mp4 to youtube-shorts
    ///   convert video.mp4 to twitter
    SocialConvert {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Platform: instagram, tiktok, youtube-shorts, or twitter")]
        platform: String,
    },

    /// social-crop video.mp4 square
    ///
    /// Quick aspect ratio fixes for social media.
    ///
    /// Examples:
    ///   social-crop video.mp4 square
    ///   social-crop video.mp4 circle
    #[command(name = "social-crop")]
    SocialCrop {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Crop shape: square or circle")]
        shape: SocialCropShapeCli,
    },

    /// convert to vertical video.mp4
    ///
    /// Convert horizontal video to vertical (9:16 aspect ratio).
    ///
    /// Examples:
    ///   convert to vertical video.mp4
    ///   convert to portrait video.mp4
    VerticalConvert {
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target: vertical or portrait")]
        _target: KeywordVertical,
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// convert video.mp4 to story
    ///
    /// Convert video to story format (9:16, 15 seconds max, optimized encoding).
    ///
    /// Examples:
    ///   convert video.mp4 to story
    StoryConvert {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target: story")]
        _story: KeywordStory,
    },

    /// reduce-noise video.mp4
    ///
    /// Remove background noise from audio.
    ///
    /// Examples:
    ///   reduce-noise video.mp4
    #[command(name = "reduce-noise")]
    ReduceNoise {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// remove-echo video.mp4
    ///
    /// Clean up audio with echo/reverb.
    ///
    /// Examples:
    ///   remove-echo video.mp4
    #[command(name = "remove-echo")]
    RemoveEcho {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// duck-audio video.mp4 when voice detected
    ///
    /// Lower background music when speech is detected.
    ///
    /// Examples:
    ///   duck-audio video.mp4 when voice detected
    #[command(name = "duck-audio")]
    DuckAudio {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "when")]
        _when: KeywordWhen,
        #[arg(help = "Condition: voice detected")]
        _condition: KeywordVoiceDetected,
    },

    /// equalize-audio video.mp4 --bass +5 --treble -2
    ///
    /// Adjust frequency bands in audio.
    ///
    /// Examples:
    ///   equalize-audio video.mp4 --bass +5 --treble -2
    ///   equalize-audio video.mp4 --bass +10 --mid -3
    #[command(name = "equalize-audio")]
    EqualizeAudio {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Bass adjustment: -20 to +20 (default: 0)")]
        bass: Option<i32>,
        #[arg(long, help = "Treble adjustment: -20 to +20 (default: 0)")]
        treble: Option<i32>,
        #[arg(long, help = "Mid adjustment: -20 to +20 (default: 0)")]
        mid: Option<i32>,
    },

    /// isolate-voice video.mp4
    ///
    /// Extract/isolate voice from background.
    ///
    /// Examples:
    ///   isolate-voice video.mp4
    #[command(name = "isolate-voice")]
    IsolateVoice {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// speed-audio video.mp4 by 1.5x --keep-pitch
    ///
    /// Time-stretch audio without pitch shift.
    ///
    /// Examples:
    ///   speed-audio video.mp4 by 1.5x --keep-pitch
    ///   speed-audio video.mp4 by 2x --keep-pitch
    #[command(name = "speed-audio")]
    SpeedAudio {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "by")]
        _by: KeywordBy,
        #[arg(help = "Speed factor: must be positive number followed by 'x' (e.g., 1.5x, 2x)")]
        factor: String,
        #[arg(long, help = "Keep original pitch (time-stretch without pitch shift)")]
        keep_pitch: bool,
    },

    /// glitch video.mp4 [--shift N] [--noise N]
    ///
    /// Apply digital glitch effects to video.
    ///
    /// Examples:
    ///   glitch video.mp4
    ///   glitch video.mp4 --shift 8 --noise 50
    Glitch {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "RGB channel shift amount in pixels (default: 3, max: 15, higher = more visible separation)")]
        shift: Option<u32>,
        #[arg(long, help = "Noise intensity (default: 30, max: 100, higher = more digital artifacts)")]
        noise: Option<u32>,
    },

    /// vintage-film video.mp4 [--era 70s|80s|90s]
    ///
    /// Apply old film effect with grain, scratches, and color grading.
    ///
    /// Examples:
    ///   vintage-film video.mp4
    ///   vintage-film video.mp4 --era 70s
    ///   vintage-film video.mp4 --era 80s
    #[command(name = "vintage-film")]
    VintageFilm {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Film era: 70s, 80s, 90s, or classic (default: classic)")]
        era: Option<String>,
    },

    /// split-screen video1.mp4 and video2.mp4
    ///
    /// Create side-by-side or top/bottom split screen.
    ///
    /// Examples:
    ///   split-screen video1.mp4 and video2.mp4
    ///   split-screen video1.mp4 and video2.mp4 --orientation horizontal
    SplitScreen {
        #[arg(help = "First video file")]
        video1: PathBuf,
        #[arg(value_name = "and")]
        _and: KeywordAnd,
        #[arg(help = "Second video file")]
        video2: PathBuf,
        #[arg(long, help = "Orientation: horizontal (side-by-side) or vertical (top/bottom), default: horizontal")]
        orientation: Option<String>,
    },

    /// mirror video.mp4 horizontal
    ///
    /// Mirror/flip video horizontally or vertically.
    ///
    /// Examples:
    ///   mirror video.mp4 horizontal
    ///   mirror video.mp4 vertical
    Mirror {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Mirror direction: horizontal or vertical")]
        direction: String,
    },

    /// color-grade video.mp4 --preset cinematic
    ///
    /// Apply advanced color grading with presets.
    ///
    /// Examples:
    ///   color-grade video.mp4 --preset cinematic
    ///   color-grade video.mp4 --preset warm
    ColorGrade {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(long, help = "Color grading preset: cinematic, warm, cool, or dramatic")]
        preset: String,
    },

    /// add-text video.mp4 "Title" at center --animate fade-in
    ///
    /// Add animated text overlays with fade, slide, or typewriter effects.
    ///
    /// Examples:
    ///   add-text video.mp4 "Title" at center --animate fade-in
    ///   add-text video.mp4 "Subtitle" at bottom-center --animate slide-in
    AnimatedText {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Text to overlay on video")]
        text: String,
        #[arg(value_name = "at")]
        _at: KeywordAt,
        #[arg(help = "Position: top-left, top-right, top-center, bottom-left, bottom-right, bottom-center, center, or x,y coordinates")]
        position: String,
        #[arg(long, help = "Animation type: fade-in, slide-in, or typewriter")]
        animate: Option<String>,
        #[arg(long, help = "Font size in pixels (default: 24)")]
        font_size: Option<u32>,
        #[arg(long, help = "Text color: named color (white, black, red, etc.) or hex (#FFFFFF or FFFFFF), default: white")]
        color: Option<String>,
    },

    /// transition video1.mp4 to video2.mp4 --type fade
    ///
    /// Create transitions between videos (fade, wipe, slide).
    ///
    /// Examples:
    ///   transition video1.mp4 to video2.mp4 --type fade
    ///   transition video1.mp4 to video2.mp4 --type wipe
    Transition {
        #[arg(help = "First video file")]
        video1: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Second video file")]
        video2: PathBuf,
        #[arg(long, help = "Transition type: fade, wipe, or slide")]
        r#type: String,
    },

    /// sync-cameras video1.mp4 video2.mp4 video3.mp4
    ///
    /// Sync multiple camera angles by audio.
    ///
    /// Examples:
    ///   sync-cameras video1.mp4 video2.mp4 video3.mp4
    #[command(name = "sync-cameras")]
    SyncCameras {
        #[arg(help = "Input video files (2 or more)")]
        videos: Vec<PathBuf>,
    },

    /// generate-test-pattern 1080p 10s
    ///
    /// Generate test patterns.
    ///
    /// Examples:
    ///   generate-test-pattern 1080p 10s
    ///   generate-test-pattern 720p 5s
    #[command(name = "generate-test-pattern")]
    Generate {
        #[arg(help = "Resolution: 720p, 1080p, 4k, or WxH format")]
        resolution: String,
        #[arg(help = "Duration (e.g., 10s, 5s)")]
        duration: String,
    },

    /// add-timecode video.mp4
    ///
    /// Burn timecode overlay.
    ///
    /// Examples:
    ///   add-timecode video.mp4
    #[command(name = "add-timecode")]
    AddTimecode {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// proxy video.mp4
    ///
    /// Generate low-res proxies for editing.
    ///
    /// Examples:
    ///   proxy video.mp4
    Proxy {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// export-edl video.mp4
    ///
    /// Export edit decision list.
    ///
    /// Examples:
    ///   export-edl video.mp4
    #[command(name = "export-edl")]
    Export {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// convert-colorspace video.mp4 to rec709
    ///
    /// Convert between color spaces.
    ///
    /// Examples:
    ///   convert-colorspace video.mp4 to rec709
    ///   convert-colorspace video.mp4 to rec2020
    #[command(name = "convert-colorspace")]
    ConvertColorspace {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target color space: rec709, rec2020, p3, or srgb")]
        target: String,
    },

    /// detect-silence video.mp4
    ///
    /// Find silent segments.
    ///
    /// Examples:
    ///   detect-silence video.mp4
    #[command(name = "detect-silence")]
    DetectSilence {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// analyze-loudness video.mp4
    ///
    /// LUFS measurement (broadcast standards).
    ///
    /// Examples:
    ///   analyze-loudness video.mp4
    #[command(name = "analyze-loudness")]
    AnalyzeLoudness {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// detect-duplicates video.mp4
    ///
    /// Find duplicate/repeated frames.
    ///
    /// Examples:
    ///   detect-duplicates video.mp4
    #[command(name = "detect-duplicates")]
    DetectDuplicates {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// collage layout 2x2 video1.mp4 video2.mp4 video3.mp4
    ///
    /// Create video collage with multiple videos in a grid layout.
    ///
    /// Examples:
    ///   collage layout 2x2 video1.mp4 video2.mp4 video3.mp4
    ///   collage layout 2x1 video1.mp4 video2.mp4
    Collage {
        #[arg(value_name = "layout")]
        _layout_kw: KeywordLayout,
        #[arg(help = "Grid layout: WxH format (e.g., 2x2, 3x1)")]
        layout: String,
        #[arg(help = "Input video files (2 or more)")]
        videos: Vec<PathBuf>,
    },

    /// slideshow duration 3s image1.jpg image2.jpg image3.jpg
    ///
    /// Create video from images with specified duration per image.
    ///
    /// Examples:
    ///   slideshow duration 3s image1.jpg image2.jpg image3.jpg
    ///   slideshow duration 2s img1.png img2.png
    Slideshow {
        #[arg(value_name = "duration")]
        _duration_kw: KeywordDuration,
        #[arg(help = "Duration per image (e.g., 3s, 2s, 1.5s)")]
        duration: String,
        #[arg(help = "Input image files (1 or more)")]
        images: Vec<PathBuf>,
    },

    /// visualize audio.mp3 --style waveform
    ///
    /// Generate video from audio with waveform or spectrum visualization.
    ///
    /// Examples:
    ///   visualize audio.mp3 --style waveform
    ///   visualize audio.mp3 --style spectrum
    Visualize {
        #[arg(help = "Input audio file")]
        audio: PathBuf,
        #[arg(long, help = "Visualization style: waveform or spectrum (default: waveform)")]
        style: Option<String>,
    },

    /// convert video.mp4 to animated-gif --loop --optimize
    ///
    /// Convert video to optimized animated GIF with loop and optimization options.
    ///
    /// Examples:
    ///   convert video.mp4 to animated-gif --loop --optimize
    ///   convert video.mp4 to animated-gif --loop
    ///   convert video.mp4 to animated-gif --optimize
    AnimatedGif {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target format: animated-gif")]
        _format: KeywordAnimatedGif,
        #[arg(long, help = "Loop the GIF animation")]
        loop_video: bool,
        #[arg(long, help = "Optimize GIF for smaller file size")]
        optimize: bool,
    },

    /// tile video.mp4 3x3
    ///
    /// Repeat video in a grid pattern.
    ///
    /// Examples:
    ///   tile video.mp4 3x3
    ///   tile video.mp4 2x2
    Tile {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Grid layout: WxH format (e.g., 3x3, 2x2)")]
        layout: String,
    },

    /// repair video.mp4
    ///
    /// Attempt to fix corrupted video files.
    ///
    /// Examples:
    ///   repair video.mp4
    Repair {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// validate video.mp4
    ///
    /// Check if file is valid and complete.
    ///
    /// Examples:
    ///   validate video.mp4
    Validate {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// extract-keyframes video.mp4
    ///
    /// Extract I-frames only.
    ///
    /// Examples:
    ///   extract-keyframes video.mp4
    #[command(name = "extract-keyframes")]
    ExtractKeyframes {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// stats video.mp4
    ///
    /// Detailed statistics (bitrate over time, frame types, etc.).
    ///
    /// Examples:
    ///   stats video.mp4
    Stats {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// convert-hdr video.mp4 to sdr
    ///
    /// Convert HDR video to standard dynamic range (tone mapping).
    ///
    /// Examples:
    ///   convert-hdr video.mp4 to sdr
    #[command(name = "convert-hdr")]
    ConvertHdr {
        #[arg(help = "Input HDR video file")]
        input: PathBuf,
        #[arg(value_name = "to")]
        _to: KeywordTo,
        #[arg(help = "Target: sdr")]
        _sdr: KeywordSdr,
    },

    /// fix-framerate video.mp4
    ///
    /// Convert variable frame rate (VFR) to constant frame rate (CFR).
    ///
    /// Examples:
    ///   fix-framerate video.mp4
    #[command(name = "fix-framerate")]
    FixFramerate {
        #[arg(help = "Input video file")]
        input: PathBuf,
    },

    /// watch folder ./input --convert to mp4
    ///
    /// Auto-process files added to folder.
    ///
    /// Examples:
    ///   watch folder ./input --convert to mp4
    Watch {
        #[arg(help = "What to watch: folder")]
        _folder: KeywordFolder,
        #[arg(help = "Folder path to watch")]
        folder: PathBuf,
        #[arg(long, help = "Operation: convert")]
        operation: Option<BatchOperationCli>,
        #[arg(long, value_name = "to")]
        _to: Option<KeywordTo>,
        #[arg(long, help = "Target format (for convert operation): gif, mp4, webm, mp3, or wav")]
        format: Option<ConvertFormatCli>,
    },

    /// apply-template video.mp4 template.yaml
    ///
    /// Apply saved processing templates.
    ///
    /// Examples:
    ///   apply-template video.mp4 template.yaml
    #[command(name = "apply-template")]
    Apply {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Template file (YAML)")]
        template_file: PathBuf,
    },

    /// pipeline video.mp4 steps.yaml
    ///
    /// Define multi-step processing pipeline.
    ///
    /// Examples:
    ///   pipeline video.mp4 steps.yaml
    Pipeline {
        #[arg(help = "Input video file")]
        input: PathBuf,
        #[arg(help = "Pipeline steps file (YAML)")]
        steps_file: PathBuf,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum BlurTypeCli {
    #[value(help = "Blur a specific rectangular region")]
    Region,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ConvertFormatCli {
    #[value(help = "Convert to animated GIF format")]
    Gif,
    #[value(help = "Convert to MP4 video format")]
    Mp4,
    #[value(help = "Convert to WebM video format")]
    Webm,
    #[value(help = "Extract audio as MP3 format")]
    Mp3,
    #[value(help = "Extract audio as WAV format")]
    Wav,
    #[value(help = "Convert to iPhone-optimized MP4 format")]
    Iphone,
    #[value(help = "Convert to Android-optimized MP4 format")]
    Android,
    #[value(help = "Convert to HLS streaming format")]
    Hls,
    #[value(help = "Convert to DASH streaming format")]
    Dash,
    #[value(name = "360", help = "Convert to 360° video format")]
    Video360,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum FlipDirCli {
    #[value(help = "Flip video horizontally (left-right)")]
    Horizontal,
    #[value(help = "Flip video vertically (up-down)")]
    Vertical,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ExtractKind {
    Audio,
}

// Literal keywords (so your command strings read naturally)
#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordTo {
    To,
}
impl std::fmt::Display for KeywordTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "to")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordFrom {
    From,
}
impl std::fmt::Display for KeywordFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordBy {
    By,
}
impl std::fmt::Display for KeywordBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "by")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordAt {
    At,
}
impl std::fmt::Display for KeywordAt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordAnd {
    And,
}
impl std::fmt::Display for KeywordAnd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "and")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordTimes {
    Times,
}
impl std::fmt::Display for KeywordTimes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "times")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordIn {
    In,
}
impl std::fmt::Display for KeywordIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "in")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordOut {
    Out,
}
impl std::fmt::Display for KeywordOut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "out")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordEvery {
    Every,
}
impl std::fmt::Display for KeywordEvery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "every")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordDuration {
    Duration,
}
impl std::fmt::Display for KeywordDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "duration")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordSpeed {
    Speed,
}
impl std::fmt::Display for KeywordSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "speed")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordOn {
    On,
}
impl std::fmt::Display for KeywordOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "on")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordBackground {
    Background,
}
impl std::fmt::Display for KeywordBackground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "background")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordColor {
    Color,
}
impl std::fmt::Display for KeywordColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "color")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordScenes {
    Scenes,
}
impl std::fmt::Display for KeywordScenes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "scenes")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordBlack {
    Black,
}
impl std::fmt::Display for KeywordBlack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "black")
    }
}


#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordRotation {
    Rotation,
}
impl std::fmt::Display for KeywordRotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rotation")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordQuality {
    Quality,
}
impl std::fmt::Display for KeywordQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "quality")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordFormat {
    Format,
}
impl std::fmt::Display for KeywordFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "format")
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum BatchOperationCli {
    #[value(help = "Convert files to different format")]
    Convert,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SocialCropShapeCli {
    #[value(help = "Crop to square aspect ratio (1:1)")]
    Square,
    #[value(help = "Crop to circle (square with circular mask)")]
    Circle,
}

// Literal keywords for new commands
#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordMetadata {
    Metadata,
}
impl std::fmt::Display for KeywordMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "metadata")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordVertical {
    Vertical,
    Portrait,
}
impl std::fmt::Display for KeywordVertical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordVertical::Vertical => write!(f, "vertical"),
            KeywordVertical::Portrait => write!(f, "portrait"),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordStory {
    Story,
}
impl std::fmt::Display for KeywordStory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "story")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordNoise {
    Noise,
}
impl std::fmt::Display for KeywordNoise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "noise")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordEcho {
    Echo,
}
impl std::fmt::Display for KeywordEcho {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "echo")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordAudio {
    Audio,
}
impl std::fmt::Display for KeywordAudio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "audio")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordWhen {
    When,
}
impl std::fmt::Display for KeywordWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "when")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordVoiceDetected {
    Voice,
    Detected,
}
impl std::fmt::Display for KeywordVoiceDetected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordVoiceDetected::Voice => write!(f, "voice"),
            KeywordVoiceDetected::Detected => write!(f, "detected"),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordVoice {
    Voice,
}
impl std::fmt::Display for KeywordVoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "voice")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordVintage {
    Vintage,
}
impl std::fmt::Display for KeywordVintage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vintage")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordFilm {
    Film,
}
impl std::fmt::Display for KeywordFilm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "film")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordCameras {
    Cameras,
}
impl std::fmt::Display for KeywordCameras {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cameras")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordTestPattern {
    #[value(name = "test-pattern")]
    TestPattern,
}
impl std::fmt::Display for KeywordTestPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "test-pattern")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordTimecode {
    Timecode,
}
impl std::fmt::Display for KeywordTimecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "timecode")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordEdl {
    Edl,
}
impl std::fmt::Display for KeywordEdl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "edl")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordColorspace {
    Colorspace,
}
impl std::fmt::Display for KeywordColorspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "colorspace")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordSilence {
    Silence,
}
impl std::fmt::Display for KeywordSilence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "silence")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordLoudness {
    Loudness,
}
impl std::fmt::Display for KeywordLoudness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "loudness")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordFaces {
    Faces,
}
impl std::fmt::Display for KeywordFaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "faces")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordMotion {
    Motion,
}
impl std::fmt::Display for KeywordMotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "motion")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordDuplicates {
    Duplicates,
}
impl std::fmt::Display for KeywordDuplicates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "duplicates")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordLayout {
    Layout,
}
impl std::fmt::Display for KeywordLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "layout")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordAnimatedGif {
    #[value(name = "animated-gif")]
    AnimatedGif,
}
impl std::fmt::Display for KeywordAnimatedGif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "animated-gif")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordKeyframes {
    Keyframes,
}
impl std::fmt::Display for KeywordKeyframes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "keyframes")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordHdr {
    Hdr,
}
impl std::fmt::Display for KeywordHdr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hdr")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordSdr {
    Sdr,
}
impl std::fmt::Display for KeywordSdr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sdr")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordFramerate {
    Framerate,
}
impl std::fmt::Display for KeywordFramerate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "framerate")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordFolder {
    Folder,
}
impl std::fmt::Display for KeywordFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "folder")
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum KeywordTemplate {
    Template,
}
impl std::fmt::Display for KeywordTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "template")
    }
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn into_intent(self) -> Result<Intent> {
        use regex::Regex;
        
        match self.cmd {
            Commands::Convert { input, format, quality, codec, .. } => {
                // Special case: "convert video.mp4 to 360" maps to Convert360 intent
                if matches!(format, ConvertFormatCli::Video360) {
                    return Ok(Intent::Convert360 { input });
                }
                
                let format_enum = match format {
                    ConvertFormatCli::Gif => ConvertFormat::Gif,
                    ConvertFormatCli::Mp4 => ConvertFormat::Mp4,
                    ConvertFormatCli::Webm => ConvertFormat::Webm,
                    ConvertFormatCli::Mp3 => ConvertFormat::Mp3,
                    ConvertFormatCli::Wav => ConvertFormat::Wav,
                    ConvertFormatCli::Iphone => ConvertFormat::Iphone,
                    ConvertFormatCli::Android => ConvertFormat::Android,
                    ConvertFormatCli::Hls => ConvertFormat::Hls,
                    ConvertFormatCli::Dash => ConvertFormat::Dash,
                    ConvertFormatCli::Video360 => ConvertFormat::Video360, // Should not reach here due to early return
                };
                
                let quality_parsed = quality.map(|s| QualityPreset::parse(&s)).transpose()?;
                
                let codec_parsed = if let Some(codec_str) = codec {
                    let codec_lower = codec_str.trim().to_lowercase();
                    Some(match codec_lower.as_str() {
                        "h264" | "x264" => VideoCodec::H264,
                        "h265" | "x265" | "hevc" => VideoCodec::H265,
                        "vp9" => VideoCodec::Vp9,
                        "copy" => VideoCodec::Copy,
                        _ => anyhow::bail!("Invalid codec: {codec_str} (try h264, h265, vp9, or copy)"),
                    })
                } else {
                    None
                };
                
                Ok(Intent::Convert { input, format: format_enum, quality: quality_parsed, codec: codec_parsed })
            }
            Commands::Compress { input, target, two_pass, .. } => {
                // Try to parse as quality preset first (e.g., "high-quality")
                let target_lower = target.trim().to_lowercase();
                if target_lower.ends_with("-quality") {
                    let quality_str = target_lower.strip_suffix("-quality").unwrap().trim();
                    let quality = QualityPreset::parse(quality_str)?;
                    Ok(Intent::Compress { input, target: CompressTarget::Quality(quality), two_pass })
                } else {
                    // Try to parse as size
                    let target_size = TargetSize::parse(&target)?;
                    Ok(Intent::Compress { input, target: CompressTarget::Size(target_size), two_pass })
                }
            }
            Commands::Trim { input, start, end, .. } => {
                let start = Time::parse(&start)?;
                let end = Time::parse(&end)?;
                Ok(Intent::Trim { input, start, end })
            }
            Commands::Extract { input, .. } => {
                Ok(Intent::ExtractAudio { input, format: AudioFormat::Mp3 })
            }
            Commands::AdjustVolume { input, adjustment, .. } => {
                let adjustment_parsed = VolumeAdjustment::parse(&adjustment)?;
                Ok(Intent::AdjustVolume { input, adjustment: adjustment_parsed })
            }
            Commands::SyncAudio { input, direction, offset, .. } => {
                let direction_parsed = AudioSyncDirection::parse(&direction)?;
                let offset_parsed = Duration::parse(&offset)?;
                Ok(Intent::SyncAudio { input, direction: direction_parsed, offset: offset_parsed })
            }
            Commands::MixAudio { audio1, audio2, .. } => {
                Ok(Intent::MixAudio { audio1, audio2 })
            }
            Commands::ExtractAudioRange { input, start, end, .. } => {
                let start_parsed = Time::parse(&start)?;
                let end_parsed = Time::parse(&end)?;
                Ok(Intent::ExtractAudioRange { input, start: start_parsed, end: end_parsed, format: AudioFormat::Mp3 })
            }
            Commands::Resize { input, target, .. } => {
                let target = ResizeTarget::parse(&target)?;
                Ok(Intent::Resize { input, target })
            }
            Commands::SpeedUp { input, factor, .. } => {
                let factor = SpeedFactor::parse(&factor)?;
                Ok(Intent::SpeedUp { input, factor })
            }
            Commands::SlowDown { input, factor, .. } => {
                let factor = SpeedFactor::parse(&factor)?;
                Ok(Intent::SlowDown { input, factor })
            }
            Commands::Reverse { input } => Ok(Intent::Reverse { input }),
            Commands::Mute { input } => Ok(Intent::Mute { input }),
            Commands::Rotate { input, degrees, .. } => {
                let degrees = RotateDegrees::new(degrees)?;
                Ok(Intent::Rotate { input, degrees })
            }
            Commands::Flip { input, direction } => {
                let direction = match direction {
                    FlipDirCli::Horizontal => FlipDirection::Horizontal,
                    FlipDirCli::Vertical => FlipDirection::Vertical,
                };
                Ok(Intent::Flip { input, direction })
            }
            Commands::Thumbnail { input, time, .. } => {
                let time = Time::parse(&time)?;
                Ok(Intent::Thumbnail { input, time })
            }
            Commands::Crop { input, size, .. } => {
                let re = Regex::new(r"^\s*(\d+)\s*x\s*(\d+)\s*$")
                    .map_err(|e| anyhow!("Invalid regex: {}", e))?;
                let caps = re.captures(&size)
                    .ok_or_else(|| anyhow!("Invalid crop size: {size} (try 640x480)"))?;
                let width = caps.get(1).unwrap().as_str().parse::<u32>()?;
                let height = caps.get(2).unwrap().as_str().parse::<u32>()?;
                Ok(Intent::Crop { input, width, height })
            }
            Commands::Fps { input, fps, .. } => Ok(Intent::SetFps { input, fps }),
            Commands::Loop { input, times, .. } => {
                if times < 2 {
                    anyhow::bail!("Loop times must be >= 2");
                }
                Ok(Intent::Loop { input, times })
            }
            Commands::Merge { a, b, .. } => Ok(Intent::Merge { a, b }),
            Commands::AddAudio { audio, video, .. } => Ok(Intent::AddAudio { audio, video }),
            Commands::Grayscale { input } => Ok(Intent::Grayscale { input }),
            Commands::Stabilize { input } => Ok(Intent::Stabilize { input }),
            Commands::Denoise { input } => Ok(Intent::Denoise { input }),
            Commands::Watermark { input, logo, position, opacity, size, .. } => {
                let position = WatermarkPosition::parse(&position)?;
                let opacity_value = opacity.unwrap_or(1.0);
                let opacity = Opacity::new(opacity_value)?;
                let size = size.map(|s| WatermarkSize::parse(&s)).transpose()?;
                Ok(Intent::Watermark { input, logo, position, opacity, size })
            }
            Commands::AddText { input, text, position, font_size, color, timestamp, .. } => {
                let position = TextPosition::parse(&position)?;
                let color = if let Some(color_str) = color {
                    TextColor::parse(&color_str)?
                } else {
                    TextColor::default()
                };
                let style = TextStyle {
                    font_size,
                    font_file: None, // Could be added later
                    color,
                };
                Ok(Intent::AddText { input, text, position, style, timestamp })
            }
            Commands::Filter { input, brightness, contrast, saturation, preset } => {
                // Validate that either adjustments or preset is provided
                let has_adjustments = brightness.is_some() || contrast.is_some() || saturation.is_some();
                if !has_adjustments && preset.is_none() {
                    anyhow::bail!("Filter requires either adjustments (--brightness, --contrast, --saturation) or --preset");
                }
                if has_adjustments && preset.is_some() {
                    anyhow::bail!("Cannot use both adjustments and preset at the same time");
                }

                // Normalize adjustments from -100..100 to -1.0..1.0
                let adjustments = FilterAdjustments {
                    brightness: brightness.map(|v| v.clamp(-100.0, 100.0) / 100.0),
                    contrast: contrast.map(|v| v.clamp(-100.0, 100.0) / 100.0),
                    saturation: saturation.map(|v| v.clamp(-100.0, 100.0) / 100.0),
                };

                let color_preset = if let Some(preset_str) = preset {
                    Some(ColorPreset::parse(&preset_str)?)
                } else {
                    None
                };

                Ok(Intent::Filter { input, adjustments, preset: color_preset })
            }
            Commands::Blur { input, blur_type, region } => {
                let blur_type_parsed = match blur_type {
                    BlurTypeCli::Region => {
                        let region_parsed = BlurRegion::parse(&region)?;
                        BlurType::Region(region_parsed)
                    }
                };
                Ok(Intent::Blur { input, blur_type: blur_type_parsed })
            }
            Commands::Normalize { input } => {
                Ok(Intent::Normalize { input })
            }
            Commands::Fade { input, fade_in, fade_out } => {
                // Validate that at least one fade is specified
                if fade_in.is_none() && fade_out.is_none() {
                    anyhow::bail!("Fade requires at least one of --fade-in or --fade-out");
                }
                
                let fade_in_parsed = fade_in.map(|s| Duration::parse(&s)).transpose()?;
                let fade_out_parsed = fade_out.map(|s| Duration::parse(&s)).transpose()?;
                
                Ok(Intent::Fade { input, fade_in: fade_in_parsed, fade_out: fade_out_parsed })
            }
            Commands::Split { input, mode } => {
                // Parse split mode: "every 30s" or "into 3 parts"
                let mode_lower = mode.trim().to_lowercase();
                
                if mode_lower.starts_with("every ") {
                    let duration_str = mode_lower.strip_prefix("every ").unwrap().trim();
                    let duration = Duration::parse(duration_str)?;
                    Ok(Intent::Split { input, mode: SplitMode::Every(duration) })
                } else if mode_lower.starts_with("into ") {
                    let re = Regex::new(r"^into\s+(\d+)\s+parts?$")
                        .map_err(|e| anyhow!("Invalid regex: {}", e))?;
                    let caps = re.captures(&mode_lower)
                        .ok_or_else(|| anyhow!("Invalid split mode: {mode} (try 'every 30s' or 'into 3 parts')"))?;
                    let parts = caps.get(1).unwrap().as_str().parse::<u32>()?;
                    if parts < 2 {
                        anyhow::bail!("Split must be into at least 2 parts");
                    }
                    Ok(Intent::Split { input, mode: SplitMode::IntoParts(parts) })
                } else {
                    anyhow::bail!("Invalid split mode: {mode} (try 'every 30s' or 'into 3 parts')");
                }
            }
            Commands::ExtractFrames { input, interval, .. } => {
                let interval_parsed = Duration::parse(&interval)?;
                Ok(Intent::ExtractFrames { input, interval: interval_parsed })
            }
            Commands::BurnSubtitle { input, subtitle } => {
                Ok(Intent::BurnSubtitle { input, subtitle })
            }
            Commands::Batch { operation, pattern, format, r#if, .. } => {
                match operation {
                    BatchOperationCli::Convert => {
                        let format_parsed = format.ok_or_else(|| anyhow!("Batch convert requires a target format"))?;
                        let format_enum = match format_parsed {
                            ConvertFormatCli::Gif => ConvertFormat::Gif,
                            ConvertFormatCli::Mp4 => ConvertFormat::Mp4,
                            ConvertFormatCli::Webm => ConvertFormat::Webm,
                            ConvertFormatCli::Mp3 => ConvertFormat::Mp3,
                            ConvertFormatCli::Wav => ConvertFormat::Wav,
                            ConvertFormatCli::Iphone => ConvertFormat::Iphone,
                            ConvertFormatCli::Android => ConvertFormat::Android,
                            ConvertFormatCli::Hls => ConvertFormat::Hls,
                            ConvertFormatCli::Dash => ConvertFormat::Dash,
                            ConvertFormatCli::Video360 => ConvertFormat::Video360,
                        };
                        let batch_op = BatchOperation::Convert(format_enum);
                        
                        if let Some(condition_str) = r#if {
                            let condition = parse_processing_condition(&condition_str)?;
                            Ok(Intent::ConditionalBatch { pattern, operation: batch_op, condition })
                        } else {
                            Ok(Intent::Batch { pattern, operation: batch_op })
                        }
                    }
                }
            }
            Commands::Compare { video1, video2, show_psnr, .. } => {
                Ok(Intent::Compare { video1, video2, show_psnr })
            }
            Commands::SetMetadata { input, field, value } => {
                let field_parsed = MetadataField::parse(&field)?;
                Ok(Intent::SetMetadata { input, field: field_parsed, value })
            }
            Commands::Montage { videos, layout, .. } => {
                if videos.len() < 2 {
                    anyhow::bail!("Montage requires at least 2 video files");
                }
                let layout_parsed = MontageLayout::parse(&layout)?;
                let total_cells = layout_parsed.total_cells();
                if videos.len() as u32 > total_cells {
                    anyhow::bail!("Too many videos ({}) for layout {} (max {})", videos.len(), layout_parsed, total_cells);
                }
                Ok(Intent::Montage { videos, layout: layout_parsed })
            }
            Commands::Crossfade { video1, video2, duration, .. } => {
                let duration_parsed = Duration::parse(&duration)?;
                Ok(Intent::Crossfade { video1, video2, duration: duration_parsed })
            }
            Commands::Timelapse { input, speed, .. } => {
                let speed_factor = SpeedFactor::parse(&speed)?;
                Ok(Intent::Timelapse { input, speed: speed_factor })
            }
            Commands::Pip { overlay_video, base_video, position, .. } => {
                let position_parsed = PipPosition::parse(&position)?;
                Ok(Intent::Pip { overlay_video, base_video, position: position_parsed })
            }
            Commands::RemoveBackground { input, color, .. } => {
                let color_parsed = ChromaKeyColor::parse(&color)?;
                Ok(Intent::RemoveBackground { input, color: color_parsed })
            }
            Commands::Overlay { overlay_video, base_video, position, opacity, .. } => {
                let position_parsed = WatermarkPosition::parse(&position)?;
                let opacity_value = opacity.unwrap_or(1.0);
                let opacity_parsed = Opacity::new(opacity_value)?;
                Ok(Intent::Overlay { overlay_video, base_video, position: position_parsed, opacity: opacity_parsed })
            }
            Commands::Concat { videos } => {
                if videos.len() < 2 {
                    anyhow::bail!("Concat requires at least 2 video files");
                }
                Ok(Intent::Concat { videos })
            }
            Commands::DetectScenes { input, .. } => {
                Ok(Intent::DetectScenes { input })
            }
            Commands::DetectBlack { input, .. } => {
                Ok(Intent::DetectBlack { input })
            }
            Commands::FixRotation { input, .. } => {
                Ok(Intent::FixRotation { input })
            }
            Commands::AnalyzeQuality { input, .. } => {
                Ok(Intent::AnalyzeQuality { input })
            }
            Commands::Preview { input } => {
                Ok(Intent::Preview { input })
            }
            Commands::Suggest { input, .. } => {
                Ok(Intent::SuggestFormat { input })
            }
            Commands::Workflow { config_file } => {
                Ok(Intent::Workflow { config_file })
            }
            Commands::MotionBlur { input, radius } => {
                Ok(Intent::MotionBlur { input, radius })
            }
            Commands::Vignette { input, intensity, size } => {
                Ok(Intent::Vignette { input, intensity, size })
            }
            Commands::LensCorrect { input } => {
                Ok(Intent::LensCorrect { input })
            }
            Commands::Interpolate { input, fps, .. } => {
                Ok(Intent::Interpolate { input, fps })
            }
            Commands::ExtractMetadata { input, format, .. } => {
                let format_enum = if let Some(fmt_str) = format {
                    match fmt_str.trim().to_lowercase().as_str() {
                        "json" => crate::model::intent::MetadataFormat::Json,
                        "xml" => crate::model::intent::MetadataFormat::Xml,
                        _ => anyhow::bail!("Invalid metadata format: {fmt_str} (try json or xml)"),
                    }
                } else {
                    crate::model::intent::MetadataFormat::Json // default
                };
                Ok(Intent::ExtractMetadata { input, format: format_enum })
            }
            Commands::Info { input } => {
                Ok(Intent::Info { input })
            }
            Commands::Thumbnails { input, layout } => {
                let layout_parsed = crate::model::intent::ThumbnailGridLayout::parse(&layout)?;
                Ok(Intent::ThumbnailGrid { input, layout: layout_parsed })
            }
            Commands::SocialConvert { input, platform, .. } => {
                let platform_parsed = crate::model::intent::SocialPlatform::parse(&platform)?;
                Ok(Intent::SocialMediaConvert { input, platform: platform_parsed })
            }
            Commands::SocialCrop { input, shape } => {
                let shape_enum = match shape {
                    SocialCropShapeCli::Square => crate::model::intent::SocialCropShape::Square,
                    SocialCropShapeCli::Circle => crate::model::intent::SocialCropShape::Circle,
                };
                Ok(Intent::SocialCrop { input, shape: shape_enum })
            }
            Commands::VerticalConvert { input, .. } => {
                Ok(Intent::VerticalConvert { input })
            }
            Commands::StoryConvert { input, .. } => {
                Ok(Intent::StoryFormat { input })
            }
            Commands::ReduceNoise { input, .. } => {
                Ok(Intent::NoiseReduction { input })
            }
            Commands::RemoveEcho { input, .. } => {
                Ok(Intent::EchoRemoval { input })
            }
            Commands::DuckAudio { input, .. } => {
                Ok(Intent::AudioDucking { input })
            }
            Commands::EqualizeAudio { input, bass, treble, mid, .. } => {
                Ok(Intent::AudioEqualizer { input, bass, treble, mid })
            }
            Commands::IsolateVoice { input, .. } => {
                Ok(Intent::VoiceIsolation { input })
            }
            Commands::SpeedAudio { input, factor, keep_pitch, .. } => {
                if !keep_pitch {
                    anyhow::bail!("Speed audio requires --keep-pitch flag");
                }
                let factor_parsed = SpeedFactor::parse(&factor)?;
                Ok(Intent::AudioSpeedKeepPitch { input, factor: factor_parsed })
            }
            Commands::Glitch { input, shift, noise } => {
                Ok(Intent::Glitch { input, shift, noise })
            }
            Commands::VintageFilm { input, era } => {
                Ok(Intent::VintageFilm { input, era })
            }
            Commands::SplitScreen { video1, video2, orientation, .. } => {
                let orientation_parsed = if let Some(orient_str) = orientation {
                    crate::model::types::SplitScreenOrientation::parse(&orient_str)?
                } else {
                    crate::model::types::SplitScreenOrientation::Horizontal // default
                };
                Ok(Intent::SplitScreen { video1, video2, orientation: orientation_parsed })
            }
            Commands::Mirror { input, direction } => {
                let direction_parsed = crate::model::types::MirrorDirection::parse(&direction)?;
                Ok(Intent::Mirror { input, direction: direction_parsed })
            }
            Commands::ColorGrade { input, preset } => {
                let preset_parsed = crate::model::types::ColorGradePreset::parse(&preset)?;
                Ok(Intent::ColorGrade { input, preset: preset_parsed })
            }
            Commands::AnimatedText { input, text, position, animate, font_size, color, .. } => {
                let position_parsed = TextPosition::parse(&position)?;
                let color_parsed = if let Some(color_str) = color {
                    TextColor::parse(&color_str)?
                } else {
                    TextColor::default()
                };
                let style = TextStyle {
                    font_size,
                    font_file: None,
                    color: color_parsed,
                };
                let animation_parsed = if let Some(anim_str) = animate {
                    crate::model::types::TextAnimation::parse(&anim_str)?
                } else {
                    anyhow::bail!("Animated text requires --animate flag (fade-in, slide-in, or typewriter)");
                };
                Ok(Intent::AnimatedText { input, text, position: position_parsed, animation: animation_parsed, style })
            }
            Commands::Transition { video1, video2, r#type, .. } => {
                let transition_type_parsed = crate::model::types::TransitionType::parse(&r#type)?;
                Ok(Intent::Transition { video1, video2, transition_type: transition_type_parsed })
            }
            Commands::SyncCameras { videos, .. } => {
                if videos.len() < 2 {
                    anyhow::bail!("Sync cameras requires at least 2 video files");
                }
                Ok(Intent::SyncCameras { videos })
            }
            Commands::Generate { resolution, duration, .. } => {
                let duration_parsed = Duration::parse(&duration)?;
                Ok(Intent::GenerateTestPattern { resolution, duration: duration_parsed })
            }
            Commands::AddTimecode { input, .. } => {
                Ok(Intent::AddTimecode { input })
            }
            Commands::Proxy { input } => {
                Ok(Intent::Proxy { input })
            }
            Commands::Export { input, .. } => {
                Ok(Intent::ExportEdl { input })
            }
            Commands::ConvertColorspace { input, target, .. } => {
                let target_parsed = crate::model::types::Colorspace::parse(&target)?;
                Ok(Intent::ConvertColorspace { input, target: target_parsed })
            }
            Commands::DetectSilence { input, .. } => {
                Ok(Intent::DetectSilence { input })
            }
            Commands::AnalyzeLoudness { input, .. } => {
                Ok(Intent::AnalyzeLoudness { input })
            }
            Commands::DetectDuplicates { input, .. } => {
                Ok(Intent::DetectDuplicates { input })
            }
            Commands::Collage { videos, layout, .. } => {
                if videos.len() < 2 {
                    anyhow::bail!("Collage requires at least 2 video files");
                }
                let layout_parsed = MontageLayout::parse(&layout)?;
                let total_cells = layout_parsed.total_cells();
                if videos.len() as u32 > total_cells {
                    anyhow::bail!("Too many videos ({}) for layout {} (max {})", videos.len(), layout_parsed, total_cells);
                }
                Ok(Intent::Collage { videos, layout: layout_parsed })
            }
            Commands::Slideshow { images, duration, .. } => {
                if images.is_empty() {
                    anyhow::bail!("Slideshow requires at least 1 image file");
                }
                let duration_parsed = Duration::parse(&duration)?;
                Ok(Intent::Slideshow { images, duration: duration_parsed })
            }
            Commands::Visualize { audio, style } => {
                let style_parsed = if let Some(style_str) = style {
                    crate::model::types::VisualizationStyle::parse(&style_str)?
                } else {
                    crate::model::types::VisualizationStyle::Waveform // default
                };
                Ok(Intent::Visualize { audio, style: style_parsed })
            }
            Commands::AnimatedGif { input, loop_video, optimize, .. } => {
                Ok(Intent::AnimatedGif { input, loop_video, optimize })
            }
            Commands::Tile { input, layout } => {
                let layout_parsed = MontageLayout::parse(&layout)?;
                Ok(Intent::Tile { input, layout: layout_parsed })
            }
            Commands::Doctor => Ok(Intent::Doctor),
            Commands::Repair { input } => {
                Ok(Intent::Repair { input })
            }
            Commands::Validate { input } => {
                Ok(Intent::Validate { input })
            }
            Commands::ExtractKeyframes { input, .. } => {
                Ok(Intent::ExtractKeyframes { input })
            }
            Commands::Stats { input } => {
                Ok(Intent::Stats { input })
            }
            Commands::ConvertHdr { input, .. } => {
                Ok(Intent::ConvertHdrToSdr { input })
            }
            Commands::FixFramerate { input, .. } => {
                Ok(Intent::FixFramerate { input })
            }
            Commands::Watch { folder, operation, format, .. } => {
                let batch_op = if let Some(BatchOperationCli::Convert) = operation {
                    let format_parsed = format.ok_or_else(|| anyhow!("Watch requires a target format"))?;
                    let format_enum = match format_parsed {
                        ConvertFormatCli::Gif => ConvertFormat::Gif,
                        ConvertFormatCli::Mp4 => ConvertFormat::Mp4,
                        ConvertFormatCli::Webm => ConvertFormat::Webm,
                        ConvertFormatCli::Mp3 => ConvertFormat::Mp3,
                        ConvertFormatCli::Wav => ConvertFormat::Wav,
                        ConvertFormatCli::Iphone => ConvertFormat::Iphone,
                        ConvertFormatCli::Android => ConvertFormat::Android,
                        ConvertFormatCli::Hls => ConvertFormat::Hls,
                        ConvertFormatCli::Dash => ConvertFormat::Dash,
                        ConvertFormatCli::Video360 => ConvertFormat::Video360,
                    };
                    BatchOperation::Convert(format_enum)
                } else {
                    anyhow::bail!("Watch currently only supports --convert operation");
                };
                Ok(Intent::WatchFolder { folder, operation: batch_op })
            }
            Commands::Apply { input, template_file, .. } => {
                Ok(Intent::ApplyTemplate { input, template_file })
            }
            Commands::Pipeline { input, steps_file } => {
                Ok(Intent::Pipeline { input, steps_file })
            }
        }
    }
}

/// Parse a processing condition string like "duration < 30s"
fn parse_processing_condition(s: &str) -> Result<crate::model::types::ProcessingCondition> {
    use crate::model::types::*;
    use regex::Regex;
    
    let s = s.trim();
    
    // Parse patterns like "duration < 30s", "duration > 1:00", "duration = 30s"
    let re = Regex::new(r"(?i)^\s*duration\s*(<|>|=)\s*(.+)$")
        .map_err(|e| anyhow!("Invalid regex: {}", e))?;
    
    let caps = re.captures(s)
        .ok_or_else(|| anyhow!("Invalid condition: {s} (try 'duration < 30s')"))?;
    
    let operator = caps.get(1).unwrap().as_str();
    let value_str = caps.get(2).unwrap().as_str();
    let duration = Duration::parse(value_str)?;
    
    match operator {
        "<" => Ok(ProcessingCondition::DurationLessThan(duration)),
        ">" => Ok(ProcessingCondition::DurationGreaterThan(duration)),
        "=" => Ok(ProcessingCondition::DurationEquals(duration)),
        _ => anyhow::bail!("Invalid operator: {operator} (use <, >, or =)"),
    }
}

