use anyhow::Result;
use ffhuman::{App, AppConfig, Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = AppConfig::new(
        cli.out.clone(),
        cli.output_dir.clone(),
        cli.explain,
        cli.dry_run,
        cli.overwrite,
    );
    let intent = cli.into_intent()?;
    let app = App::new(config);
    app.execute(intent)
}
