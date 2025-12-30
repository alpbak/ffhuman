use std::path::PathBuf;

/// Application configuration holding global flags
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub out: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub explain: bool,
    pub dry_run: bool,
    pub overwrite: bool,
}

impl AppConfig {
    pub fn new(
        out: Option<PathBuf>,
        output_dir: Option<PathBuf>,
        explain: bool,
        dry_run: bool,
        overwrite: bool,
    ) -> Self {
        Self {
            out,
            output_dir,
            explain,
            dry_run,
            overwrite,
        }
    }
}

