mod command;
mod commands;
mod output;

use clap::{Parser, Subcommand};

use command::Command;
use commands::init::InitArgs;

#[derive(Debug, Parser)]
#[command(
    name = "navi",
    version,
    about = "Local code intelligence for multi-repository codebases",
    long_about = "Navi indexes repositories and provides local code intelligence \
                  through symbols, relationships, dependency graphs, and history.",
    styles = output::cli_styles()
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        name = "init",
        about = "Initialize a Navi workspace",
        long_about = "Initialize a new Navi workspace in the specified directory.",
        styles = output::cli_styles()
    )]
    Init(InitArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init(args) => args.run(),
    };

    if let Err(err) = result {
        eprintln!("{} {}", output::error().apply_to("Error:"), err);

        std::process::exit(1);
    }
}
