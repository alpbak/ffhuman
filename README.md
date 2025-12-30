# ffhuman

> **FFmpeg for humans.**
> **Built by someone who was tired, not by a committee.**

---

## Why this exists

Because this:

```bash
ffhuman convert video.mp4 to gif
```

is much easier to remember then this:

```bash
ffmpeg -y -i input.mp4 -vf "fps=15,scale=480:-1:flags=lanczos,palettegen" palette.png && \
ffmpeg -y -i input.mp4 -i palette.png -lavfi "fps=15,scale=480:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer" output.gif
```

This is **not a normal sentence**.

It's a spell. A spell you Google every time. A spell you *never* remember. A spell that somehow breaks differently on every machine.

I kept Googling the same commands. I kept copy-pasting things I didn't want to understand again. I kept thinking: *"there has to be a better way to say this."*

So I built **`ffhuman`**.

This is **the tool that worked for me**. It may or may not work for you. That's fine.

---

## What `ffhuman` is

`ffhuman` is a **human-friendly CLI wrapper around FFmpeg**.

You say **what you want**. `ffhuman` figures out **how to say it to FFmpeg**.

For example, converting a video to a GIF:

```bash
ffhuman convert video.mp4 to gif
```

Same result as the FFmpeg command shown above.
Less ritual. No palette files to clean up. No remembering why you need two passes.

Same result. Less ritual. No palette files to clean up. No remembering why you need two passes.

---

## What it is NOT

Let's be clear:

* Not a replacement for FFmpeg
* Not a GUI
* Not trying to be "flexible" or "enterprise-ready"
* Not impressed by your custom codec pipeline

This is for **getting things done**.

---

## Philosophy (read this before opening an issue)

* Tools don't need more options. They need better defaults.
* This is **opinionated by design**.
* This solves *my* problems first.

I'm not trying to build:
* a universal solution
* a framework
* or a community-driven roadmap

If this tool fits your workflow — great. If not — that's also fine.

---

## Installation

> FFmpeg must already be installed.
> If you don't have FFmpeg, that's a different life choice.

### Build from source

```bash
git clone https://github.com/alpbak/ffhuman.git
cd ffhuman
cargo build --release
```

Put the binary in your `$PATH` and forget about it.

### Verify installation

```bash
ffhuman doctor
```

This checks your system and FFmpeg installation.

---

## Quick Start

```bash
# Convert video to GIF
ffhuman convert video.mp4 to gif

# Extract audio
ffhuman extract-audio video.mp4

# Resize to 1080p
ffhuman resize video.mp4 to 1080p

# Compress to 10MB
ffhuman compress video.mp4 to 10mb
```

That's it. No flags. No memorization. No pretending you enjoy this part.

---

## Global Flags

For when you need a bit more control:

- `--dry-run` - Print generated FFmpeg commands without executing
- `--explain` - Show detailed explanation of what will be done
- `--overwrite` / `-y` - Overwrite output files if they exist
- `--out <path>` - Specify exact output file path
- `--output-dir <dir>` - Specify output directory

---

## Documentation

This README is intentionally brief. For the full feature list and examples:

- **[FEATURES.md](FEATURES.md)** - Complete list of all available features
- **[EXAMPLES.md](EXAMPLES.md)** - Comprehensive usage examples for every command

---

## Why not just use FFmpeg directly?

You absolutely can.

You can also:
* write assembly
* manage memory manually
* calculate CRCs by hand
* brew your own coffee beans

But most days, you just want the video converted.

---

## Who is this for?

* Developers who are tired
* Indie makers
* Content creators who touch the terminal unwillingly
* Anyone who has typed `ffmpeg -i` more than once this week

---

## Who is this NOT for?

* FFmpeg purists
* People who enjoy tweaking bitrates for fun
* Anyone offended by defaults

---

## About feature requests

You're welcome to:
* fork it
* modify it
* adapt it to your needs

But please understand:

> This project is not a promise to implement every idea posted on the internet.

Suggestions are fine. Demands will be ignored.

---

## License

MIT.

Use it. Break it. Blame FFmpeg if something explodes.

---

## Final note

If you've ever:
* re-Googled the same FFmpeg command,
* copied a command without caring how it works,
* thought "I'll never remember this anyway"…

This tool was built by someone exactly like you.

No guarantees. No obligations. Just fewer spells.
