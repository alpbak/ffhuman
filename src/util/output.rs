use crate::config::AppConfig;
use anyhow::{anyhow, Context, Result};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs;

/// Get the base stem (filename without extension) from a path
fn base_stem(path: &Path) -> Result<String> {
    path.file_stem()
        .and_then(OsStr::to_str)
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Invalid input filename"))
}

/// Generate default output path based on input, suffix, and extension
pub fn default_out(
    config: &AppConfig,
    input: &Path,
    suffix: &str,
    ext: &str,
) -> Result<PathBuf> {
    if let Some(out) = &config.out {
        // If full path is specified, ensure its parent directory exists
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent).context("Failed to create output directory")?;
        }
        return Ok(out.clone());
    }
    
    let stem = base_stem(input)?;
    let dir = if let Some(output_dir) = &config.output_dir {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir).context("Failed to create output directory")?;
        output_dir
    } else {
        input.parent().unwrap_or_else(|| Path::new("."))
    };
    
    Ok(dir.join(format!("{stem}_{suffix}.{ext}")))
}

