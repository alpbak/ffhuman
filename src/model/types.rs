use crate::model::ConvertFormat;
use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::fmt;

/// Represents a time duration that can be parsed from various formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

impl Time {
    /// Parse time from string formats: "SS", "M:SS", "H:MM:SS"
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        let re = Regex::new(r"^(\d+)(?::(\d{1,2}))?(?::(\d{1,2}))?$")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;
        
        let caps = re.captures(s)
            .ok_or_else(|| anyhow!("Invalid time format: {s}"))?;

        let a = caps.get(1).unwrap().as_str().parse::<u32>()?;
        let b = caps.get(2).map(|m| m.as_str().parse::<u32>()).transpose()?;
        let c = caps.get(3).map(|m| m.as_str().parse::<u32>()).transpose()?;

        // If 3 groups: H:MM:SS where a=H, b=MM, c=SS
        // If 2 groups: M:SS where a=M, b=SS
        // If 1 group: SS where a=SS
        let (h, m, s) = match (b, c) {
            (Some(mm), Some(ss)) => (a, mm, ss),
            (Some(ss), None) => (0, a, ss),
            (None, None) => (0, 0, a),
            _ => bail!("Invalid time format: {s}"),
        };

        Ok(Time {
            hours: h,
            minutes: m,
            seconds: s,
        })
    }

    /// Convert to FFmpeg time format (HH:MM:SS)
    pub fn to_ffmpeg(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }

    /// Convert to total seconds
    pub fn to_seconds(&self) -> u32 {
        self.hours * 3600 + self.minutes * 60 + self.seconds
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hours > 0 {
            write!(f, "{}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
        } else if self.minutes > 0 {
            write!(f, "{}:{:02}", self.minutes, self.seconds)
        } else {
            write!(f, "{}", self.seconds)
        }
    }
}

/// Represents a target file size that can be parsed from strings like "10mb", "800k", "1.5gb"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetSize {
    pub bytes: u64,
}

impl TargetSize {
    /// Parse size from string formats: "10mb", "800k", "1.5gb"
    pub fn parse(s: &str) -> Result<Self> {
        let re = Regex::new(r"(?i)^\s*(\d+(?:\.\d+)?)\s*(b|kb|k|mb|m|gb|g)\s*$")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;
        
        let caps = re.captures(s)
            .ok_or_else(|| anyhow!("Invalid size: {s} (try 10mb, 800k, 1.5gb)"))?;
        
        let num = caps.get(1).unwrap().as_str().parse::<f64>()?;
        let unit = caps.get(2).unwrap().as_str().to_lowercase();

        let mult: f64 = match unit.as_str() {
            "b" => 1.0,
            "kb" | "k" => 1024.0,
            "mb" | "m" => 1024.0 * 1024.0,
            "gb" | "g" => 1024.0 * 1024.0 * 1024.0,
            _ => bail!("Invalid unit in size: {s}"),
        };

        Ok(TargetSize {
            bytes: (num * mult).round() as u64,
        })
    }
}

impl fmt::Display for TargetSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bytes >= 1024 * 1024 * 1024 {
            write!(f, "{:.2} GB", self.bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if self.bytes >= 1024 * 1024 {
            write!(f, "{:.2} MB", self.bytes as f64 / (1024.0 * 1024.0))
        } else if self.bytes >= 1024 {
            write!(f, "{:.2} KB", self.bytes as f64 / 1024.0)
        } else {
            write!(f, "{} B", self.bytes)
        }
    }
}

/// Represents a resize target - either a preset (720p, 1080p, 4k) or explicit dimensions (WxH)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResizeTarget {
    Preset(ResolutionPreset),
    Dimensions { width: u32, height: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionPreset {
    P720,   // 1280x720
    P1080,  // 1920x1080
    P4K,    // 3840x2160
}

impl ResizeTarget {
    /// Parse resize target from string: "720p", "1080p", "4k" or "1280x720"
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.to_lowercase();
        
        // Check for presets (either ending with 'p' or "4k")
        if s_lower.ends_with('p') {
            let preset = match s_lower.as_str() {
                "720p" => ResolutionPreset::P720,
                "1080p" => ResolutionPreset::P1080,
                "2160p" => ResolutionPreset::P4K,
                _ => bail!("Unknown preset: {s} (try 720p, 1080p, 4k)"),
            };
            Ok(ResizeTarget::Preset(preset))
        } else if s_lower == "4k" {
            Ok(ResizeTarget::Preset(ResolutionPreset::P4K))
        } else {
            // WxH like 1280x720
            let re = Regex::new(r"^\s*(\d+)\s*x\s*(\d+)\s*$")
                .map_err(|e| anyhow!("Invalid regex: {}", e))?;
            
            let caps = re.captures(s)
                .ok_or_else(|| anyhow!("Invalid size: {s} (try 1280x720 or 720p)"))?;
            
            let width = caps.get(1).unwrap().as_str().parse::<u32>()?;
            let height = caps.get(2).unwrap().as_str().parse::<u32>()?;
            
            Ok(ResizeTarget::Dimensions { width, height })
        }
    }

    /// Get the scale filter string for FFmpeg
    pub fn to_ffmpeg_scale(&self) -> String {
        match self {
            ResizeTarget::Preset(preset) => match preset {
                ResolutionPreset::P720 => "scale=1280:720".to_string(),
                ResolutionPreset::P1080 => "scale=1920:1080".to_string(),
                ResolutionPreset::P4K => "scale=3840:2160".to_string(),
            },
            ResizeTarget::Dimensions { width, height } => {
                format!("scale={width}:{height}")
            }
        }
    }
}

impl fmt::Display for ResizeTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResizeTarget::Preset(preset) => match preset {
                ResolutionPreset::P720 => write!(f, "720p"),
                ResolutionPreset::P1080 => write!(f, "1080p"),
                ResolutionPreset::P4K => write!(f, "4K"),
            },
            ResizeTarget::Dimensions { width, height } => {
                write!(f, "{}x{}", width, height)
            }
        }
    }
}

/// Represents a speed factor that can be parsed from strings like "2x", "0.5x"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpeedFactor {
    pub factor: f64,
}

impl SpeedFactor {
    /// Parse speed factor from string format: "2x", "0.5x"
    pub fn parse(s: &str) -> Result<Self> {
        let re = Regex::new(r"(?i)^\s*(\d+(?:\.\d+)?)\s*x\s*$")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;
        
        let caps = re.captures(s)
            .ok_or_else(|| anyhow!("Invalid factor: {s} (try 2x, 0.5x)"))?;
        
        let factor = caps.get(1).unwrap().as_str().parse::<f64>()?;
        
        if factor <= 0.0 {
            bail!("Speed factor must be positive");
        }
        
        Ok(SpeedFactor { factor })
    }
}

impl fmt::Display for SpeedFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x", self.factor)
    }
}

/// Represents flip direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlipDirection {
    Horizontal,
    Vertical,
}

/// Represents rotation degrees (0, 90, 180, 270)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotateDegrees(pub i32);

impl RotateDegrees {
    pub fn new(degrees: i32) -> Result<Self> {
        let normalized = degrees.rem_euclid(360);
        match normalized {
            0 | 90 | 180 | 270 => Ok(RotateDegrees(normalized)),
            _ => bail!("Rotate supports 0/90/180/270 for now."),
        }
    }
}

/// Represents watermark position - corners or custom coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatermarkPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Custom { x: u32, y: u32 },
}

impl WatermarkPosition {
    /// Parse position from string: "top-left", "top-right", "bottom-left", "bottom-right", or "100,50"
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        
        // Check for corner positions
        match s_lower.as_str() {
            "top-left" | "topleft" => Ok(WatermarkPosition::TopLeft),
            "top-right" | "topright" => Ok(WatermarkPosition::TopRight),
            "bottom-left" | "bottomleft" => Ok(WatermarkPosition::BottomLeft),
            "bottom-right" | "bottomright" => Ok(WatermarkPosition::BottomRight),
            _ => {
                // Try to parse as custom coordinates "x,y"
                let re = Regex::new(r"^\s*(\d+)\s*,\s*(\d+)\s*$")
                    .map_err(|e| anyhow!("Invalid regex: {}", e))?;
                
                let caps = re.captures(&s_lower)
                    .ok_or_else(|| anyhow!("Invalid position: {s} (try top-left, top-right, bottom-left, bottom-right, or 100,50)"))?;
                
                let x = caps.get(1).unwrap().as_str().parse::<u32>()?;
                let y = caps.get(2).unwrap().as_str().parse::<u32>()?;
                
                Ok(WatermarkPosition::Custom { x, y })
            }
        }
    }
}

/// Represents watermark size - percentage or pixel dimensions
#[derive(Debug, Clone, PartialEq)]
pub enum WatermarkSize {
    Percentage(f64),  // 0.0 to 1.0
    Pixels { width: u32, height: Option<u32> },  // height=None maintains aspect ratio
}

impl WatermarkSize {
    /// Parse size from string: "20%" or "0.2" (percentage) or "200x100" or "200" (pixels)
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        
        // Check for percentage format (ends with % or is a decimal 0.0-1.0)
        if s.ends_with('%') {
            let num_str = s.trim_end_matches('%');
            let percentage = num_str.parse::<f64>()? / 100.0;
            if percentage < 0.0 || percentage > 1.0 {
                bail!("Percentage must be between 0% and 100%");
            }
            Ok(WatermarkSize::Percentage(percentage))
        } else if let Ok(decimal) = s.parse::<f64>() {
            // Check if it's a decimal between 0.0 and 1.0 (percentage without %)
            if decimal >= 0.0 && decimal <= 1.0 && s.contains('.') {
                Ok(WatermarkSize::Percentage(decimal))
            } else {
                // Treat as pixel width only
                let width = decimal as u32;
                Ok(WatermarkSize::Pixels { width, height: None })
            }
        } else {
            // Try to parse as pixel dimensions "WxH" or just "W"
            let re = Regex::new(r"^\s*(\d+)(?:\s*x\s*(\d+))?\s*$")
                .map_err(|e| anyhow!("Invalid regex: {}", e))?;
            
            let caps = re.captures(s)
                .ok_or_else(|| anyhow!("Invalid size: {s} (try 20%, 0.2, 200x100, or 200)"))?;
            
            let width = caps.get(1).unwrap().as_str().parse::<u32>()?;
            let height = caps.get(2)
                .map(|m| m.as_str().parse::<u32>())
                .transpose()?;
            
            Ok(WatermarkSize::Pixels { width, height })
        }
    }
}

/// Represents opacity value (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opacity(pub f64);

impl Opacity {
    pub fn new(value: f64) -> Result<Self> {
        if value < 0.0 || value > 1.0 {
            bail!("Opacity must be between 0.0 and 1.0");
        }
        Ok(Opacity(value))
    }
}

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// Represents text position - corners, center, or custom coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextPosition {
    TopLeft,
    TopRight,
    TopCenter,
    BottomLeft,
    BottomRight,
    BottomCenter,
    Center,
    Custom { x: u32, y: u32 },
}

impl TextPosition {
    /// Parse position from string: "top-left", "top-right", "top-center", "bottom-left", "bottom-right", "bottom-center", "center", or "x,y"
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        
        // Check for named positions
        match s_lower.as_str() {
            "top-left" | "topleft" => Ok(TextPosition::TopLeft),
            "top-right" | "topright" => Ok(TextPosition::TopRight),
            "top-center" | "topcenter" | "top" => Ok(TextPosition::TopCenter),
            "bottom-left" | "bottomleft" => Ok(TextPosition::BottomLeft),
            "bottom-right" | "bottomright" => Ok(TextPosition::BottomRight),
            "bottom-center" | "bottomcenter" | "bottom" => Ok(TextPosition::BottomCenter),
            "center" | "centre" => Ok(TextPosition::Center),
            _ => {
                // Try to parse as custom coordinates "x,y"
                let re = Regex::new(r"^\s*(\d+)\s*,\s*(\d+)\s*$")
                    .map_err(|e| anyhow!("Invalid regex: {}", e))?;
                
                let caps = re.captures(&s_lower)
                    .ok_or_else(|| anyhow!("Invalid position: {s} (try top-left, top-right, top-center, bottom-left, bottom-right, bottom-center, center, or 100,50)"))?;
                
                let x = caps.get(1).unwrap().as_str().parse::<u32>()?;
                let y = caps.get(2).unwrap().as_str().parse::<u32>()?;
                
                Ok(TextPosition::Custom { x, y })
            }
        }
    }
}

/// Represents text color in RGB format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TextColor {
    /// Parse color from string: "red", "white", "black", or hex "#RRGGBB" or "RRGGBB"
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim().to_lowercase();
        
        // Check for named colors
        let (r, g, b) = match s.as_str() {
            "white" => (255, 255, 255),
            "black" => (0, 0, 0),
            "red" => (255, 0, 0),
            "green" => (0, 255, 0),
            "blue" => (0, 0, 255),
            "yellow" => (255, 255, 0),
            "cyan" => (0, 255, 255),
            "magenta" => (255, 0, 255),
            _ => {
                // Try to parse as hex color
                let hex = s.trim_start_matches('#');
                if hex.len() == 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16)?;
                    let g = u8::from_str_radix(&hex[2..4], 16)?;
                    let b = u8::from_str_radix(&hex[4..6], 16)?;
                    (r, g, b)
                } else {
                    bail!("Invalid color: {s} (try named color like 'white' or hex like '#FFFFFF')");
                }
            }
        };
        
        Ok(TextColor { r, g, b })
    }
    
    /// Convert to FFmpeg color format (0xRRGGBB)
    pub fn to_ffmpeg(&self) -> String {
        format!("0x{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor { r: 255, g: 255, b: 255 } // White
    }
}

/// Represents text style configuration
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    pub font_size: Option<u32>,
    pub font_file: Option<String>,
    pub color: TextColor,
}

/// Represents video filter adjustments (brightness, contrast, saturation)
#[derive(Debug, Clone, PartialEq)]
pub struct FilterAdjustments {
    pub brightness: Option<f64>,  // -1.0 to 1.0, default 0.0
    pub contrast: Option<f64>,    // -1.0 to 1.0, default 0.0
    pub saturation: Option<f64>,  // -1.0 to 1.0, default 0.0
}

impl FilterAdjustments {
    pub fn new() -> Self {
        Self {
            brightness: None,
            contrast: None,
            saturation: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.brightness.is_none() && self.contrast.is_none() && self.saturation.is_none()
    }
}

impl Default for FilterAdjustments {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents color grading presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorPreset {
    Vintage,
    BlackAndWhite,
    Sepia,
}

impl ColorPreset {
    /// Parse preset from string: "vintage", "black-and-white", "sepia"
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "vintage" => Ok(ColorPreset::Vintage),
            "black-and-white" | "blackandwhite" | "bw" | "grayscale" => Ok(ColorPreset::BlackAndWhite),
            "sepia" => Ok(ColorPreset::Sepia),
            _ => bail!("Invalid color preset: {s} (try vintage, black-and-white, or sepia)"),
        }
    }
}

impl fmt::Display for ColorPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorPreset::Vintage => write!(f, "vintage"),
            ColorPreset::BlackAndWhite => write!(f, "black-and-white"),
            ColorPreset::Sepia => write!(f, "sepia"),
        }
    }
}

/// Represents a blur region with coordinates and dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlurRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl BlurRegion {
    /// Parse region from string format: "x,y,width,height" (e.g., "100,100,200,200")
    pub fn parse(s: &str) -> Result<Self> {
        let re = Regex::new(r"^\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*$")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;
        
        let caps = re.captures(s)
            .ok_or_else(|| anyhow!("Invalid region format: {s} (try 100,100,200,200)"))?;
        
        let x = caps.get(1).unwrap().as_str().parse::<u32>()?;
        let y = caps.get(2).unwrap().as_str().parse::<u32>()?;
        let width = caps.get(3).unwrap().as_str().parse::<u32>()?;
        let height = caps.get(4).unwrap().as_str().parse::<u32>()?;
        
        if width == 0 || height == 0 {
            bail!("Region width and height must be greater than 0");
        }
        
        Ok(BlurRegion { x, y, width, height })
    }
}

impl fmt::Display for BlurRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{},{}", self.x, self.y, self.width, self.height)
    }
}

/// Represents blur type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlurType {
    Region(BlurRegion),
    // Face detection could be added later
}

/// Represents a duration in seconds (for fade in/out, etc.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Duration {
    pub seconds: f64,
}

impl Duration {
    /// Parse duration from string formats: "2s", "1.5s", "30s", "2" (seconds)
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        let re = Regex::new(r"^(?i)(\d+(?:\.\d+)?)\s*s?$")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;
        
        let caps = re.captures(s)
            .ok_or_else(|| anyhow!("Invalid duration format: {s} (try 2s, 1.5s, or 2)"))?;
        
        let seconds = caps.get(1).unwrap().as_str().parse::<f64>()?;
        
        if seconds < 0.0 {
            bail!("Duration must be non-negative");
        }
        
        Ok(Duration { seconds })
    }
    
    /// Convert to total seconds as f64
    pub fn to_seconds(&self) -> f64 {
        self.seconds
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.seconds == self.seconds.floor() {
            write!(f, "{}s", self.seconds as u32)
        } else {
            write!(f, "{}s", self.seconds)
        }
    }
}

/// Represents compression target - either size or quality preset
#[derive(Debug, Clone, PartialEq)]
pub enum CompressTarget {
    Size(TargetSize),
    Quality(QualityPreset),
}

/// Represents how to split a video
#[derive(Debug, Clone, PartialEq)]
pub enum SplitMode {
    /// Split every N seconds
    Every(Duration),
    /// Split into N equal parts
    IntoParts(u32),
}

/// Represents a batch operation to apply to multiple files
#[derive(Debug, Clone, PartialEq)]
pub enum BatchOperation {
    Convert(ConvertFormat),
    // Other operations can be added here
}

/// Represents a condition for conditional batch processing
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingCondition {
    DurationLessThan(Duration),
    DurationGreaterThan(Duration),
    DurationEquals(Duration),
    // More conditions can be added here (file size, resolution, etc.)
}

/// Represents quality presets for compression and conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityPreset {
    Low,
    Medium,
    High,
    Ultra,
}

impl QualityPreset {
    /// Parse quality preset from string: "low", "medium", "high", "ultra"
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "low" => Ok(QualityPreset::Low),
            "medium" | "med" => Ok(QualityPreset::Medium),
            "high" => Ok(QualityPreset::High),
            "ultra" => Ok(QualityPreset::Ultra),
            _ => bail!("Invalid quality preset: {s} (try low, medium, high, or ultra)"),
        }
    }
    
    /// Get CRF value for video encoding (lower = higher quality)
    pub fn crf_value(&self) -> u8 {
        match self {
            QualityPreset::Low => 28,
            QualityPreset::Medium => 23,
            QualityPreset::High => 18,
            QualityPreset::Ultra => 15,
        }
    }
    
    /// Get approximate bitrate multiplier (relative to medium)
    pub fn bitrate_multiplier(&self) -> f64 {
        match self {
            QualityPreset::Low => 0.5,
            QualityPreset::Medium => 1.0,
            QualityPreset::High => 2.0,
            QualityPreset::Ultra => 4.0,
        }
    }
}

impl fmt::Display for QualityPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QualityPreset::Low => write!(f, "low"),
            QualityPreset::Medium => write!(f, "medium"),
            QualityPreset::High => write!(f, "high"),
            QualityPreset::Ultra => write!(f, "ultra"),
        }
    }
}

/// Represents video codec options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodec {
    H264,   // libx264
    H265,   // libx265 (HEVC)
    Vp9,    // libvpx-vp9
    Copy,   // Copy without re-encoding
}

impl VideoCodec {
    /// Get FFmpeg codec name
    pub fn ffmpeg_name(&self) -> &'static str {
        match self {
            VideoCodec::H264 => "libx264",
            VideoCodec::H265 => "libx265",
            VideoCodec::Vp9 => "libvpx-vp9",
            VideoCodec::Copy => "copy",
        }
    }
}

/// Represents montage layout (grid dimensions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MontageLayout {
    pub cols: u32,
    pub rows: u32,
}

impl MontageLayout {
    /// Parse layout from string format: "2x2", "3x1", etc.
    pub fn parse(s: &str) -> Result<Self> {
        let re = Regex::new(r"^\s*(\d+)\s*x\s*(\d+)\s*$")
            .map_err(|e| anyhow!("Invalid regex: {}", e))?;
        
        let caps = re.captures(s)
            .ok_or_else(|| anyhow!("Invalid layout format: {s} (try 2x2, 3x1)"))?;
        
        let cols = caps.get(1).unwrap().as_str().parse::<u32>()?;
        let rows = caps.get(2).unwrap().as_str().parse::<u32>()?;
        
        if cols == 0 || rows == 0 {
            bail!("Layout columns and rows must be greater than 0");
        }
        
        Ok(MontageLayout { cols, rows })
    }
    
    /// Get total cells needed
    pub fn total_cells(&self) -> u32 {
        self.cols * self.rows
    }
}

impl fmt::Display for MontageLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.cols, self.rows)
    }
}

/// Represents metadata field types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataField {
    Title,
    Author,
    Copyright,
    Comment,
    Description,
}

impl MetadataField {
    /// Parse metadata field from string
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "title" => Ok(MetadataField::Title),
            "author" => Ok(MetadataField::Author),
            "copyright" => Ok(MetadataField::Copyright),
            "comment" => Ok(MetadataField::Comment),
            "description" => Ok(MetadataField::Description),
            _ => bail!("Invalid metadata field: {s} (try title, author, copyright, comment, or description)"),
        }
    }
    
    /// Get FFmpeg metadata key name
    pub fn ffmpeg_key(&self) -> &'static str {
        match self {
            MetadataField::Title => "title",
            MetadataField::Author => "author",
            MetadataField::Copyright => "copyright",
            MetadataField::Comment => "comment",
            MetadataField::Description => "description",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_parse() {
        assert_eq!(Time::parse("30").unwrap(), Time { hours: 0, minutes: 0, seconds: 30 });
        assert_eq!(Time::parse("1:30").unwrap(), Time { hours: 0, minutes: 1, seconds: 30 });
        assert_eq!(Time::parse("1:05:30").unwrap(), Time { hours: 1, minutes: 5, seconds: 30 });
        assert_eq!(Time::parse("0:05").unwrap(), Time { hours: 0, minutes: 0, seconds: 5 });
        
        assert!(Time::parse("invalid").is_err());
    }

    #[test]
    fn test_time_to_ffmpeg() {
        assert_eq!(Time::parse("30").unwrap().to_ffmpeg(), "00:00:30");
        assert_eq!(Time::parse("1:30").unwrap().to_ffmpeg(), "00:01:30");
        assert_eq!(Time::parse("1:05:30").unwrap().to_ffmpeg(), "01:05:30");
    }

    #[test]
    fn test_target_size_parse() {
        assert_eq!(TargetSize::parse("10mb").unwrap().bytes, 10 * 1024 * 1024);
        assert_eq!(TargetSize::parse("800k").unwrap().bytes, 800 * 1024);
        assert_eq!(TargetSize::parse("1.5gb").unwrap().bytes, (1.5 * 1024.0 * 1024.0 * 1024.0) as u64);
        assert_eq!(TargetSize::parse("1024b").unwrap().bytes, 1024);
        
        assert!(TargetSize::parse("invalid").is_err());
    }

    #[test]
    fn test_resize_target_parse() {
        assert!(matches!(ResizeTarget::parse("720p").unwrap(), ResizeTarget::Preset(ResolutionPreset::P720)));
        assert!(matches!(ResizeTarget::parse("1080p").unwrap(), ResizeTarget::Preset(ResolutionPreset::P1080)));
        assert!(matches!(ResizeTarget::parse("4k").unwrap(), ResizeTarget::Preset(ResolutionPreset::P4K)));
        assert!(matches!(ResizeTarget::parse("4K").unwrap(), ResizeTarget::Preset(ResolutionPreset::P4K)));
        
        if let ResizeTarget::Dimensions { width, height } = ResizeTarget::parse("1280x720").unwrap() {
            assert_eq!(width, 1280);
            assert_eq!(height, 720);
        } else {
            panic!("Expected Dimensions variant");
        }
        
        assert!(ResizeTarget::parse("invalid").is_err());
    }

    #[test]
    fn test_speed_factor_parse() {
        assert_eq!(SpeedFactor::parse("2x").unwrap().factor, 2.0);
        assert_eq!(SpeedFactor::parse("0.5x").unwrap().factor, 0.5);
        assert_eq!(SpeedFactor::parse("1.5x").unwrap().factor, 1.5);
        
        assert!(SpeedFactor::parse("invalid").is_err());
        assert!(SpeedFactor::parse("0x").is_err());
        assert!(SpeedFactor::parse("-1x").is_err());
    }

    #[test]
    fn test_rotate_degrees() {
        assert_eq!(RotateDegrees::new(90).unwrap().0, 90);
        assert_eq!(RotateDegrees::new(180).unwrap().0, 180);
        assert_eq!(RotateDegrees::new(270).unwrap().0, 270);
        assert_eq!(RotateDegrees::new(0).unwrap().0, 0);
        assert_eq!(RotateDegrees::new(360).unwrap().0, 0);
        assert_eq!(RotateDegrees::new(450).unwrap().0, 90);
        
        assert!(RotateDegrees::new(45).is_err());
    }

    #[test]
    fn test_blur_region_parse() {
        let region = BlurRegion::parse("100,100,200,200").unwrap();
        assert_eq!(region.x, 100);
        assert_eq!(region.y, 100);
        assert_eq!(region.width, 200);
        assert_eq!(region.height, 200);
        
        let region2 = BlurRegion::parse("0,0,640,480").unwrap();
        assert_eq!(region2.x, 0);
        assert_eq!(region2.y, 0);
        assert_eq!(region2.width, 640);
        assert_eq!(region2.height, 480);
        
        // Test with spaces
        let region3 = BlurRegion::parse(" 50 , 60 , 100 , 150 ").unwrap();
        assert_eq!(region3.x, 50);
        assert_eq!(region3.y, 60);
        assert_eq!(region3.width, 100);
        assert_eq!(region3.height, 150);
        
        // Test invalid formats
        assert!(BlurRegion::parse("invalid").is_err());
        assert!(BlurRegion::parse("100,100").is_err());
        assert!(BlurRegion::parse("100,100,200").is_err());
        assert!(BlurRegion::parse("100,100,0,200").is_err()); // width = 0
        assert!(BlurRegion::parse("100,100,200,0").is_err()); // height = 0
    }

    #[test]
    fn test_duration_parse() {
        let dur = Duration::parse("2s").unwrap();
        assert_eq!(dur.seconds, 2.0);
        
        let dur2 = Duration::parse("1.5s").unwrap();
        assert_eq!(dur2.seconds, 1.5);
        
        let dur3 = Duration::parse("30").unwrap();
        assert_eq!(dur3.seconds, 30.0);
        
        let dur4 = Duration::parse("0.5s").unwrap();
        assert_eq!(dur4.seconds, 0.5);
        
        // Test with spaces
        let dur5 = Duration::parse(" 2s ").unwrap();
        assert_eq!(dur5.seconds, 2.0);
        
        // Test invalid formats
        assert!(Duration::parse("invalid").is_err());
        assert!(Duration::parse("-2s").is_err()); // negative
        assert!(Duration::parse("abc").is_err());
    }
}

/// Represents volume adjustment - either percentage or dB
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VolumeAdjustment {
    Percentage(f64),  // 0.0 to 100.0
    Decibels(f64),    // Can be positive or negative
}

impl VolumeAdjustment {
    /// Parse volume adjustment from string: "50%" or "+10db" or "-5db"
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim();
        let s_lower = s.to_lowercase();
        
        if s_lower.ends_with('%') {
            let num_str = s.trim_end_matches('%');
            let percentage = num_str.parse::<f64>()?;
            if percentage < 0.0 || percentage > 100.0 {
                bail!("Percentage must be between 0% and 100%");
            }
            Ok(VolumeAdjustment::Percentage(percentage))
        } else if s_lower.ends_with("db") || s_lower.ends_with("decibel") || s_lower.ends_with("decibels") {
            let num_str = if s_lower.ends_with("db") {
                s.trim_end_matches("db").trim_end_matches("dB")
            } else {
                s.trim_end_matches("decibel").trim_end_matches("decibels")
            };
            let db = num_str.parse::<f64>()?;
            Ok(VolumeAdjustment::Decibels(db))
        } else {
            bail!("Invalid volume adjustment: {s} (try 50% or +10db)");
        }
    }
    
    /// Convert to FFmpeg volume filter value
    /// For percentage: volume = percentage / 100.0
    /// For dB: volume = 10^(dB/20)
    pub fn to_ffmpeg_volume(&self) -> f64 {
        match self {
            VolumeAdjustment::Percentage(p) => *p / 100.0,
            VolumeAdjustment::Decibels(db) => 10.0_f64.powf(*db / 20.0),
        }
    }
}

impl fmt::Display for VolumeAdjustment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VolumeAdjustment::Percentage(p) => write!(f, "{}%", p),
            VolumeAdjustment::Decibels(db) => {
                if *db >= 0.0 {
                    write!(f, "+{}db", db)
                } else {
                    write!(f, "{}db", db)
                }
            }
        }
    }
}

/// Represents audio sync adjustment direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSyncDirection {
    Delay,   // Audio is delayed (audio comes later)
    Advance, // Audio is advanced (audio comes earlier)
}

impl AudioSyncDirection {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "delay" => Ok(AudioSyncDirection::Delay),
            "advance" => Ok(AudioSyncDirection::Advance),
            _ => bail!("Invalid sync direction: {s} (try 'delay' or 'advance')"),
        }
    }
}

impl fmt::Display for AudioSyncDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioSyncDirection::Delay => write!(f, "delay"),
            AudioSyncDirection::Advance => write!(f, "advance"),
        }
    }
}

/// Represents picture-in-picture position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl PipPosition {
    /// Parse position from string: "top-left", "top-right", "bottom-left", "bottom-right", "center"
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "top-left" | "topleft" => Ok(PipPosition::TopLeft),
            "top-right" | "topright" => Ok(PipPosition::TopRight),
            "bottom-left" | "bottomleft" => Ok(PipPosition::BottomLeft),
            "bottom-right" | "bottomright" => Ok(PipPosition::BottomRight),
            "center" | "centre" => Ok(PipPosition::Center),
            _ => bail!("Invalid PIP position: {s} (try top-left, top-right, bottom-left, bottom-right, or center)"),
        }
    }
}

/// Represents chroma key color for green screen removal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromaKeyColor {
    Green,
    Blue,
    Custom { r: u8, g: u8, b: u8 },
}

impl ChromaKeyColor {
    /// Parse color from string: "green", "blue", or hex "#RRGGBB" or "RRGGBB"
    pub fn parse(s: &str) -> Result<Self> {
        let s = s.trim().to_lowercase();
        match s.as_str() {
            "green" => Ok(ChromaKeyColor::Green),
            "blue" => Ok(ChromaKeyColor::Blue),
            _ => {
                // Try to parse as hex color
                let hex = s.trim_start_matches('#');
                if hex.len() == 6 {
                    let r = u8::from_str_radix(&hex[0..2], 16)?;
                    let g = u8::from_str_radix(&hex[2..4], 16)?;
                    let b = u8::from_str_radix(&hex[4..6], 16)?;
                    Ok(ChromaKeyColor::Custom { r, g, b })
                } else {
                    bail!("Invalid chroma key color: {s} (try green, blue, or hex like '#00FF00')");
                }
            }
        }
    }
    
    /// Convert to FFmpeg color format (0xRRGGBB)
    pub fn to_ffmpeg(&self) -> String {
        match self {
            ChromaKeyColor::Green => "0x00FF00".to_string(),
            ChromaKeyColor::Blue => "0x0000FF".to_string(),
            ChromaKeyColor::Custom { r, g, b } => format!("0x{:02X}{:02X}{:02X}", r, g, b),
        }
    }
}

/// Represents split screen orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitScreenOrientation {
    Horizontal,  // Side-by-side
    Vertical,     // Top/bottom
}

impl SplitScreenOrientation {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "horizontal" | "side-by-side" | "side" => Ok(SplitScreenOrientation::Horizontal),
            "vertical" | "top-bottom" | "top" | "bottom" => Ok(SplitScreenOrientation::Vertical),
            _ => bail!("Invalid split screen orientation: {s} (try horizontal or vertical)"),
        }
    }
}

/// Represents mirror direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MirrorDirection {
    Horizontal,
    Vertical,
}

impl MirrorDirection {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "horizontal" | "h" => Ok(MirrorDirection::Horizontal),
            "vertical" | "v" => Ok(MirrorDirection::Vertical),
            _ => bail!("Invalid mirror direction: {s} (try horizontal or vertical)"),
        }
    }
}

/// Represents color grading preset
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorGradePreset {
    Cinematic,
    Warm,
    Cool,
    Dramatic,
}

impl ColorGradePreset {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "cinematic" => Ok(ColorGradePreset::Cinematic),
            "warm" => Ok(ColorGradePreset::Warm),
            "cool" => Ok(ColorGradePreset::Cool),
            "dramatic" => Ok(ColorGradePreset::Dramatic),
            _ => bail!("Invalid color grade preset: {s} (try cinematic, warm, cool, or dramatic)"),
        }
    }
}

/// Represents text animation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAnimation {
    FadeIn,
    SlideIn,
    Typewriter,
}

impl TextAnimation {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "fade-in" | "fadein" | "fade" => Ok(TextAnimation::FadeIn),
            "slide-in" | "slidein" | "slide" => Ok(TextAnimation::SlideIn),
            "typewriter" | "type" => Ok(TextAnimation::Typewriter),
            _ => bail!("Invalid text animation: {s} (try fade-in, slide-in, or typewriter)"),
        }
    }
}

/// Represents transition type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Fade,
    Wipe,
    Slide,
}

impl TransitionType {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "fade" => Ok(TransitionType::Fade),
            "wipe" => Ok(TransitionType::Wipe),
            "slide" => Ok(TransitionType::Slide),
            _ => bail!("Invalid transition type: {s} (try fade, wipe, or slide)"),
        }
    }
}

/// Represents color space for conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colorspace {
    Rec709,
    Rec2020,
    P3,
    Srgb,
}

impl Colorspace {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "rec709" | "rec-709" | "bt709" | "bt-709" => Ok(Colorspace::Rec709),
            "rec2020" | "rec-2020" | "bt2020" | "bt-2020" => Ok(Colorspace::Rec2020),
            "p3" | "dci-p3" => Ok(Colorspace::P3),
            "srgb" | "s-rgb" => Ok(Colorspace::Srgb),
            _ => bail!("Invalid color space: {s} (try rec709, rec2020, p3, or srgb)"),
        }
    }
    
    pub fn to_ffmpeg(&self) -> &str {
        match self {
            Colorspace::Rec709 => "bt709",
            Colorspace::Rec2020 => "bt2020",
            Colorspace::P3 => "smpte170m", // P3 uses similar to rec709
            Colorspace::Srgb => "bt709", // sRGB typically uses rec709
        }
    }
}

/// Represents visualization style for audio-to-video conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationStyle {
    Waveform,
    Spectrum,
}

impl VisualizationStyle {
    pub fn parse(s: &str) -> Result<Self> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "waveform" => Ok(VisualizationStyle::Waveform),
            "spectrum" => Ok(VisualizationStyle::Spectrum),
            _ => anyhow::bail!("Invalid visualization style: {s} (try waveform or spectrum)"),
        }
    }
}

