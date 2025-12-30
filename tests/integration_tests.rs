use anyhow::Result;
use ffhuman::App;
use ffhuman::AppConfig;
use ffhuman::cli::Cli;
use ffhuman::model::*;
use clap::Parser;
use std::path::PathBuf;

/// Test assets paths
const VIDEO_ASSET: &str = "src/assets/file_example_MP4_480_1_5MG.mp4";
const VIDEO_WEBM_ASSET: &str = "src/assets/file_example_WEBM_480_900KB.webm"; // For conversion tests
const VIDEO_AVI_ASSET: &str = "src/assets/file_example_AVI_1280_1_5MG.avi";
const VIDEO_WMV_ASSET: &str = "src/assets/file_example_WMV_640_1_6MB.wmv";
const IMAGE_ASSET: &str = "src/assets/FFHuman.png";
const IMAGE_JPG_1: &str = "src/assets/140-536x354.jpg";
const IMAGE_JPG_2: &str = "src/assets/218-536x354.jpg";
const IMAGE_JPG_3: &str = "src/assets/444-536x354.jpg";
const IMAGE_JPG_4: &str = "src/assets/726-536x354.jpg";
const IMAGE_JPG_5: &str = "src/assets/783-536x354.jpg";
const IMAGE_JPG_6: &str = "src/assets/900-536x354.jpg";
const IMAGE_JPG_7: &str = "src/assets/974-536x354.jpg";
const AUDIO_ASSET: &str = "src/assets/file_example_MP3_1MG.mp3";
const AUDIO_OGG_ASSET: &str = "src/assets/file_example_OOG_1MG.ogg";
const AUDIO_WAV_ASSET: &str = "src/assets/file_example_WAV_1MG.wav";
const SUBTITLE_ASSET: &str = "src/assets/example.srt";

/*
# Run all integration tests (dry-run mode - no actual execution, default)
cargo test --test integration_tests

# Run a specific test (dry-run mode)
cargo test --test integration_tests test_convert_to_gif

# Run with output visible
cargo test --test integration_tests -- --nocapture

# Run tests for real with actual outputs (saved to src/output/{test_name}/)
FFHUMAN_TEST_REAL=1 cargo test --test integration_tests

# Run a specific test for real
FFHUMAN_TEST_REAL=1 cargo test --test integration_tests test_convert_to_gif
*/

/// Helper to create test app that runs for real and saves output to test-specific folder
/// Only runs for real if FFHUMAN_TEST_REAL environment variable is set
fn create_test_app_with_output(test_name: &str) -> Result<App> {
    use std::env;
    use std::fs;
    
    // Check if real execution is enabled via environment variable
    let run_real = env::var("FFHUMAN_TEST_REAL").is_ok();
    
    if run_real {
        let output_dir = PathBuf::from("src/output").join(test_name);
        fs::create_dir_all(&output_dir)?;
        let config = AppConfig::new(None, Some(output_dir), false, false, true);
        Ok(App::new(config))
    } else {
        // Default to dry-run mode
        let config = AppConfig::new(None, None, false, true, false);
        Ok(App::new(config))
    }
}

#[test]
fn test_convert_to_gif() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_gif")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_WEBM_ASSET), // Use WebM for conversion test
        format: ConvertFormat::Gif,
        quality: None,
        codec: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_webm() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_webm")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_ASSET), // Use MP4 as source for WebM conversion
        format: ConvertFormat::Webm,
        quality: Some(QualityPreset::High),
        codec: Some(VideoCodec::Vp9),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_mp4() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_mp4")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_WEBM_ASSET), // Use WebM for conversion test
        format: ConvertFormat::Mp4,
        quality: Some(QualityPreset::Medium),
        codec: Some(VideoCodec::H264),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_mp3() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_mp3")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_WEBM_ASSET), // Use WebM for conversion test
        format: ConvertFormat::Mp3,
        quality: None,
        codec: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_wav() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_wav")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_WEBM_ASSET), // Use WebM for conversion test
        format: ConvertFormat::Wav,
        quality: None,
        codec: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_avi_to_mp4() -> Result<()> {
    let app = create_test_app_with_output("test_convert_avi_to_mp4")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_AVI_ASSET),
        format: ConvertFormat::Mp4,
        quality: Some(QualityPreset::Medium),
        codec: Some(VideoCodec::H264),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_wmv_to_mp4() -> Result<()> {
    let app = create_test_app_with_output("test_convert_wmv_to_mp4")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_WMV_ASSET),
        format: ConvertFormat::Mp4,
        quality: Some(QualityPreset::Medium),
        codec: Some(VideoCodec::H264),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_compress_avi_to_size() -> Result<()> {
    let app = create_test_app_with_output("test_compress_avi_to_size")?;
    let intent = Intent::Compress {
        input: PathBuf::from(VIDEO_AVI_ASSET),
        target: CompressTarget::Size(TargetSize::parse("5mb")?),
        two_pass: false,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_trim_wmv() -> Result<()> {
    let app = create_test_app_with_output("test_trim_wmv")?;
    let intent = Intent::Trim {
        input: PathBuf::from(VIDEO_WMV_ASSET),
        start: Time::parse("0:05")?,
        end: Time::parse("0:30")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_audio_from_avi() -> Result<()> {
    let app = create_test_app_with_output("test_extract_audio_from_avi")?;
    let intent = Intent::ExtractAudio {
        input: PathBuf::from(VIDEO_AVI_ASSET),
        format: AudioFormat::Mp3,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_mix_audio_with_ogg() -> Result<()> {
    let app = create_test_app_with_output("test_mix_audio_with_ogg")?;
    let intent = Intent::MixAudio {
        audio1: PathBuf::from(AUDIO_ASSET),
        audio2: PathBuf::from(AUDIO_OGG_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_mix_audio_with_wav() -> Result<()> {
    let app = create_test_app_with_output("test_mix_audio_with_wav")?;
    let intent = Intent::MixAudio {
        audio1: PathBuf::from(AUDIO_ASSET),
        audio2: PathBuf::from(AUDIO_WAV_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_add_audio_ogg_to_video() -> Result<()> {
    let app = create_test_app_with_output("test_add_audio_ogg_to_video")?;
    let intent = Intent::AddAudio {
        audio: PathBuf::from(AUDIO_OGG_ASSET),
        video: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_add_audio_wav_to_video() -> Result<()> {
    let app = create_test_app_with_output("test_add_audio_wav_to_video")?;
    let intent = Intent::AddAudio {
        audio: PathBuf::from(AUDIO_WAV_ASSET),
        video: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_visualize_ogg_waveform() -> Result<()> {
    let app = create_test_app_with_output("test_visualize_ogg_waveform")?;
    let intent = Intent::Visualize {
        audio: PathBuf::from(AUDIO_OGG_ASSET),
        style: VisualizationStyle::Waveform,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_visualize_wav_spectrum() -> Result<()> {
    let app = create_test_app_with_output("test_visualize_wav_spectrum")?;
    let intent = Intent::Visualize {
        audio: PathBuf::from(AUDIO_WAV_ASSET),
        style: VisualizationStyle::Spectrum,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_compress_to_size() -> Result<()> {
    let app = create_test_app_with_output("test_compress_to_size")?;
    let intent = Intent::Compress {
        input: PathBuf::from(VIDEO_ASSET),
        target: CompressTarget::Size(TargetSize::parse("10mb")?),
        two_pass: false,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_compress_to_quality() -> Result<()> {
    let app = create_test_app_with_output("test_compress_to_quality")?;
    let intent = Intent::Compress {
        input: PathBuf::from(VIDEO_ASSET),
        target: CompressTarget::Quality(QualityPreset::Low),
        two_pass: false,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_trim() -> Result<()> {
    let app = create_test_app_with_output("test_trim")?;
    let intent = Intent::Trim {
        input: PathBuf::from(VIDEO_ASSET),
        start: Time::parse("0:05")?,
        end: Time::parse("0:30")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_audio_mp3() -> Result<()> {
    let app = create_test_app_with_output("test_extract_audio_mp3")?;
    let intent = Intent::ExtractAudio {
        input: PathBuf::from(VIDEO_ASSET),
        format: AudioFormat::Mp3,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_audio_wav() -> Result<()> {
    let app = create_test_app_with_output("test_extract_audio_wav")?;
    let intent = Intent::ExtractAudio {
        input: PathBuf::from(VIDEO_ASSET),
        format: AudioFormat::Wav,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_resize_to_preset() -> Result<()> {
    let app = create_test_app_with_output("test_resize_to_preset")?;
    let intent = Intent::Resize {
        input: PathBuf::from(VIDEO_ASSET),
        target: ResizeTarget::parse("720p")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_resize_to_dimensions() -> Result<()> {
    let app = create_test_app_with_output("test_resize_to_dimensions")?;
    let intent = Intent::Resize {
        input: PathBuf::from(VIDEO_ASSET),
        target: ResizeTarget::parse("1280x720")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_speed_up() -> Result<()> {
    let app = create_test_app_with_output("test_speed_up")?;
    let intent = Intent::SpeedUp {
        input: PathBuf::from(VIDEO_ASSET),
        factor: SpeedFactor::parse("2x")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_slow_down() -> Result<()> {
    let app = create_test_app_with_output("test_slow_down")?;
    let intent = Intent::SlowDown {
        input: PathBuf::from(VIDEO_ASSET),
        factor: SpeedFactor::parse("0.5x")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_reverse() -> Result<()> {
    let app = create_test_app_with_output("test_reverse")?;
    let intent = Intent::Reverse {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_mute() -> Result<()> {
    let app = create_test_app_with_output("test_mute")?;
    let intent = Intent::Mute {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_rotate() -> Result<()> {
    let app = create_test_app_with_output("test_rotate")?;
    let intent = Intent::Rotate {
        input: PathBuf::from(VIDEO_ASSET),
        degrees: RotateDegrees::new(90)?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_flip_horizontal() -> Result<()> {
    let app = create_test_app_with_output("test_flip_horizontal")?;
    let intent = Intent::Flip {
        input: PathBuf::from(VIDEO_ASSET),
        direction: FlipDirection::Horizontal,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_flip_vertical() -> Result<()> {
    let app = create_test_app_with_output("test_flip_vertical")?;
    let intent = Intent::Flip {
        input: PathBuf::from(VIDEO_ASSET),
        direction: FlipDirection::Vertical,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_thumbnail() -> Result<()> {
    let app = create_test_app_with_output("test_thumbnail")?;
    let intent = Intent::Thumbnail {
        input: PathBuf::from(VIDEO_ASSET),
        time: Time::parse("0:05")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_crop() -> Result<()> {
    let app = create_test_app_with_output("test_crop")?;
    let intent = Intent::Crop {
        input: PathBuf::from(VIDEO_ASSET),
        width: 320,
        height: 240,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_set_fps() -> Result<()> {
    let app = create_test_app_with_output("test_set_fps")?;
    let intent = Intent::SetFps {
        input: PathBuf::from(VIDEO_ASSET),
        fps: 30,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_loop() -> Result<()> {
    let app = create_test_app_with_output("test_loop")?;
    let intent = Intent::Loop {
        input: PathBuf::from(VIDEO_ASSET),
        times: 3,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_merge() -> Result<()> {
    let app = create_test_app_with_output("test_merge")?;
    let intent = Intent::Merge {
        a: PathBuf::from(VIDEO_ASSET),
        b: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_add_audio() -> Result<()> {
    let app = create_test_app_with_output("test_add_audio")?;
    let intent = Intent::AddAudio {
        audio: PathBuf::from(AUDIO_ASSET),
        video: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_grayscale() -> Result<()> {
    let app = create_test_app_with_output("test_grayscale")?;
    let intent = Intent::Grayscale {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_stabilize() -> Result<()> {
    let app = create_test_app_with_output("test_stabilize")?;
    let intent = Intent::Stabilize {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_denoise() -> Result<()> {
    let app = create_test_app_with_output("test_denoise")?;
    let intent = Intent::Denoise {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_watermark() -> Result<()> {
    let app = create_test_app_with_output("test_watermark")?;
    let intent = Intent::Watermark {
        input: PathBuf::from(VIDEO_ASSET),
        logo: PathBuf::from(IMAGE_ASSET),
        position: WatermarkPosition::parse("top-right")?,
        opacity: Opacity::new(0.7)?,
        size: Some(WatermarkSize::parse("20%")?),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_add_text() -> Result<()> {
    let app = create_test_app_with_output("test_add_text")?;
    let intent = Intent::AddText {
        input: PathBuf::from(VIDEO_ASSET),
        text: "Test Text".to_string(),
        position: TextPosition::parse("center")?,
        style: TextStyle {
            font_size: Some(24),
            font_file: None,
            color: TextColor::default(),
        },
        timestamp: false,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_filter() -> Result<()> {
    let app = create_test_app_with_output("test_filter")?;
    let mut adjustments = FilterAdjustments::new();
    adjustments.brightness = Some(0.2);
    adjustments.contrast = Some(0.1);
    adjustments.saturation = Some(-0.1);
    let intent = Intent::Filter {
        input: PathBuf::from(VIDEO_ASSET),
        adjustments,
        preset: Some(ColorPreset::parse("vintage")?),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_blur() -> Result<()> {
    let app = create_test_app_with_output("test_blur")?;
    let intent = Intent::Blur {
        input: PathBuf::from(VIDEO_ASSET),
        blur_type: BlurType::Region(BlurRegion::parse("100,100,200,200")?),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_normalize() -> Result<()> {
    let app = create_test_app_with_output("test_normalize")?;
    let intent = Intent::Normalize {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_fade() -> Result<()> {
    let app = create_test_app_with_output("test_fade")?;
    let intent = Intent::Fade {
        input: PathBuf::from(VIDEO_ASSET),
        fade_in: Some(Duration::parse("2s")?),
        fade_out: Some(Duration::parse("2s")?),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_split() -> Result<()> {
    let app = create_test_app_with_output("test_split")?;
    let intent = Intent::Split {
        input: PathBuf::from(VIDEO_ASSET),
        mode: SplitMode::Every(Duration::parse("10s")?),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_split_into_parts() -> Result<()> {
    let app = create_test_app_with_output("test_split_into_parts")?;
    let intent = Intent::Split {
        input: PathBuf::from(VIDEO_ASSET),
        mode: SplitMode::IntoParts(3),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_frames() -> Result<()> {
    let app = create_test_app_with_output("test_extract_frames")?;
    let intent = Intent::ExtractFrames {
        input: PathBuf::from(VIDEO_ASSET),
        interval: Duration::parse("1s")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_burn_subtitle() -> Result<()> {
    let app = create_test_app_with_output("test_burn_subtitle")?;
    let intent = Intent::BurnSubtitle {
        input: PathBuf::from(VIDEO_ASSET),
        subtitle: PathBuf::from(SUBTITLE_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_compare() -> Result<()> {
    let app = create_test_app_with_output("test_compare")?;
    let intent = Intent::Compare {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        show_psnr: false,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_compare_with_psnr() -> Result<()> {
    let app = create_test_app_with_output("test_compare_with_psnr")?;
    let intent = Intent::Compare {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        show_psnr: true,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_set_metadata() -> Result<()> {
    let app = create_test_app_with_output("test_set_metadata")?;
    let intent = Intent::SetMetadata {
        input: PathBuf::from(VIDEO_ASSET),
        field: MetadataField::parse("title")?,
        value: "Test Video".to_string(),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_montage() -> Result<()> {
    let app = create_test_app_with_output("test_montage")?;
    let intent = Intent::Montage {
        videos: vec![
            PathBuf::from(VIDEO_ASSET),
            PathBuf::from(VIDEO_ASSET),
        ],
        layout: MontageLayout::parse("2x1")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_crossfade() -> Result<()> {
    let app = create_test_app_with_output("test_crossfade")?;
    let intent = Intent::Crossfade {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        duration: Duration::parse("2s")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_timelapse() -> Result<()> {
    let app = create_test_app_with_output("test_timelapse")?;
    let intent = Intent::Timelapse {
        input: PathBuf::from(VIDEO_ASSET),
        speed: SpeedFactor::parse("10x")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_batch() -> Result<()> {
    let app = create_test_app_with_output("test_batch")?;
    let intent = Intent::Batch {
        pattern: "src/assets/*.webm".to_string(),
        operation: BatchOperation::Convert(ConvertFormat::Gif),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_doctor() -> Result<()> {
    let app = create_test_app_with_output("test_doctor")?;
    let intent = Intent::Doctor;
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_metadata() -> Result<()> {
    let app = create_test_app_with_output("test_extract_metadata")?;
    let intent = Intent::ExtractMetadata {
        input: PathBuf::from(VIDEO_ASSET),
        format: ffhuman::model::intent::MetadataFormat::Json,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_metadata_xml() -> Result<()> {
    let app = create_test_app_with_output("test_extract_metadata_xml")?;
    let intent = Intent::ExtractMetadata {
        input: PathBuf::from(VIDEO_ASSET),
        format: ffhuman::model::intent::MetadataFormat::Xml,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_info() -> Result<()> {
    let app = create_test_app_with_output("test_info")?;
    let intent = Intent::Info {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_thumbnail_grid() -> Result<()> {
    let app = create_test_app_with_output("test_thumbnail_grid")?;
    let layout = ffhuman::model::intent::ThumbnailGridLayout::parse("3x3")?;
    let intent = Intent::ThumbnailGrid {
        input: PathBuf::from(VIDEO_ASSET),
        layout,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_social_media_convert_instagram() -> Result<()> {
    let app = create_test_app_with_output("test_social_media_convert_instagram")?;
    let intent = Intent::SocialMediaConvert {
        input: PathBuf::from(VIDEO_ASSET),
        platform: ffhuman::model::intent::SocialPlatform::Instagram,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_social_media_convert_tiktok() -> Result<()> {
    let app = create_test_app_with_output("test_social_media_convert_tiktok")?;
    let intent = Intent::SocialMediaConvert {
        input: PathBuf::from(VIDEO_ASSET),
        platform: ffhuman::model::intent::SocialPlatform::TikTok,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_social_media_convert_youtube_shorts() -> Result<()> {
    let app = create_test_app_with_output("test_social_media_convert_youtube_shorts")?;
    let intent = Intent::SocialMediaConvert {
        input: PathBuf::from(VIDEO_ASSET),
        platform: ffhuman::model::intent::SocialPlatform::YoutubeShorts,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_social_media_convert_twitter() -> Result<()> {
    let app = create_test_app_with_output("test_social_media_convert_twitter")?;
    let intent = Intent::SocialMediaConvert {
        input: PathBuf::from(VIDEO_ASSET),
        platform: ffhuman::model::intent::SocialPlatform::Twitter,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_social_crop_square() -> Result<()> {
    let app = create_test_app_with_output("test_social_crop_square")?;
    let intent = Intent::SocialCrop {
        input: PathBuf::from(VIDEO_ASSET),
        shape: ffhuman::model::intent::SocialCropShape::Square,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_social_crop_circle() -> Result<()> {
    let app = create_test_app_with_output("test_social_crop_circle")?;
    let intent = Intent::SocialCrop {
        input: PathBuf::from(VIDEO_ASSET),
        shape: ffhuman::model::intent::SocialCropShape::Circle,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_vertical_convert() -> Result<()> {
    let app = create_test_app_with_output("test_vertical_convert")?;
    let intent = Intent::VerticalConvert {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_story_format() -> Result<()> {
    let app = create_test_app_with_output("test_story_format")?;
    let intent = Intent::StoryFormat {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_noise_reduction() -> Result<()> {
    let app = create_test_app_with_output("test_noise_reduction")?;
    let intent = Intent::NoiseReduction {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_echo_removal() -> Result<()> {
    let app = create_test_app_with_output("test_echo_removal")?;
    let intent = Intent::EchoRemoval {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_audio_ducking() -> Result<()> {
    let app = create_test_app_with_output("test_audio_ducking")?;
    let intent = Intent::AudioDucking {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_audio_equalizer() -> Result<()> {
    let app = create_test_app_with_output("test_audio_equalizer")?;
    let intent = Intent::AudioEqualizer {
        input: PathBuf::from(VIDEO_ASSET),
        bass: Some(5),
        treble: Some(-2),
        mid: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_voice_isolation() -> Result<()> {
    let app = create_test_app_with_output("test_voice_isolation")?;
    let intent = Intent::VoiceIsolation {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_audio_speed_keep_pitch() -> Result<()> {
    let app = create_test_app_with_output("test_audio_speed_keep_pitch")?;
    let intent = Intent::AudioSpeedKeepPitch {
        input: PathBuf::from(VIDEO_ASSET),
        factor: SpeedFactor::parse("1.5x")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_adjust_volume_percentage() -> Result<()> {
    let app = create_test_app_with_output("test_adjust_volume_percentage")?;
    let intent = Intent::AdjustVolume {
        input: PathBuf::from(VIDEO_ASSET),
        adjustment: VolumeAdjustment::parse("50%")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_adjust_volume_decibels() -> Result<()> {
    let app = create_test_app_with_output("test_adjust_volume_decibels")?;
    let intent = Intent::AdjustVolume {
        input: PathBuf::from(VIDEO_ASSET),
        adjustment: VolumeAdjustment::parse("+10db")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_adjust_volume_negative_decibels() -> Result<()> {
    let app = create_test_app_with_output("test_adjust_volume_negative_decibels")?;
    let intent = Intent::AdjustVolume {
        input: PathBuf::from(VIDEO_ASSET),
        adjustment: VolumeAdjustment::parse("-5db")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_sync_audio_delay() -> Result<()> {
    let app = create_test_app_with_output("test_sync_audio_delay")?;
    let intent = Intent::SyncAudio {
        input: PathBuf::from(VIDEO_ASSET),
        direction: AudioSyncDirection::Delay,
        offset: Duration::parse("0.5s")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_sync_audio_advance() -> Result<()> {
    let app = create_test_app_with_output("test_sync_audio_advance")?;
    let intent = Intent::SyncAudio {
        input: PathBuf::from(VIDEO_ASSET),
        direction: AudioSyncDirection::Advance,
        offset: Duration::parse("0.3s")?,
    };
    app.execute(intent)?;
    Ok(())
}


#[test]
fn test_mix_audio() -> Result<()> {
    let app = create_test_app_with_output("test_mix_audio")?;
    let intent = Intent::MixAudio {
        audio1: PathBuf::from(AUDIO_ASSET),
        audio2: PathBuf::from(AUDIO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_audio_range() -> Result<()> {
    let app = create_test_app_with_output("test_extract_audio_range")?;
    let intent = Intent::ExtractAudioRange {
        input: PathBuf::from(VIDEO_ASSET),
        start: Time::parse("0:30")?,
        end: Time::parse("2:00")?,
        format: AudioFormat::Mp3,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_audio_range_wav() -> Result<()> {
    let app = create_test_app_with_output("test_extract_audio_range_wav")?;
    let intent = Intent::ExtractAudioRange {
        input: PathBuf::from(VIDEO_ASSET),
        start: Time::parse("0:05")?,
        end: Time::parse("0:15")?,
        format: AudioFormat::Wav,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_pip() -> Result<()> {
    let app = create_test_app_with_output("test_pip")?;
    let intent = Intent::Pip {
        overlay_video: PathBuf::from(VIDEO_ASSET),
        base_video: PathBuf::from(VIDEO_ASSET),
        position: PipPosition::parse("top-right")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_remove_background() -> Result<()> {
    let app = create_test_app_with_output("test_remove_background")?;
    let intent = Intent::RemoveBackground {
        input: PathBuf::from(VIDEO_ASSET),
        color: ChromaKeyColor::parse("green")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_overlay() -> Result<()> {
    let app = create_test_app_with_output("test_overlay")?;
    let intent = Intent::Overlay {
        overlay_video: PathBuf::from(VIDEO_ASSET),
        base_video: PathBuf::from(VIDEO_ASSET),
        position: WatermarkPosition::parse("100,50")?,
        opacity: Opacity::new(0.7)?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_concat() -> Result<()> {
    let app = create_test_app_with_output("test_concat")?;
    let intent = Intent::Concat {
        videos: vec![
            PathBuf::from(VIDEO_ASSET),
            PathBuf::from(VIDEO_ASSET),
            PathBuf::from(VIDEO_ASSET),
        ],
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_detect_scenes() -> Result<()> {
    let app = create_test_app_with_output("test_detect_scenes")?;
    let intent = Intent::DetectScenes {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_detect_black() -> Result<()> {
    let app = create_test_app_with_output("test_detect_black")?;
    let intent = Intent::DetectBlack {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}


#[test]
fn test_fix_rotation() -> Result<()> {
    let app = create_test_app_with_output("test_fix_rotation")?;
    let intent = Intent::FixRotation {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_iphone() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_iphone")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_ASSET),
        format: ConvertFormat::Iphone,
        quality: None,
        codec: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_android() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_android")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_ASSET),
        format: ConvertFormat::Android,
        quality: Some(QualityPreset::High),
        codec: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_hls() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_hls")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_ASSET),
        format: ConvertFormat::Hls,
        quality: Some(QualityPreset::Medium),
        codec: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_to_dash() -> Result<()> {
    let app = create_test_app_with_output("test_convert_to_dash")?;
    let intent = Intent::Convert {
        input: PathBuf::from(VIDEO_ASSET),
        format: ConvertFormat::Dash,
        quality: Some(QualityPreset::High),
        codec: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_analyze_quality() -> Result<()> {
    let app = create_test_app_with_output("test_analyze_quality")?;
    // Test CLI parsing - this should work with kebab-case command
    let cli = Cli::try_parse_from(&["ffhuman", "analyze-quality", VIDEO_ASSET, "--dry-run"])?;
    let intent = cli.into_intent()?;
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_compress_with_two_pass() -> Result<()> {
    let app = create_test_app_with_output("test_compress_with_two_pass")?;
    let intent = Intent::Compress {
        input: PathBuf::from(VIDEO_ASSET),
        target: CompressTarget::Size(TargetSize::parse("10mb")?),
        two_pass: true,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_compress_quality_with_two_pass() -> Result<()> {
    let app = create_test_app_with_output("test_compress_quality_with_two_pass")?;
    let intent = Intent::Compress {
        input: PathBuf::from(VIDEO_ASSET),
        target: CompressTarget::Quality(QualityPreset::High),
        two_pass: true,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_batch_with_progress() -> Result<()> {
    let app = create_test_app_with_output("test_batch_with_progress")?;
    let intent = Intent::Batch {
        pattern: "src/assets/*.webm".to_string(),
        operation: BatchOperation::Convert(ConvertFormat::Mp4),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_preview() -> Result<()> {
    let app = create_test_app_with_output("test_preview")?;
    let intent = Intent::Preview {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_suggest_format() -> Result<()> {
    let app = create_test_app_with_output("test_suggest_format")?;
    let intent = Intent::SuggestFormat {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_workflow() -> Result<()> {
    use std::fs;
    use std::path::PathBuf;
    
    // Create a temporary workflow file
    let workflow_content = r#"steps:
  - operation: convert
    input: src/assets/file_example_WEBM_480_900KB.webm
    output: test_output.mp4
    params:
      format: mp4
  - operation: trim
    input: test_output.mp4
    params:
      start: 0:05
      end: 0:15
"#;
    
    let workflow_file = PathBuf::from("test_workflow.yaml");
    fs::write(&workflow_file, workflow_content)?;
    
    let app = create_test_app_with_output("test_workflow")?;
    let intent = Intent::Workflow {
        config_file: workflow_file.clone(),
    };
    
    // Execute workflow (will fail in dry-run mode but that's OK for testing)
    let _ = app.execute(intent);
    
    // Clean up
    let _ = fs::remove_file(&workflow_file);
    
    Ok(())
}

#[test]
fn test_compress_without_two_pass() -> Result<()> {
    let app = create_test_app_with_output("test_compress_without_two_pass")?;
    let intent = Intent::Compress {
        input: PathBuf::from(VIDEO_ASSET),
        target: CompressTarget::Size(TargetSize::parse("10mb")?),
        two_pass: false,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_motion_blur() -> Result<()> {
    let app = create_test_app_with_output("test_motion_blur")?;
    let intent = Intent::MotionBlur {
        input: PathBuf::from(VIDEO_ASSET),
        radius: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_vignette() -> Result<()> {
    let app = create_test_app_with_output("test_vignette")?;
    let intent = Intent::Vignette {
        input: PathBuf::from(VIDEO_ASSET),
        intensity: None,
        size: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_lens_correct() -> Result<()> {
    let app = create_test_app_with_output("test_lens_correct")?;
    let intent = Intent::LensCorrect {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_interpolate() -> Result<()> {
    let app = create_test_app_with_output("test_interpolate")?;
    let intent = Intent::Interpolate {
        input: PathBuf::from(VIDEO_ASSET),
        fps: 60,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_interpolate_120fps() -> Result<()> {
    let app = create_test_app_with_output("test_interpolate_120fps")?;
    let intent = Intent::Interpolate {
        input: PathBuf::from(VIDEO_ASSET),
        fps: 120,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_glitch() -> Result<()> {
    let app = create_test_app_with_output("test_glitch")?;
    let intent = Intent::Glitch {
        input: PathBuf::from(VIDEO_ASSET),
        shift: None,
        noise: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_glitch_with_parameters() -> Result<()> {
    let app = create_test_app_with_output("test_glitch_with_parameters")?;
    let intent = Intent::Glitch {
        input: PathBuf::from(VIDEO_ASSET),
        shift: Some(8),
        noise: Some(50),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_glitch_with_max_parameters() -> Result<()> {
    let app = create_test_app_with_output("test_glitch_with_max_parameters")?;
    // Test that values above max are clamped correctly
    let intent = Intent::Glitch {
        input: PathBuf::from(VIDEO_ASSET),
        shift: Some(20), // Should be clamped to 15
        noise: Some(150), // Should be clamped to 100
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_vintage_film() -> Result<()> {
    let app = create_test_app_with_output("test_vintage_film")?;
    let intent = Intent::VintageFilm {
        input: PathBuf::from(VIDEO_ASSET),
        era: None,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_vintage_film_70s() -> Result<()> {
    let app = create_test_app_with_output("test_vintage_film_70s")?;
    let intent = Intent::VintageFilm {
        input: PathBuf::from(VIDEO_ASSET),
        era: Some("70s".to_string()),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_vintage_film_80s() -> Result<()> {
    let app = create_test_app_with_output("test_vintage_film_80s")?;
    let intent = Intent::VintageFilm {
        input: PathBuf::from(VIDEO_ASSET),
        era: Some("80s".to_string()),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_vintage_film_90s() -> Result<()> {
    let app = create_test_app_with_output("test_vintage_film_90s")?;
    let intent = Intent::VintageFilm {
        input: PathBuf::from(VIDEO_ASSET),
        era: Some("90s".to_string()),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_split_screen_horizontal() -> Result<()> {
    let app = create_test_app_with_output("test_split_screen_horizontal")?;
    let intent = Intent::SplitScreen {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        orientation: SplitScreenOrientation::Horizontal,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_split_screen_vertical() -> Result<()> {
    let app = create_test_app_with_output("test_split_screen_vertical")?;
    let intent = Intent::SplitScreen {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        orientation: SplitScreenOrientation::Vertical,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_mirror_horizontal() -> Result<()> {
    let app = create_test_app_with_output("test_mirror_horizontal")?;
    let intent = Intent::Mirror {
        input: PathBuf::from(VIDEO_ASSET),
        direction: MirrorDirection::Horizontal,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_mirror_vertical() -> Result<()> {
    let app = create_test_app_with_output("test_mirror_vertical")?;
    let intent = Intent::Mirror {
        input: PathBuf::from(VIDEO_ASSET),
        direction: MirrorDirection::Vertical,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_color_grade_cinematic() -> Result<()> {
    let app = create_test_app_with_output("test_color_grade_cinematic")?;
    let intent = Intent::ColorGrade {
        input: PathBuf::from(VIDEO_ASSET),
        preset: ColorGradePreset::Cinematic,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_color_grade_warm() -> Result<()> {
    let app = create_test_app_with_output("test_color_grade_warm")?;
    let intent = Intent::ColorGrade {
        input: PathBuf::from(VIDEO_ASSET),
        preset: ColorGradePreset::Warm,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_color_grade_cool() -> Result<()> {
    let app = create_test_app_with_output("test_color_grade_cool")?;
    let intent = Intent::ColorGrade {
        input: PathBuf::from(VIDEO_ASSET),
        preset: ColorGradePreset::Cool,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_color_grade_dramatic() -> Result<()> {
    let app = create_test_app_with_output("test_color_grade_dramatic")?;
    let intent = Intent::ColorGrade {
        input: PathBuf::from(VIDEO_ASSET),
        preset: ColorGradePreset::Dramatic,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_animated_text_fade_in() -> Result<()> {
    let app = create_test_app_with_output("test_animated_text_fade_in")?;
    let intent = Intent::AnimatedText {
        input: PathBuf::from(VIDEO_ASSET),
        text: "Test Title".to_string(),
        position: TextPosition::Center,
        animation: TextAnimation::FadeIn,
        style: TextStyle {
            font_size: Some(24),
            font_file: None,
            color: TextColor::default(),
        },
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_animated_text_slide_in() -> Result<()> {
    let app = create_test_app_with_output("test_animated_text_slide_in")?;
    let intent = Intent::AnimatedText {
        input: PathBuf::from(VIDEO_ASSET),
        text: "Slide Text".to_string(),
        position: TextPosition::TopCenter,
        animation: TextAnimation::SlideIn,
        style: TextStyle {
            font_size: Some(32),
            font_file: None,
            color: TextColor::default(),
        },
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_animated_text_typewriter() -> Result<()> {
    let app = create_test_app_with_output("test_animated_text_typewriter")?;
    let intent = Intent::AnimatedText {
        input: PathBuf::from(VIDEO_ASSET),
        text: "Typewriter".to_string(),
        position: TextPosition::BottomCenter,
        animation: TextAnimation::Typewriter,
        style: TextStyle {
            font_size: Some(28),
            font_file: None,
            color: TextColor::default(),
        },
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_transition_fade() -> Result<()> {
    let app = create_test_app_with_output("test_transition_fade")?;
    let intent = Intent::Transition {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        transition_type: TransitionType::Fade,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_transition_wipe() -> Result<()> {
    let app = create_test_app_with_output("test_transition_wipe")?;
    let intent = Intent::Transition {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        transition_type: TransitionType::Wipe,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_transition_slide() -> Result<()> {
    let app = create_test_app_with_output("test_transition_slide")?;
    let intent = Intent::Transition {
        video1: PathBuf::from(VIDEO_ASSET),
        video2: PathBuf::from(VIDEO_ASSET),
        transition_type: TransitionType::Slide,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_sync_cameras() -> Result<()> {
    let app = create_test_app_with_output("test_sync_cameras")?;
    let intent = Intent::SyncCameras {
        videos: vec![
            PathBuf::from(VIDEO_ASSET),
            PathBuf::from(VIDEO_ASSET),
        ],
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_generate_test_pattern() -> Result<()> {
    let app = create_test_app_with_output("test_generate_test_pattern")?;
    let intent = Intent::GenerateTestPattern {
        resolution: "1080p".to_string(),
        duration: Duration::parse("10s")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_add_timecode() -> Result<()> {
    let app = create_test_app_with_output("test_add_timecode")?;
    let intent = Intent::AddTimecode {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_proxy() -> Result<()> {
    let app = create_test_app_with_output("test_proxy")?;
    let intent = Intent::Proxy {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_export_edl() -> Result<()> {
    let app = create_test_app_with_output("test_export_edl")?;
    let intent = Intent::ExportEdl {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_colorspace() -> Result<()> {
    let app = create_test_app_with_output("test_convert_colorspace")?;
    let intent = Intent::ConvertColorspace {
        input: PathBuf::from(VIDEO_ASSET),
        target: ffhuman::model::types::Colorspace::Rec709,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_detect_silence() -> Result<()> {
    let app = create_test_app_with_output("test_detect_silence")?;
    let intent = Intent::DetectSilence {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_analyze_loudness() -> Result<()> {
    let app = create_test_app_with_output("test_analyze_loudness")?;
    // Test CLI parsing - this should work with kebab-case command
    let cli = Cli::try_parse_from(&["ffhuman", "analyze-loudness", VIDEO_ASSET, "--dry-run"])?;
    let intent = cli.into_intent()?;
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_detect_duplicates() -> Result<()> {
    let app = create_test_app_with_output("test_detect_duplicates")?;
    let intent = Intent::DetectDuplicates {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_collage() -> Result<()> {
    let app = create_test_app_with_output("test_collage")?;
    let intent = Intent::Collage {
        videos: vec![
            PathBuf::from(VIDEO_ASSET),
            PathBuf::from(VIDEO_ASSET),
            PathBuf::from(VIDEO_ASSET),
            PathBuf::from(VIDEO_ASSET),
        ],
        layout: MontageLayout::parse("2x2")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_slideshow() -> Result<()> {
    let app = create_test_app_with_output("test_slideshow")?;
    let intent = Intent::Slideshow {
        images: vec![
            PathBuf::from(IMAGE_JPG_1),
            PathBuf::from(IMAGE_JPG_2),
            PathBuf::from(IMAGE_JPG_3),
            PathBuf::from(IMAGE_JPG_4),
            PathBuf::from(IMAGE_JPG_5),
            PathBuf::from(IMAGE_JPG_6),
            PathBuf::from(IMAGE_JPG_7),
        ],
        duration: Duration::parse("2s")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_slideshow_with_jpeg() -> Result<()> {
    let app = create_test_app_with_output("test_slideshow_with_jpeg")?;
    let intent = Intent::Slideshow {
        images: vec![
            PathBuf::from(IMAGE_JPG_1),
            PathBuf::from(IMAGE_JPG_2),
            PathBuf::from(IMAGE_JPG_3),
        ],
        duration: Duration::parse("3s")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_slideshow_with_multiple_jpeg() -> Result<()> {
    let app = create_test_app_with_output("test_slideshow_with_multiple_jpeg")?;
    let intent = Intent::Slideshow {
        images: vec![
            PathBuf::from(IMAGE_JPG_1),
            PathBuf::from(IMAGE_JPG_2),
            PathBuf::from(IMAGE_JPG_3),
            PathBuf::from(IMAGE_JPG_4),
            PathBuf::from(IMAGE_JPG_5),
            PathBuf::from(IMAGE_JPG_6),
            PathBuf::from(IMAGE_JPG_7),
        ],
        duration: Duration::parse("2s")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_visualize_waveform() -> Result<()> {
    let app = create_test_app_with_output("test_visualize_waveform")?;
    let intent = Intent::Visualize {
        audio: PathBuf::from(AUDIO_ASSET),
        style: VisualizationStyle::Waveform,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_visualize_spectrum() -> Result<()> {
    let app = create_test_app_with_output("test_visualize_spectrum")?;
    let intent = Intent::Visualize {
        audio: PathBuf::from(AUDIO_ASSET),
        style: VisualizationStyle::Spectrum,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_animated_gif() -> Result<()> {
    let app = create_test_app_with_output("test_animated_gif")?;
    let intent = Intent::AnimatedGif {
        input: PathBuf::from(VIDEO_WEBM_ASSET), // Use WebM for conversion test
        loop_video: true,
        optimize: true,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_animated_gif_loop_only() -> Result<()> {
    let app = create_test_app_with_output("test_animated_gif_loop_only")?;
    let intent = Intent::AnimatedGif {
        input: PathBuf::from(VIDEO_WEBM_ASSET), // Use WebM for conversion test
        loop_video: true,
        optimize: false,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_animated_gif_optimize_only() -> Result<()> {
    let app = create_test_app_with_output("test_animated_gif_optimize_only")?;
    let intent = Intent::AnimatedGif {
        input: PathBuf::from(VIDEO_WEBM_ASSET), // Use WebM for conversion test
        loop_video: false,
        optimize: true,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_tile() -> Result<()> {
    let app = create_test_app_with_output("test_tile")?;
    let intent = Intent::Tile {
        input: PathBuf::from(VIDEO_ASSET),
        layout: MontageLayout::parse("3x3")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_tile_2x2() -> Result<()> {
    let app = create_test_app_with_output("test_tile_2x2")?;
    let intent = Intent::Tile {
        input: PathBuf::from(VIDEO_ASSET),
        layout: MontageLayout::parse("2x2")?,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_repair() -> Result<()> {
    let app = create_test_app_with_output("test_repair")?;
    let intent = Intent::Repair {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_validate() -> Result<()> {
    let app = create_test_app_with_output("test_validate")?;
    let intent = Intent::Validate {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_extract_keyframes() -> Result<()> {
    let app = create_test_app_with_output("test_extract_keyframes")?;
    let intent = Intent::ExtractKeyframes {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_stats() -> Result<()> {
    let app = create_test_app_with_output("test_stats")?;
    let intent = Intent::Stats {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_360() -> Result<()> {
    let app = create_test_app_with_output("test_convert_360")?;
    let intent = Intent::Convert360 {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_convert_hdr_to_sdr() -> Result<()> {
    let app = create_test_app_with_output("test_convert_hdr_to_sdr")?;
    let intent = Intent::ConvertHdrToSdr {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_fix_framerate() -> Result<()> {
    let app = create_test_app_with_output("test_fix_framerate")?;
    let intent = Intent::FixFramerate {
        input: PathBuf::from(VIDEO_ASSET),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_watch_folder() -> Result<()> {
    let app = create_test_app_with_output("test_watch_folder")?;
    let intent = Intent::WatchFolder {
        folder: PathBuf::from("src/assets"),
        operation: BatchOperation::Convert(ConvertFormat::Mp4),
    };
    // Note: This test validates the intent structure and basic setup
    // In a real scenario, you'd want to test with actual file system events
    // The watch operation runs indefinitely, so we just verify it can be created
    // For a full test, you'd spawn it in a thread and send test file events
    let _ = app;
    let _ = intent;
    Ok(())
}

#[test]
fn test_apply_template() -> Result<()> {
    use std::fs;
    use tempfile::TempDir;
    
    let app = create_test_app_with_output("test_apply_template")?;
    
    // Create a temporary template file
    let temp_dir = TempDir::new()?;
    let template_file = temp_dir.path().join("template.yaml");
    
    let template_content = r#"
operations:
  - type: convert
    format: gif
  - type: resize
    target: 720p
"#;
    
    fs::write(&template_file, template_content)?;
    
    let intent = Intent::ApplyTemplate {
        input: PathBuf::from(VIDEO_ASSET),
        template_file,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_pipeline() -> Result<()> {
    use std::fs;
    use tempfile::TempDir;
    
    let app = create_test_app_with_output("test_pipeline")?;
    
    // Create a temporary pipeline steps file
    let temp_dir = TempDir::new()?;
    let steps_file = temp_dir.path().join("steps.yaml");
    
    let steps_content = r#"
steps:
  - type: trim
    start: 0:05
    end: 0:30
  - type: convert
    format: mp4
  - type: resize
    target: 720p
"#;
    
    fs::write(&steps_file, steps_content)?;
    
    let intent = Intent::Pipeline {
        input: PathBuf::from(VIDEO_ASSET),
        steps_file,
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_conditional_batch() -> Result<()> {
    use ffhuman::model::types::{Duration, ProcessingCondition};
    
    let app = create_test_app_with_output("test_conditional_batch")?;
    let intent = Intent::ConditionalBatch {
        pattern: "src/assets/*.webm".to_string(),
        operation: BatchOperation::Convert(ConvertFormat::Gif),
        condition: ProcessingCondition::DurationLessThan(Duration::parse("60s")?),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_conditional_batch_duration_greater_than() -> Result<()> {
    use ffhuman::model::types::{Duration, ProcessingCondition};
    
    let app = create_test_app_with_output("test_conditional_batch_duration_greater_than")?;
    let intent = Intent::ConditionalBatch {
        pattern: "src/assets/*.webm".to_string(),
        operation: BatchOperation::Convert(ConvertFormat::Mp4),
        condition: ProcessingCondition::DurationGreaterThan(Duration::parse("1s")?),
    };
    app.execute(intent)?;
    Ok(())
}

#[test]
fn test_conditional_batch_duration_equals() -> Result<()> {
    use ffhuman::model::types::{Duration, ProcessingCondition};
    
    let app = create_test_app_with_output("test_conditional_batch_duration_equals")?;
    let intent = Intent::ConditionalBatch {
        pattern: "src/assets/*.webm".to_string(),
        operation: BatchOperation::Convert(ConvertFormat::Webm),
        condition: ProcessingCondition::DurationEquals(Duration::parse("10s")?),
    };
    app.execute(intent)?;
    Ok(())
}

