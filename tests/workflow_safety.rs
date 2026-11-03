use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn fixture(name: &str) -> PathBuf {
    let root =
        Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    root
}

fn repository(path: &Path) {
    std::fs::create_dir_all(path).unwrap();
    assert!(
        Command::new("git")
            .arg("init")
            .arg("--quiet")
            .arg(path)
            .status()
            .unwrap()
            .success()
    );
    std::fs::write(path.join("input.txt"), b"user input must stay unchanged\n").unwrap();
}

fn config(root: &Path, library: &Path) -> PathBuf {
    let mut config: serde_json::Value =
        serde_json::from_str(include_str!("../examples/pipe/vanllia.json")).unwrap();
    config["library"] = serde_json::json!(library);
    let path = root.join("config.json");
    std::fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    path
}

fn cli(config: &Path, state: &Path, action: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_vanllia-build"))
        .env("RUSTUP_TOOLCHAIN", "nightly-2026-09-04")
        .arg("--config")
        .arg(config)
        .arg("--state")
        .arg(state)
        .args(action)
        .output()
        .unwrap()
}

#[test]
fn state_inside_library_is_rejected_before_writing() {
    let root = fixture("state-inside-library");
    let library = root.join("library");
    repository(&library);
    let config = config(&root, &library);
    let state = library.join("state");
    let result = cli(&config, &state, &["override", "clear", "pipe"]);
    assert!(
        !result.status.success(),
        "state nested inside input was accepted"
    );
    assert!(
        !state.exists(),
        "rejection must not write to the input checkout"
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn state_inside_library_through_a_parent_symlink_is_rejected() {
    let root = fixture("state-inside-library-alias");
    let library = root.join("library");
    repository(&library);
    let alias = root.join("alias");
    std::os::unix::fs::symlink(&library, &alias).unwrap();
    let config = config(&root, &library);
    let result = cli(
        &config,
        &alias.join("nested/state"),
        &["override", "clear", "pipe"],
    );
    assert!(!result.status.success());
    assert!(!library.join("nested").exists());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn state_inside_proposed_override_is_rejected_before_writing() {
    let root = fixture("state-inside-override");
    let library = root.join("library");
    let local = root.join("local");
    repository(&library);
    repository(&local);
    let config = config(&root, &library);
    let state = local.join("state");
    let result = cli(
        &config,
        &state,
        &["override", "set", "pipe", local.to_str().unwrap()],
    );
    assert!(
        !result.status.success(),
        "state nested inside override was accepted"
    );
    assert!(
        !state.exists(),
        "setting an override must not write into that checkout"
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn inspecting_missing_state_does_not_create_it() {
    let root = fixture("inspect-missing-state");
    let library = root.join("library");
    repository(&library);
    let config = config(&root, &library);
    let state = root.join("missing-state");
    assert!(cli(&config, &state, &["inspect"]).status.success());
    assert!(!state.exists(), "inspect must be read only");
    std::fs::remove_dir_all(root).unwrap();
}
