use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::Args;

use crate::output;

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

pub fn run(args: InitArgs) -> Result<()> {
    if !args.path.exists() {
        bail!("Path does not exist: {}", args.path.display());
    }

    if !args.path.is_dir() {
        bail!("Path is not a directory: {}", args.path.display());
    }

    output::separator();

    println!(
        "{}",
        output::title().apply_to("Initializing Navi workspace")
    );

    output::separator();

    println!();

    println!(
        "{} {}",
        output::label().apply_to("Path:"),
        output::path().apply_to(args.path.display())
    );

    println!(
        "{}",
        output::success().apply_to("[placeholder]")
    );

    Ok(())
}