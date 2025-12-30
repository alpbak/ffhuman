use crate::config::AppConfig;
use crate::ffmpeg::recipes;
use crate::ffmpeg::runner::Runner;
use crate::model::Time;
use crate::util::{default_out, system::ensure_ffmpeg_exists};
use anyhow::Result;
use std::path::Path;

pub fn handle_trim(
    config: &AppConfig,
    runner: &dyn Runner,
    input: impl AsRef<Path>,
    start: Time,
    end: Time,
) -> Result<()> {
    ensure_ffmpeg_exists()?;
    let input = input.as_ref();

    let out = default_out(config, input, "trim", "mp4")?;
    let steps = recipes::trim_steps(input, &out, &start, &end, config.overwrite);
    for step in steps {
        runner.run(&step)?;
    }

    eprintln!("Output: {}", out.display());
    Ok(())
}

