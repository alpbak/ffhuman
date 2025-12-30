use crate::model::types::*;
use std::path::PathBuf;

/// Represents the user's intent - what they want to do with their media
#[derive(Debug, Clone)]
pub enum Intent {
    Convert {
        input: PathBuf,
        format: ConvertFormat,
        quality: Option<QualityPreset>,
        codec: Option<VideoCodec>,
    },
    Compress {
        input: PathBuf,
        target: CompressTarget,
        two_pass: bool,
    },
    Trim {
        input: PathBuf,
        start: Time,
        end: Time,
    },
    ExtractAudio {
        input: PathBuf,
        format: AudioFormat,
    },
    AdjustVolume {
        input: PathBuf,
        adjustment: VolumeAdjustment,
    },
    SyncAudio {
        input: PathBuf,
        direction: AudioSyncDirection,
        offset: Duration,
    },
    MixAudio {
        audio1: PathBuf,
        audio2: PathBuf,
    },
    ExtractAudioRange {
        input: PathBuf,
        start: Time,
        end: Time,
        format: AudioFormat,
    },
    Resize {
        input: PathBuf,
        target: ResizeTarget,
    },
    SpeedUp {
        input: PathBuf,
        factor: SpeedFactor,
    },
    SlowDown {
        input: PathBuf,
        factor: SpeedFactor,
    },
    Reverse {
        input: PathBuf,
    },
    Mute {
        input: PathBuf,
    },
    Rotate {
        input: PathBuf,
        degrees: RotateDegrees,
    },
    Flip {
        input: PathBuf,
        direction: FlipDirection,
    },
    Thumbnail {
        input: PathBuf,
        time: Time,
    },
    Crop {
        input: PathBuf,
        width: u32,
        height: u32,
    },
    SetFps {
        input: PathBuf,
        fps: u32,
    },
    Loop {
        input: PathBuf,
        times: u32,
    },
    Merge {
        a: PathBuf,
        b: PathBuf,
    },
    AddAudio {
        audio: PathBuf,
        video: PathBuf,
    },
    Grayscale {
        input: PathBuf,
    },
    Stabilize {
        input: PathBuf,
    },
    Denoise {
        input: PathBuf,
    },
    Watermark {
        input: PathBuf,
        logo: PathBuf,
        position: WatermarkPosition,
        opacity: Opacity,
        size: Option<WatermarkSize>,
    },
    AddText {
        input: PathBuf,
        text: String,
        position: TextPosition,
        style: TextStyle,
        timestamp: bool,
    },
    Filter {
        input: PathBuf,
        adjustments: FilterAdjustments,
        preset: Option<ColorPreset>,
    },
    Blur {
        input: PathBuf,
        blur_type: BlurType,
    },
    Normalize {
        input: PathBuf,
    },
    Fade {
        input: PathBuf,
        fade_in: Option<Duration>,
        fade_out: Option<Duration>,
    },
    Split {
        input: PathBuf,
        mode: SplitMode,
    },
    ExtractFrames {
        input: PathBuf,
        interval: Duration,
    },
    BurnSubtitle {
        input: PathBuf,
        subtitle: PathBuf,
    },
    Batch {
        pattern: String,
        operation: BatchOperation,
    },
    Compare {
        video1: PathBuf,
        video2: PathBuf,
        show_psnr: bool,
    },
    SetMetadata {
        input: PathBuf,
        field: MetadataField,
        value: String,
    },
    Montage {
        videos: Vec<PathBuf>,
        layout: MontageLayout,
    },
    Crossfade {
        video1: PathBuf,
        video2: PathBuf,
        duration: Duration,
    },
    Timelapse {
        input: PathBuf,
        speed: SpeedFactor,
    },
    Pip {
        overlay_video: PathBuf,
        base_video: PathBuf,
        position: PipPosition,
    },
    RemoveBackground {
        input: PathBuf,
        color: ChromaKeyColor,
    },
    Overlay {
        overlay_video: PathBuf,
        base_video: PathBuf,
        position: WatermarkPosition,
        opacity: Opacity,
    },
    Concat {
        videos: Vec<PathBuf>,
    },
    DetectScenes {
        input: PathBuf,
    },
    DetectBlack {
        input: PathBuf,
    },
    FixRotation {
        input: PathBuf,
    },
    AnalyzeQuality {
        input: PathBuf,
    },
    Preview {
        input: PathBuf,
    },
    SuggestFormat {
        input: PathBuf,
    },
    Workflow {
        config_file: PathBuf,
    },
    MotionBlur {
        input: PathBuf,
        radius: Option<u32>,
    },
    Vignette {
        input: PathBuf,
        intensity: Option<f32>,
        size: Option<f32>,
    },
    LensCorrect {
        input: PathBuf,
    },
    Interpolate {
        input: PathBuf,
        fps: u32,
    },
    ExtractMetadata {
        input: PathBuf,
        format: MetadataFormat,
    },
    Info {
        input: PathBuf,
    },
    ThumbnailGrid {
        input: PathBuf,
        layout: ThumbnailGridLayout,
    },
    SocialMediaConvert {
        input: PathBuf,
        platform: SocialPlatform,
    },
    SocialCrop {
        input: PathBuf,
        shape: SocialCropShape,
    },
    VerticalConvert {
        input: PathBuf,
    },
    StoryFormat {
        input: PathBuf,
    },
    NoiseReduction {
        input: PathBuf,
    },
    EchoRemoval {
        input: PathBuf,
    },
    AudioDucking {
        input: PathBuf,
    },
    AudioEqualizer {
        input: PathBuf,
        bass: Option<i32>,
        treble: Option<i32>,
        mid: Option<i32>,
    },
    VoiceIsolation {
        input: PathBuf,
    },
    AudioSpeedKeepPitch {
        input: PathBuf,
        factor: SpeedFactor,
    },
    Glitch {
        input: PathBuf,
        shift: Option<u32>,
        noise: Option<u32>,
    },
    VintageFilm {
        input: PathBuf,
        era: Option<String>,
    },
    SplitScreen {
        video1: PathBuf,
        video2: PathBuf,
        orientation: crate::model::types::SplitScreenOrientation,
    },
    Mirror {
        input: PathBuf,
        direction: crate::model::types::MirrorDirection,
    },
    ColorGrade {
        input: PathBuf,
        preset: crate::model::types::ColorGradePreset,
    },
    AnimatedText {
        input: PathBuf,
        text: String,
        position: crate::model::types::TextPosition,
        animation: crate::model::types::TextAnimation,
        style: crate::model::types::TextStyle,
    },
    Transition {
        video1: PathBuf,
        video2: PathBuf,
        transition_type: crate::model::types::TransitionType,
    },
    SyncCameras {
        videos: Vec<PathBuf>,
    },
    GenerateTestPattern {
        resolution: String,
        duration: Duration,
    },
    AddTimecode {
        input: PathBuf,
    },
    Proxy {
        input: PathBuf,
    },
    ExportEdl {
        input: PathBuf,
    },
    ConvertColorspace {
        input: PathBuf,
        target: Colorspace,
    },
    DetectSilence {
        input: PathBuf,
    },
    AnalyzeLoudness {
        input: PathBuf,
    },
    DetectDuplicates {
        input: PathBuf,
    },
    Collage {
        videos: Vec<PathBuf>,
        layout: MontageLayout,
    },
    Slideshow {
        images: Vec<PathBuf>,
        duration: Duration,
    },
    Visualize {
        audio: PathBuf,
        style: VisualizationStyle,
    },
    AnimatedGif {
        input: PathBuf,
        loop_video: bool,
        optimize: bool,
    },
    Tile {
        input: PathBuf,
        layout: MontageLayout,
    },
    Doctor,
    Repair {
        input: PathBuf,
    },
    Validate {
        input: PathBuf,
    },
    ExtractKeyframes {
        input: PathBuf,
    },
    Stats {
        input: PathBuf,
    },
    Convert360 {
        input: PathBuf,
    },
    ConvertHdrToSdr {
        input: PathBuf,
    },
    FixFramerate {
        input: PathBuf,
    },
    WatchFolder {
        folder: PathBuf,
        operation: BatchOperation,
    },
    ApplyTemplate {
        input: PathBuf,
        template_file: PathBuf,
    },
    Pipeline {
        input: PathBuf,
        steps_file: PathBuf,
    },
    ConditionalBatch {
        pattern: String,
        operation: BatchOperation,
        condition: ProcessingCondition,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertFormat {
    Gif,
    Mp4,
    Webm,
    Mp3,
    Wav,
    Iphone,
    Android,
    Hls,
    Dash,
    Video360,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Mp3,
    Wav,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataFormat {
    Json,
    Xml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThumbnailGridLayout {
    pub cols: u32,
    pub rows: u32,
}

impl ThumbnailGridLayout {
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        use regex::Regex;
        let re = Regex::new(r"^\s*(\d+)\s*x\s*(\d+)\s*$")
            .map_err(|e| anyhow::anyhow!("Invalid regex: {}", e))?;
        
        let caps = re.captures(s)
            .ok_or_else(|| anyhow::anyhow!("Invalid grid layout: {s} (try 3x3)"))?;
        
        let cols = caps.get(1).unwrap().as_str().parse::<u32>()?;
        let rows = caps.get(2).unwrap().as_str().parse::<u32>()?;
        
        if cols == 0 || rows == 0 {
            anyhow::bail!("Grid columns and rows must be greater than 0");
        }
        
        Ok(ThumbnailGridLayout { cols, rows })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocialPlatform {
    Instagram,
    TikTok,
    YoutubeShorts,
    Twitter,
}

impl SocialPlatform {
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "instagram" | "ig" => Ok(SocialPlatform::Instagram),
            "tiktok" | "tt" => Ok(SocialPlatform::TikTok),
            "youtube-shorts" | "youtube shorts" | "shorts" | "yt-shorts" => Ok(SocialPlatform::YoutubeShorts),
            "twitter" | "x" => Ok(SocialPlatform::Twitter),
            _ => anyhow::bail!("Invalid social platform: {s} (try instagram, tiktok, youtube-shorts, or twitter)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocialCropShape {
    Square,
    Circle,
}

