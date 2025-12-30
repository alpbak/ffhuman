use crate::commands::{*, analysis};
use crate::config::AppConfig;
use crate::ffmpeg::runner::{CliRunner, Runner};
use crate::model::Intent;
use anyhow::Result;

pub struct App {
    runner: Box<dyn Runner>,
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        let runner = Box::new(CliRunner::new(
            config.dry_run,
            config.overwrite,
            config.explain,
        ));
        Self { runner, config }
    }

    pub fn execute(&self, intent: Intent) -> Result<()> {
        match intent {
            Intent::Convert { input, format, quality, codec } => {
                convert::handle_convert(&self.config, self.runner.as_ref(), input, format, quality, codec)
            }
            Intent::Compress { input, target, two_pass } => {
                compress::handle_compress(&self.config, self.runner.as_ref(), input, target, two_pass)
            }
            Intent::Trim { input, start, end } => {
                trim::handle_trim(&self.config, self.runner.as_ref(), input, start, end)
            }
            Intent::ExtractAudio { input, format } => {
                audio::handle_extract_audio(&self.config, self.runner.as_ref(), input, format)
            }
            Intent::AdjustVolume { input, adjustment } => {
                audio::handle_adjust_volume(&self.config, self.runner.as_ref(), input, adjustment)
            }
            Intent::SyncAudio { input, direction, offset } => {
                audio::handle_sync_audio(&self.config, self.runner.as_ref(), input, direction, offset)
            }
            Intent::MixAudio { audio1, audio2 } => {
                audio::handle_mix_audio(&self.config, self.runner.as_ref(), audio1, audio2)
            }
            Intent::ExtractAudioRange { input, start, end, format } => {
                audio::handle_extract_audio_range(&self.config, self.runner.as_ref(), input, start, end, format)
            }
            Intent::Resize { input, target } => {
                video::handle_resize(&self.config, self.runner.as_ref(), input, target)
            }
            Intent::SpeedUp { input, factor } => {
                video::handle_speed_up(&self.config, self.runner.as_ref(), input, factor)
            }
            Intent::SlowDown { input, factor } => {
                video::handle_slow_down(&self.config, self.runner.as_ref(), input, factor)
            }
            Intent::Reverse { input } => {
                video::handle_reverse(&self.config, self.runner.as_ref(), input)
            }
            Intent::Mute { input } => {
                audio::handle_mute(&self.config, self.runner.as_ref(), input)
            }
            Intent::Rotate { input, degrees } => {
                video::handle_rotate(&self.config, self.runner.as_ref(), input, degrees)
            }
            Intent::Flip { input, direction } => {
                video::handle_flip(&self.config, self.runner.as_ref(), input, direction)
            }
            Intent::Thumbnail { input, time } => {
                video::handle_thumbnail(&self.config, self.runner.as_ref(), input, time)
            }
            Intent::Crop { input, width, height } => {
                video::handle_crop(&self.config, self.runner.as_ref(), input, width, height)
            }
            Intent::SetFps { input, fps } => {
                video::handle_set_fps(&self.config, self.runner.as_ref(), input, fps)
            }
            Intent::Loop { input, times } => {
                combine::handle_loop(&self.config, self.runner.as_ref(), input, times)
            }
            Intent::Merge { a, b } => {
                combine::handle_merge(&self.config, self.runner.as_ref(), a, b)
            }
            Intent::AddAudio { audio, video } => {
                audio::handle_add_audio(&self.config, self.runner.as_ref(), audio, video)
            }
            Intent::Grayscale { input } => {
                video::handle_grayscale(&self.config, self.runner.as_ref(), input)
            }
            Intent::Stabilize { input } => {
                effects::handle_stabilize(&self.config, self.runner.as_ref(), input)
            }
            Intent::Denoise { input } => {
                effects::handle_denoise(&self.config, self.runner.as_ref(), input)
            }
            Intent::Watermark { input, logo, position, opacity, size } => {
                video::handle_watermark(&self.config, self.runner.as_ref(), input, logo, position, opacity, size)
            }
            Intent::AddText { input, text, position, style, timestamp } => {
                video::handle_add_text(&self.config, self.runner.as_ref(), input, &text, position, style, timestamp)
            }
            Intent::Filter { input, adjustments, preset } => {
                video::handle_filter(&self.config, self.runner.as_ref(), input, adjustments, preset)
            }
            Intent::Blur { input, blur_type } => {
                video::handle_blur(&self.config, self.runner.as_ref(), input, blur_type)
            }
            Intent::Normalize { input } => {
                audio::handle_normalize(&self.config, self.runner.as_ref(), input)
            }
            Intent::Fade { input, fade_in, fade_out } => {
                audio::handle_fade(&self.config, self.runner.as_ref(), input, fade_in, fade_out)
            }
            Intent::Split { input, mode } => {
                video::handle_split(&self.config, self.runner.as_ref(), input, mode)
            }
            Intent::ExtractFrames { input, interval } => {
                video::handle_extract_frames(&self.config, self.runner.as_ref(), input, interval)
            }
            Intent::BurnSubtitle { input, subtitle } => {
                video::handle_burn_subtitle(&self.config, self.runner.as_ref(), input, subtitle)
            }
            Intent::Batch { pattern, operation } => {
                batch::handle_batch(&self.config, self.runner.as_ref(), &pattern, operation)
            }
            Intent::Compare { video1, video2, show_psnr } => {
                combine::handle_compare(&self.config, self.runner.as_ref(), video1, video2, show_psnr)
            }
            Intent::SetMetadata { input, field, value } => {
                metadata::handle_set_metadata(&self.config, self.runner.as_ref(), input, field, &value)
            }
            Intent::Montage { videos, layout } => {
                combine::handle_montage(&self.config, self.runner.as_ref(), &videos, layout)
            }
            Intent::Crossfade { video1, video2, duration } => {
                combine::handle_crossfade(&self.config, self.runner.as_ref(), video1, video2, duration)
            }
            Intent::Timelapse { input, speed } => {
                video::handle_timelapse(&self.config, self.runner.as_ref(), input, speed)
            }
            Intent::Pip { overlay_video, base_video, position } => {
                video::handle_pip(&self.config, self.runner.as_ref(), overlay_video, base_video, position)
            }
            Intent::RemoveBackground { input, color } => {
                video::handle_remove_background(&self.config, self.runner.as_ref(), input, color)
            }
            Intent::Overlay { overlay_video, base_video, position, opacity } => {
                video::handle_overlay(&self.config, self.runner.as_ref(), overlay_video, base_video, position, opacity)
            }
            Intent::Concat { videos } => {
                combine::handle_concat(&self.config, self.runner.as_ref(), &videos)
            }
            Intent::DetectScenes { input } => {
                video::handle_detect_scenes(&self.config, self.runner.as_ref(), input)
            }
            Intent::DetectBlack { input } => {
                video::handle_detect_black(&self.config, self.runner.as_ref(), input)
            }
            Intent::FixRotation { input } => {
                video::handle_fix_rotation(&self.config, self.runner.as_ref(), input)
            }
            Intent::AnalyzeQuality { input } => {
                video::handle_analyze_quality(&self.config, self.runner.as_ref(), input)
            }
            Intent::Preview { input } => {
                video::handle_preview(&self.config, self.runner.as_ref(), input)
            }
            Intent::SuggestFormat { input } => {
                video::handle_suggest_format(&self.config, self.runner.as_ref(), input)
            }
            Intent::Workflow { config_file } => {
                workflow::handle_workflow(&self.config, self.runner.as_ref(), config_file)
            }
            Intent::MotionBlur { input, radius } => {
                effects::handle_motion_blur(&self.config, self.runner.as_ref(), input, radius)
            }
            Intent::Vignette { input, intensity, size } => {
                effects::handle_vignette(&self.config, self.runner.as_ref(), input, intensity, size)
            }
            Intent::LensCorrect { input } => {
                effects::handle_lens_correct(&self.config, self.runner.as_ref(), input)
            }
            Intent::Interpolate { input, fps } => {
                effects::handle_interpolate(&self.config, self.runner.as_ref(), input, fps)
            }
            Intent::ExtractMetadata { input, format } => {
                metadata::handle_extract_metadata(&self.config, self.runner.as_ref(), input, format)
            }
            Intent::Info { input } => {
                metadata::handle_info(&self.config, self.runner.as_ref(), input)
            }
            Intent::ThumbnailGrid { input, layout } => {
                video::handle_thumbnail_grid(&self.config, self.runner.as_ref(), input, layout)
            }
            Intent::SocialMediaConvert { input, platform } => {
                video::handle_social_media_convert(&self.config, self.runner.as_ref(), input, platform)
            }
            Intent::SocialCrop { input, shape } => {
                video::handle_social_crop(&self.config, self.runner.as_ref(), input, shape)
            }
            Intent::VerticalConvert { input } => {
                video::handle_vertical_convert(&self.config, self.runner.as_ref(), input)
            }
            Intent::StoryFormat { input } => {
                video::handle_story_format(&self.config, self.runner.as_ref(), input)
            }
            Intent::NoiseReduction { input } => {
                audio::handle_noise_reduction(&self.config, self.runner.as_ref(), input)
            }
            Intent::EchoRemoval { input } => {
                audio::handle_echo_removal(&self.config, self.runner.as_ref(), input)
            }
            Intent::AudioDucking { input } => {
                audio::handle_audio_ducking(&self.config, self.runner.as_ref(), input)
            }
            Intent::AudioEqualizer { input, bass, treble, mid } => {
                audio::handle_audio_equalizer(&self.config, self.runner.as_ref(), input, bass, treble, mid)
            }
            Intent::VoiceIsolation { input } => {
                audio::handle_voice_isolation(&self.config, self.runner.as_ref(), input)
            }
            Intent::AudioSpeedKeepPitch { input, factor } => {
                audio::handle_audio_speed_keep_pitch(&self.config, self.runner.as_ref(), input, factor)
            }
            Intent::Glitch { input, shift, noise } => {
                effects::handle_glitch(&self.config, self.runner.as_ref(), input, shift, noise)
            }
            Intent::VintageFilm { input, era } => {
                effects::handle_vintage_film(&self.config, self.runner.as_ref(), input, era)
            }
            Intent::SplitScreen { video1, video2, orientation } => {
                video::handle_split_screen(&self.config, self.runner.as_ref(), video1, video2, orientation)
            }
            Intent::Mirror { input, direction } => {
                video::handle_mirror(&self.config, self.runner.as_ref(), input, direction)
            }
            Intent::ColorGrade { input, preset } => {
                video::handle_color_grade(&self.config, self.runner.as_ref(), input, preset)
            }
            Intent::AnimatedText { input, text, position, animation, style } => {
                video::handle_animated_text(&self.config, self.runner.as_ref(), input, &text, position, animation, &style)
            }
            Intent::Transition { video1, video2, transition_type } => {
                video::handle_transition(&self.config, self.runner.as_ref(), video1, video2, transition_type)
            }
            Intent::SyncCameras { videos } => {
                combine::handle_sync_cameras(&self.config, self.runner.as_ref(), &videos)
            }
            Intent::GenerateTestPattern { resolution, duration } => {
                video::handle_generate_test_pattern(&self.config, self.runner.as_ref(), &resolution, duration)
            }
            Intent::AddTimecode { input } => {
                video::handle_add_timecode(&self.config, self.runner.as_ref(), input)
            }
            Intent::Proxy { input } => {
                video::handle_proxy(&self.config, self.runner.as_ref(), input)
            }
            Intent::ExportEdl { input } => {
                metadata::handle_export_edl(&self.config, self.runner.as_ref(), input)
            }
            Intent::ConvertColorspace { input, target } => {
                video::handle_convert_colorspace(&self.config, self.runner.as_ref(), input, target)
            }
            Intent::DetectSilence { input } => {
                analysis::handle_detect_silence(&self.config, self.runner.as_ref(), input)
            }
            Intent::AnalyzeLoudness { input } => {
                analysis::handle_analyze_loudness(&self.config, self.runner.as_ref(), input)
            }
            Intent::DetectDuplicates { input } => {
                analysis::handle_detect_duplicates(&self.config, self.runner.as_ref(), input)
            }
            Intent::Collage { videos, layout } => {
                combine::handle_collage(&self.config, self.runner.as_ref(), &videos, layout)
            }
            Intent::Slideshow { images, duration } => {
                combine::handle_slideshow(&self.config, self.runner.as_ref(), &images, duration)
            }
            Intent::Visualize { audio, style } => {
                video::handle_visualize(&self.config, self.runner.as_ref(), audio, style)
            }
            Intent::AnimatedGif { input, loop_video, optimize } => {
                convert::handle_animated_gif(&self.config, self.runner.as_ref(), input, loop_video, optimize)
            }
            Intent::Tile { input, layout } => {
                video::handle_tile(&self.config, self.runner.as_ref(), input, layout)
            }
            Intent::Doctor => crate::commands::doctor::handle_doctor(),
            Intent::Repair { input } => {
                video::handle_repair(&self.config, self.runner.as_ref(), input)
            }
            Intent::Validate { input } => {
                video::handle_validate(&self.config, self.runner.as_ref(), input)
            }
            Intent::ExtractKeyframes { input } => {
                video::handle_extract_keyframes(&self.config, self.runner.as_ref(), input)
            }
            Intent::Stats { input } => {
                video::handle_stats(&self.config, self.runner.as_ref(), input)
            }
            Intent::Convert360 { input } => {
                video::handle_convert_360(&self.config, self.runner.as_ref(), input)
            }
            Intent::ConvertHdrToSdr { input } => {
                video::handle_convert_hdr_to_sdr(&self.config, self.runner.as_ref(), input)
            }
            Intent::FixFramerate { input } => {
                video::handle_fix_framerate(&self.config, self.runner.as_ref(), input)
            }
            Intent::WatchFolder { folder, operation } => {
                watch::handle_watch_folder(&self.config, self.runner.as_ref(), folder, operation)
            }
            Intent::ApplyTemplate { input, template_file } => {
                template::handle_apply_template(&self.config, self.runner.as_ref(), input, template_file)
            }
            Intent::Pipeline { input, steps_file } => {
                pipeline::handle_pipeline(&self.config, self.runner.as_ref(), input, steps_file)
            }
            Intent::ConditionalBatch { pattern, operation, condition } => {
                batch::handle_conditional_batch(&self.config, self.runner.as_ref(), &pattern, operation, condition)
            }
        }
    }
}

