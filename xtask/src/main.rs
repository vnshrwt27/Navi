use std::process::{Command, ExitCode};

fn run(command: &str, args: &[&str]) -> bool {
    println!("==> {} {}", command, args.join(" "));

    match Command::new(command).args(args).status() {
        Ok(status) => status.success(),
        Err(error) => {
            eprintln!("Failed to execute `{command}`: {error}");
            false
        }
    }
}

fn check() -> bool {
    run("cargo", &["fmt", "--all", "--", "--check"])
        && run("cargo", &["check", "--all-targets", "--all-features"])
        && run(
            "cargo",
            &[
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        )
        && run("cargo", &["test", "--all-features"])
}

fn coverage() -> bool {
    run(
        "cargo",
        &[
            "llvm-cov",
            "--workspace",
            "--all-features",
            "--exclude",
            "xtask",
            "--fail-under-lines",
            "80",
        ],
    )
}

fn main() -> ExitCode {
    let command = std::env::args().nth(1);

    let success = match command.as_deref() {
        Some("check") => check(),
        Some("coverage") => coverage(),
        Some("pr") => check() && coverage(),
        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo xtask check");
            eprintln!("  cargo xtask coverage");
            eprintln!("  cargo xtask pr");
            return ExitCode::FAILURE;
        }
    };

    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
