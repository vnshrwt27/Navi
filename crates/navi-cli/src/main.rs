mod commands;
mod output;

use clap::{Parser, Subcommand};

use commands::init;

#[derive(Debug, Parser)]
#[command(
    name = "navi",
    version,
    about = "Local code intelligence for multi-repository codebases",
    long_about = "Navi indexes repositories and provides local code intelligence \
                  through symbols, relationships, dependency graphs, and history.",
    styles = output::cli_styles()
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(
        name = "init",
        about = "Initialize a Navi workspace",
        long_about = "Initialize a new Navi workspace in the specified directory.",
        styles = output::cli_styles()
    )]
    Init(init::InitArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init(args) => init::run(args),
    }
}
