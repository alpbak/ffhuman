# FFHuman Features

A comprehensive guide to all implemented features organized by category.

---

## Format Conversion

### Convert
Convert video or audio files to different formats (GIF, MP4, WebM, MP3, WAV, iPhone, Android, HLS, DASH, 360°).

### Convert 360°
Convert videos to 360° format.

### Convert HDR to SDR
Tone map HDR content to standard dynamic range.

### Convert Colorspace
Convert between different color spaces.

### Social Media Convert
Convert videos optimized for social media platforms (Instagram, TikTok, YouTube Shorts, Twitter).

### Vertical Convert
Convert horizontal videos to vertical (9:16) format.

### Story Format
Convert to story format (9:16, 15 seconds max, optimized encoding).

### Animated GIF
Convert video to optimized animated GIF with optional looping and optimization.

---

## Compression & Quality

### Compress
Compress video to a target file size, bitrate, or quality preset (low-quality, medium-quality, high-quality, ultra-quality). Supports two-pass encoding for accurate size and bitrate targeting.

### Analyze Quality
Show detailed quality metrics (bitrate, resolution, codec info).

### Preview
Generate quick preview (first 10 seconds, low quality).

### Suggest Format
Recommend best format based on content analysis.

---

## Video Editing

### Trim
Trim video to a specific time range.

### Resize
Resize video to specific dimensions or preset resolution (720p, 1080p, 4k).

### Crop
Crop video to specific dimensions (centered crop).

### Rotate
Rotate video by 90, 180, or 270 degrees.

### Flip
Flip video horizontally or vertically.

### Set FPS
Change video frame rate.

### Loop
Loop video multiple times.

### Merge
Merge two videos sequentially.

### Concat
Join multiple videos without re-encoding (faster than merge).

### Split
Split video every N seconds or into N equal parts.

### Fix Rotation
Automatically detect and fix video orientation.

### Fix Framerate
Convert variable frame rate (VFR) to constant frame rate (CFR).

### Repair
Attempt to fix corrupted video files.

---

## Speed & Time Manipulation

### Speed Up
Speed up video playback by a factor.

### Slow Down
Slow down video playback by a factor.

### Reverse
Reverse video playback (play backwards).

### Timelapse
Create time-lapse video by speeding up playback.

---

## Audio Processing

### Extract Audio
Extract audio from video file.

### Extract Audio Range
Extract audio from a specific time range.

### Adjust Volume
Adjust audio volume by percentage or decibels.

### Sync Audio
Fix audio/video sync issues by delaying or advancing audio.

### Mix Audio
Combine multiple audio files into one.

### Normalize
Normalize audio levels in video.

### Fade
Apply fade in/out effects to audio in video.

### Mute
Remove audio track from video.

### Noise Reduction
Remove background noise from audio.

### Echo Removal
Clean up audio with echo or reverb.

### Audio Ducking
Automatically lower background music when speech is detected.

### Audio Equalizer
Adjust frequency bands (bass, treble, mid).

### Voice Isolation
Extract or isolate voice from background audio.

### Audio Speed Keep Pitch
Time-stretch audio without changing pitch.

---

## Video Effects & Filters

### Grayscale
Convert video to grayscale.

### Stabilize
Stabilize shaky video using video stabilization algorithm.

### Denoise
Reduce video noise using denoising filter.

### Filter
Apply video filters: brightness/contrast/saturation adjustments or color grading presets (vintage, black-and-white, sepia).

### Blur
Blur faces or regions in video for privacy.

### Motion Blur
Add motion blur effect to video. Optionally control the blur intensity with the radius parameter.

### Vignette
Add vignette effect (darkened edges). Optionally control the intensity and size of the vignette.

### Lens Correct
Fix lens distortion in video.

### Interpolate
Generate intermediate frames for smooth slow-motion.

### Glitch
Apply digital glitch effects with RGB channel separation and digital artifacts. Optionally control the intensity.

### Vintage Film
Apply vintage film look with grain, scratches, and color grading. Optionally choose a specific era (70s, 80s, 90s, classic).

### Mirror
Mirror or flip video horizontally or vertically.

### Color Grade
Apply advanced color grading with presets (cinematic, etc.).

---

## Video Composition

### Watermark
Add a logo/image overlay to a video with configurable position, opacity, and size.

### Add Text
Add text overlay to video with configurable position, font, size, and color. Supports timestamp overlay.

### Animated Text
Add animated text overlays with various effects (fade-in, etc.).

### Picture-in-Picture (PIP)
Overlay one video on another in a specific position.

### Remove Background
Remove backgrounds using chroma key technology (green screen, etc.).

### Overlay
Overlay videos with adjustable opacity.

### Split Screen
Create side-by-side or top/bottom split screen videos.

### Montage
Create video montages with multiple videos in various layouts (2x2, 3x1, etc.).

### Collage
Create video collages with multiple videos in various layouts.

### Tile
Repeat video in a grid pattern.

### Crossfade
Create crossfade transition between videos.

### Transition
Add transitions between videos (fade, wipe, slide).

---

## Overlays & Text

### Thumbnail
Extract a single frame as a thumbnail image.

### Thumbnail Grid
Generate a grid of thumbnails from video.

### Extract Frames
Extract frames from video at specified intervals.

### Burn Subtitle
Burn subtitles (SRT, ASS) into video.

---

## Analysis & Detection

### Detect Scenes
Automatically detect scene changes in video.

### Detect Black
Find and optionally remove black frames.

### Detect Silence
Find silent segments in video/audio.

### Analyze Loudness
Measure LUFS (Loudness Units relative to Full Scale) for broadcast standards.

### Detect Duplicates
Find duplicate or repeated frames.

---

## Metadata & Information

### Extract Metadata
Export all metadata to JSON or XML format.

### Info
Display human-readable summary of video properties.

### Stats
Show detailed statistics (bitrate over time, frame types, etc.).

### Extract Keyframes
Extract I-frames only from video.

### Set Metadata
Edit video metadata (title, author, copyright, comment, description).

---

## Batch Processing & Automation

### Batch
Batch process files matching a pattern.

### Watch Folder
Auto-process files added to a folder.

### Workflow
Define multi-step operations in a configuration file.

### Apply Template
Apply saved processing templates.

### Pipeline
Define multi-step processing pipeline.

### Conditional Batch
Process files based on conditions.

---

## Professional Features

### Sync Cameras
Sync multiple camera angles by audio.

### Generate Test Pattern
Generate test patterns for calibration.

### Add Timecode
Burn timecode overlay onto video.

### Proxy
Generate low-resolution proxies for editing.

### Export EDL
Export edit decision list for professional editing software.

---

## Advanced Composition

### Slideshow
Create video from a sequence of images.

### Visualize
Generate video from audio (waveform, spectrum visualization).

---

## Utility & Maintenance

### Doctor
Check system and FFmpeg installation.

### Validate
Check if file is valid and complete.

### Compare
Compare two videos side-by-side with quality metrics.

---

## Feature Categories Summary

- **Format Conversion** (8 features): Convert, 360°, HDR to SDR, colorspace, social media, vertical, story format, animated GIF
- **Compression & Quality** (4 features): Compress, analyze quality, preview, suggest format
- **Video Editing** (13 features): Trim, resize, crop, rotate, flip, set FPS, loop, merge, concat, split, fix rotation, fix framerate, repair
- **Speed & Time Manipulation** (4 features): Speed up, slow down, reverse, timelapse
- **Audio Processing** (14 features): Extract audio, extract audio range, adjust volume, sync audio, mix audio, normalize, fade, mute, noise reduction, echo removal, audio ducking, equalizer, voice isolation, audio speed keep pitch
- **Video Effects & Filters** (13 features): Grayscale, stabilize, denoise, filter, blur, motion blur, vignette, lens correct, interpolate, glitch, vintage film, mirror, color grade
- **Video Composition** (12 features): Watermark, add text, animated text, PIP, remove background, overlay, split screen, montage, collage, tile, crossfade, transition
- **Overlays & Text** (4 features): Thumbnail, thumbnail grid, extract frames, burn subtitle
- **Analysis & Detection** (5 features): Detect scenes, detect black, detect silence, analyze loudness, detect duplicates
- **Metadata & Information** (5 features): Extract metadata, info, stats, extract keyframes, set metadata
- **Batch Processing & Automation** (6 features): Batch, watch folder, workflow, apply template, pipeline, conditional batch
- **Professional Features** (5 features): Sync cameras, generate test pattern, add timecode, proxy, export EDL
- **Advanced Composition** (2 features): Slideshow, visualize
- **Utility & Maintenance** (3 features): Doctor, validate, compare

**Total: 98 implemented features**
