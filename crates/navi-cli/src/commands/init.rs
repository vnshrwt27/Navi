use std::{error::Error, path::PathBuf};

use clap::Args;

use crate::{command::Command, output};

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

impl Command for InitArgs {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        if !self.path.exists() {
            return Err(format!("Path does not exist: {}", self.path.display()).into());
        }

        if !self.path.is_dir() {
            return Err(format!("Path is not a directory: {}", self.path.display()).into());
        }

        let info = output::separator("Initializing Navi workspace");
        println!("{}", info);

        println!(
            "{} {}",
            output::label().apply_to("Path:"),
            output::path().apply_to(self.path.display())
        );

        println!("{}", output::success().apply_to("[placeholder]"));

        Ok(())
    }
}
