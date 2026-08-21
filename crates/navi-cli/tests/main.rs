use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn navi_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_navi"))
}

fn unique_temp_path(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after the Unix epoch")
        .as_nanos();

    env::temp_dir().join(format!(
        "navi-cli-{name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_path(path: &Path) {
    if path.is_dir() {
        fs::remove_dir_all(path).expect("temporary directory should be removable");
    } else if path.exists() {
        fs::remove_file(path).expect("temporary file should be removable");
    }
}

#[test]
fn init_defaults_to_the_current_directory() {
    let output = navi_command()
        .arg("init")
        .output()
        .expect("navi should start");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Initializing Navi workspace"));
    assert!(stdout.contains("Path:"));
    assert!(stdout.contains("[placeholder]"));
}

#[test]
fn init_succeeds_for_a_directory() {
    let path = unique_temp_path("directory");
    fs::create_dir(&path).expect("temporary directory should be created");

    let output = navi_command()
        .args([
            "init",
            path.to_str().expect("temporary path should be valid UTF-8"),
        ])
        .output()
        .expect("navi should start");

    remove_path(&path);

    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Initializing Navi workspace"));
}

#[test]
fn init_rejects_a_missing_path() {
    let path = unique_temp_path("missing");

    let output = navi_command()
        .args([
            "init",
            path.to_str().expect("temporary path should be valid UTF-8"),
        ])
        .output()
        .expect("navi should start");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Path does not exist"));
}

#[test]
fn init_rejects_a_file_path() {
    let path = unique_temp_path("file");
    File::create(&path).expect("temporary file should be created");

    let output = navi_command()
        .args([
            "init",
            path.to_str().expect("temporary path should be valid UTF-8"),
        ])
        .output()
        .expect("navi should start");

    remove_path(&path);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Path is not a directory"));
}

#[test]
fn cli_supports_help_and_version() {
    let help = navi_command()
        .arg("--help")
        .output()
        .expect("navi should start");
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("Initialize a Navi workspace"));

    let version = navi_command()
        .arg("--version")
        .output()
        .expect("navi should start");
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("navi 0.1.0"));
}
