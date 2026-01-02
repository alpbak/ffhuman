# FFHuman Examples

Comprehensive usage examples for all FFHuman commands, organized by category.

---

## Format Conversion

### Convert Video to Different Formats

```bash
# Convert to GIF
ffhuman convert video.mp4 to gif

# Convert to MP4
ffhuman convert video.mp4 to mp4

# Convert to WebM with quality preset
ffhuman convert video.mp4 to webm --quality high

# Convert to WebM with specific codec
ffhuman convert video.mp4 to webm --quality high --codec vp9

# Extract audio to MP3
ffhuman convert video.mp4 to mp3

# Extract audio to WAV
ffhuman convert video.mp4 to wav

# Convert for iPhone
ffhuman convert video.mp4 to iphone

# Convert for Android
ffhuman convert video.mp4 to android

# Convert to HLS streaming format
ffhuman convert video.mp4 to hls

# Convert to DASH streaming format
ffhuman convert video.mp4 to dash

# Convert to 360° format
ffhuman convert video.mp4 to 360
```

### Social Media Conversion

```bash
# Convert for Instagram
ffhuman convert video.mp4 to instagram

# Convert for TikTok
ffhuman convert video.mp4 to tiktok

# Convert for YouTube Shorts
ffhuman convert video.mp4 to youtube-shorts

# Convert for Twitter
ffhuman convert video.mp4 to twitter
```

### Special Format Conversions

```bash
# Convert to vertical/portrait format
ffhuman convert to vertical video.mp4
ffhuman convert to portrait video.mp4

# Convert to story format
ffhuman convert video.mp4 to story

# Convert to animated GIF
ffhuman convert video.mp4 to animated-gif --loop --optimize
ffhuman convert video.mp4 to animated-gif --loop
ffhuman convert video.mp4 to animated-gif --optimize

# Convert HDR to SDR
ffhuman convert-hdr video.mp4 to sdr

# Convert colorspace
ffhuman convert-colorspace video.mp4 to rec709
ffhuman convert-colorspace video.mp4 to rec2020
```

---

## Compression & Quality

### Compress to Target Size

```bash
# Compress to 10MB
ffhuman compress video.mp4 to 10mb

# Compress to 800KB
ffhuman compress video.mp4 to 800k

# Compress to 1.5GB
ffhuman compress video.mp4 to 1.5gb

# Compress with two-pass encoding (more accurate)
ffhuman compress video.mp4 to 10mb --two-pass
```

### Compress to Target Bitrate

```bash
# Compress to 2000 kbps
ffhuman compress video.mp4 to 2000kbps

# Compress to 2 Mbps
ffhuman compress video.mp4 to 2mbps

# Compress to 500 kbps
ffhuman compress video.mp4 to 500k

# Compress with two-pass encoding (more accurate)
ffhuman compress video.mp4 to 2000kbps --two-pass
```

### Compress with Quality Presets

```bash
# High quality compression
ffhuman compress video.mp4 to high-quality

# Medium quality
ffhuman compress video.mp4 to medium-quality

# Low quality (smaller file)
ffhuman compress video.mp4 to low-quality

# Ultra quality (largest file, best quality)
ffhuman compress video.mp4 to ultra-quality
```

### Quality Analysis

```bash
# Analyze video quality
ffhuman analyze-quality video.mp4

# Generate preview
ffhuman preview video.mp4

# Get format suggestions
ffhuman suggest-format video.mp4
```

---

## Video Editing

### Trim

```bash
# Trim from 30 seconds to 60 seconds
ffhuman trim video.mp4 from 30 to 60

# Trim from 0:30 to 1:00
ffhuman trim video.mp4 from 0:30 to 1:00

# Trim from 1:05:30 to 2:10:45
ffhuman trim video.mp4 from 1:05:30 to 2:10:45
```

### Resize

```bash
# Resize to specific dimensions
ffhuman resize video.mp4 to 1280x720

# Resize to preset resolution
ffhuman resize video.mp4 to 720p
ffhuman resize video.mp4 to 1080p
ffhuman resize video.mp4 to 4k
```

### Crop

```bash
# Crop to 640x480 (centered)
ffhuman crop video.mp4 to 640x480

# Crop to 1920x1080
ffhuman crop video.mp4 to 1920x1080

# Social media crop
ffhuman social-crop video.mp4 square
ffhuman social-crop video.mp4 circle
```

### Rotate

```bash
# Rotate 90 degrees clockwise
ffhuman rotate video.mp4 by 90

# Rotate 180 degrees
ffhuman rotate video.mp4 by 180

# Rotate 270 degrees
ffhuman rotate video.mp4 by 270
```

### Flip

```bash
# Flip horizontally
ffhuman flip video.mp4 horizontal

# Flip vertically
ffhuman flip video.mp4 vertical
```

### Frame Rate

```bash
# Set to 30 FPS
ffhuman fps video.mp4 to 30

# Set to 60 FPS
ffhuman fps video.mp4 to 60

# Fix variable frame rate
ffhuman fix-framerate video.mp4
```

### Other Editing Operations

```bash
# Loop video 3 times
ffhuman loop video.mp4 3 times

# Loop video 5 times
ffhuman loop video.mp4 5 times

# Fix rotation automatically
ffhuman fix-rotation video.mp4

# Repair corrupted video
ffhuman repair video.mp4
```

---

## Speed & Time Manipulation

### Speed Up

```bash
# Speed up 2x
ffhuman speed-up video.mp4 by 2x

# Speed up 1.5x
ffhuman speed-up video.mp4 by 1.5x
```

### Slow Down

```bash
# Slow down 2x
ffhuman slow-down video.mp4 by 2x

# Slow down to 0.5x (half speed)
ffhuman slow-down video.mp4 by 0.5x
```

### Reverse

```bash
# Reverse video playback
ffhuman reverse video.mp4
```

### Timelapse

```bash
# Create 10x time-lapse
ffhuman timelapse video.mp4 speed 10x

# Create 100x time-lapse
ffhuman timelapse video.mp4 speed 100x
```

---

## Audio Operations

### Extract Audio

```bash
# Extract audio from video
ffhuman extract-audio video.mp4

# Extract audio from time range
ffhuman extract-audio-range video.mp4 from 0:30 to 2:00
```

### Volume Adjustment

```bash
# Adjust volume to 50%
ffhuman adjust-volume video.mp4 to 50%

# Increase volume by 10dB
ffhuman adjust-volume video.mp4 by +10db

# Decrease volume by 5dB
ffhuman adjust-volume video.mp4 by -5db
```

### Audio Sync

```bash
# Delay audio by 0.5 seconds
ffhuman sync-audio video.mp4 delay 0.5s

# Advance audio by 0.3 seconds
ffhuman sync-audio video.mp4 advance 0.3s
```

### Audio Mixing

```bash
# Mix two audio files
ffhuman mix-audio audio.mp3 and audio2.mp3
```

### Audio Effects

```bash
# Normalize audio levels
ffhuman normalize video.mp4

# Mute audio
ffhuman mute video.mp4

# Fade in and out
ffhuman fade video.mp4 --fade-in 2s --fade-out 2s

# Fade in only
ffhuman fade video.mp4 --fade-in 1.5s

# Fade out only
ffhuman fade video.mp4 --fade-out 3s
```

### Advanced Audio Processing

```bash
# Reduce noise
ffhuman reduce-noise video.mp4

# Remove echo/reverb
ffhuman remove-echo video.mp4

# Audio ducking (lower music when voice detected)
ffhuman duck-audio video.mp4 when voice detected

# Equalize audio
ffhuman equalize-audio video.mp4 --bass +5 --treble -2
ffhuman equalize-audio video.mp4 --bass +10 --mid -3

# Isolate voice
ffhuman isolate-voice video.mp4

# Speed audio without pitch change
ffhuman speed-audio video.mp4 by 1.5x --keep-pitch
ffhuman speed-audio video.mp4 by 2x --keep-pitch
```

---

## Video Effects & Filters

### Basic Effects

```bash
# Convert to grayscale
ffhuman grayscale video.mp4

# Stabilize shaky video
ffhuman stabilize video.mp4

# Denoise video
ffhuman denoise video.mp4
```

### Color Filters

```bash
# Adjust brightness and contrast
ffhuman filter video.mp4 --brightness 10 --contrast 5

# Adjust saturation
ffhuman filter video.mp4 --saturation 20

# Apply color preset
ffhuman filter video.mp4 --preset vintage
ffhuman filter video.mp4 --preset black-and-white
ffhuman filter video.mp4 --preset sepia
```

### Advanced Effects

```bash
# Motion blur
ffhuman motion-blur video.mp4
ffhuman motion-blur video.mp4 --radius 5

# Vignette
ffhuman vignette video.mp4
ffhuman vignette video.mp4 --intensity 0.8 --size 0.5
ffhuman vignette video.mp4 --intensity 0.3 --size 0.9

# Lens correction
ffhuman lens-correct video.mp4

# Frame interpolation
ffhuman interpolate video.mp4 to 60fps
ffhuman interpolate video.mp4 to 120fps

# Glitch effect
ffhuman glitch video.mp4
ffhuman glitch video.mp4 --shift 8 --noise 50
ffhuman glitch video.mp4 --shift 12 --noise 70

# Vintage film effect
ffhuman vintage-film video.mp4
ffhuman vintage-film video.mp4 --era 70s
ffhuman vintage-film video.mp4 --era 80s
ffhuman vintage-film video.mp4 --era 90s

# Mirror effect
ffhuman mirror video.mp4 horizontal
ffhuman mirror video.mp4 vertical

# Color grading
ffhuman color-grade video.mp4 --preset cinematic
ffhuman color-grade video.mp4 --preset warm
```

### Blur

```bash
# Blur a specific region (x, y, width, height)
ffhuman blur video.mp4 region 100,100,200,200
```

---

## Video Composition

### Watermark

```bash
# Add logo at top-right
ffhuman watermark video.mp4 logo.png at top-right

# Add logo with opacity
ffhuman watermark video.mp4 logo.png at top-right --opacity 0.5

# Add logo with custom size (percentage)
ffhuman watermark video.mp4 logo.png at bottom-left --size 20%

# Add logo with custom size (pixels)
ffhuman watermark video.mp4 logo.png at 100,50 --opacity 0.8 --size 200x100
```

### Text Overlay

```bash
# Add text at bottom-center
ffhuman add-text video.mp4 "My Video" at bottom-center

# Add text with custom styling
ffhuman add-text video.mp4 "Title" at top-left --font-size 48 --color red

# Add text with hex color
ffhuman add-text video.mp4 "Watermark" at top-right --color "#FFFFFF" --font-size 32

# Add timestamp overlay
ffhuman add-text video.mp4 "" at bottom-right --timestamp
```

### Animated Text

```bash
# Animated text with fade-in
ffhuman add-text video.mp4 "Title" at center --animate fade-in

# Animated text with slide-in
ffhuman add-text video.mp4 "Subtitle" at bottom-center --animate slide-in

# Animated text with typewriter effect
ffhuman add-text video.mp4 "Text" at center --animate typewriter
```

### Picture-in-Picture

```bash
# PIP at top-right
ffhuman pip video1.mp4 on video2.mp4 at top-right

# PIP at bottom-left
ffhuman pip video1.mp4 on video2.mp4 at bottom-left
```

### Background Removal

```bash
# Remove green screen
ffhuman remove-background video.mp4 color green

# Remove blue screen
ffhuman remove-background video.mp4 color blue

# Remove custom color (hex)
ffhuman remove-background video.mp4 color #00FF00
```

### Video Overlay

```bash
# Overlay with opacity at coordinates
ffhuman overlay video1.mp4 on video2.mp4 at 100,50 opacity 0.7

# Overlay at position with opacity
ffhuman overlay video1.mp4 on video2.mp4 at top-right opacity 0.5
```

### Split Screen

```bash
# Split screen (horizontal, side-by-side)
ffhuman split-screen video1.mp4 and video2.mp4

# Split screen with orientation
ffhuman split-screen video1.mp4 and video2.mp4 --orientation horizontal
ffhuman split-screen video1.mp4 and video2.mp4 --orientation vertical
```

### Montage & Collage

```bash
# Create 2x2 montage
ffhuman montage layout 2x2 video1.mp4 video2.mp4 video3.mp4 video4.mp4

# Create 3x1 horizontal montage
ffhuman montage layout 3x1 video1.mp4 video2.mp4 video3.mp4

# Create collage
ffhuman collage layout 2x2 video1.mp4 video2.mp4 video3.mp4
ffhuman collage layout 2x1 video1.mp4 video2.mp4
```

### Tiling

```bash
# Tile video in 3x3 grid
ffhuman tile video.mp4 3x3

# Tile video in 2x2 grid
ffhuman tile video.mp4 2x2
```

### Transitions

```bash
# Crossfade transition
ffhuman crossfade video1.mp4 and video2.mp4 duration 2s
ffhuman crossfade video1.mp4 and video2.mp4 duration 1.5s

# Other transitions
ffhuman transition video1.mp4 to video2.mp4 --type fade
ffhuman transition video1.mp4 to video2.mp4 --type wipe
ffhuman transition video1.mp4 to video2.mp4 --type slide
```

---

## Advanced Operations

### Merge & Concat

```bash
# Merge two videos sequentially
ffhuman merge video1.mp4 and video2.mp4

# Concatenate multiple videos (faster, no re-encoding)
ffhuman concat video1.mp4 video2.mp4 video3.mp4
```

### Split

```bash
# Split every 30 seconds
ffhuman split video.mp4 every 30s

# Split into 3 equal parts
ffhuman split video.mp4 into 3 parts
```

### Extract Frames

```bash
# Extract frames every 1 second
ffhuman extract-frames video.mp4 every 1s

# Extract frames every 0.5 seconds
ffhuman extract-frames video.mp4 every 0.5s
```

### Thumbnails

```bash
# Extract single thumbnail
ffhuman thumbnail video.mp4 at 5
ffhuman thumbnail video.mp4 at 0:05
ffhuman thumbnail video.mp4 at 1:05:30

# Generate thumbnail grid
ffhuman thumbnails video.mp4 3x3
ffhuman thumbnails video.mp4 2x2
```

### Subtitles

```bash
# Burn SRT subtitles
ffhuman burn-subtitle video.mp4 subtitle.srt

# Burn ASS subtitles
ffhuman burn-subtitle video.mp4 subtitle.ass
```

---

## Analysis & Detection

### Scene Detection

```bash
# Detect scene changes
ffhuman detect-scenes video.mp4
```

### Black Frame Detection

```bash
# Detect black frames
ffhuman detect-black video.mp4
```

### Silence Detection

```bash
# Detect silent segments
ffhuman detect-silence video.mp4
```

### Quality Analysis

```bash
# Analyze loudness (LUFS)
ffhuman analyze-loudness video.mp4

# Detect duplicate frames
ffhuman detect-duplicates video.mp4
```

---

## Metadata & Information

### Extract Metadata

```bash
# Extract to JSON (default)
ffhuman extract-metadata video.mp4

# Extract to JSON explicitly
ffhuman extract-metadata video.mp4 --format json

# Extract to XML
ffhuman extract-metadata video.mp4 --format xml
```

### Video Information

```bash
# Display video info
ffhuman info video.mp4

# Show detailed statistics
ffhuman stats video.mp4

# Extract keyframes only
ffhuman extract-keyframes video.mp4
```

### Set Metadata

```bash
# Set video title
ffhuman set-metadata video.mp4 title "My Video"

# Set author
ffhuman set-metadata video.mp4 author "John Doe"

# Set copyright
ffhuman set-metadata video.mp4 copyright "2024"

# Set comment
ffhuman set-metadata video.mp4 comment "This is a test video"

# Set description
ffhuman set-metadata video.mp4 description "Video description here"
```

---

## Batch Processing

### Batch Convert

```bash
# Convert all MP4 files to GIF
ffhuman batch convert *.mp4 to gif

# Convert with condition
ffhuman batch convert *.mp4 to gif --if duration < 30s
```

### Watch Folder

```bash
# Watch folder and auto-convert
ffhuman watch folder ./input --convert to mp4
```

### Workflow

```bash
# Run workflow from config file
ffhuman workflow process.yaml
```

### Templates

```bash
# Apply template
ffhuman apply-template video.mp4 template.yaml
```

### Pipeline

```bash
# Run processing pipeline
ffhuman pipeline video.mp4 steps.yaml
```

---

## Professional Features

### Multi-Camera Sync

```bash
# Sync multiple camera angles
ffhuman sync-cameras video1.mp4 video2.mp4 video3.mp4
```

### Test Patterns

```bash
# Generate test pattern
ffhuman generate-test-pattern 1080p 10s
ffhuman generate-test-pattern 720p 5s
```

### Timecode

```bash
# Add timecode overlay
ffhuman add-timecode video.mp4
```

### Proxy Generation

```bash
# Generate proxy for editing
ffhuman proxy video.mp4
```

### EDL Export

```bash
# Export edit decision list
ffhuman export-edl video.mp4
```

---

## Advanced Composition

### Slideshow

```bash
# Create slideshow from images
ffhuman slideshow duration 3s image1.jpg image2.jpg image3.jpg
ffhuman slideshow duration 2s img1.png img2.png
```

### Audio Visualization

```bash
# Generate waveform visualization
ffhuman visualize audio.mp3 --style waveform

# Generate spectrum visualization
ffhuman visualize audio.mp3 --style spectrum
```

---

## Utility & Maintenance

### Doctor

```bash
# Check system and FFmpeg installation
ffhuman doctor
```

### Validation

```bash
# Validate video file
ffhuman validate video.mp4
```

### Comparison

```bash
# Compare two videos
ffhuman compare video1.mp4 and video2.mp4

# Compare with quality metrics
ffhuman compare video1.mp4 and video2.mp4 --show-psnr
```

---

## Global Flags

All commands support these global flags:

### Dry Run

```bash
# See commands without executing
ffhuman convert video.mp4 to gif --dry-run
```

### Explain

```bash
# See detailed explanation
ffhuman compress video.mp4 to 10mb --explain
```

### Overwrite

```bash
# Overwrite existing files
ffhuman convert video.mp4 to gif --overwrite
ffhuman convert video.mp4 to gif -y
```

### Output Path

```bash
# Specify exact output file
ffhuman convert video.mp4 to gif --out /path/to/output.gif

# Specify output directory
ffhuman convert video.mp4 to gif --output-dir ./output
```

### Combined Flags

```bash
# Use multiple flags together
ffhuman convert video.mp4 to gif --dry-run --explain --overwrite
```

---

## Complete Workflow Example

Here's a complete workflow example:

```bash
# 1. Trim video
ffhuman trim video.mp4 from 0:30 to 5:00

# 2. Resize to 1080p
ffhuman resize video.mp4 to 1080p

# 3. Add watermark
ffhuman watermark video.mp4 logo.png at top-right --opacity 0.7 --size 15%

# 4. Normalize audio
ffhuman normalize video.mp4

# 5. Compress to 10MB
ffhuman compress video.mp4 to 10mb

# 6. Convert to WebM with high quality
ffhuman convert video.mp4 to webm --quality high --codec vp9
```

