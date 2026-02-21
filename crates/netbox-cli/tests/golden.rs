//! Golden output tests for CLI commands.
//!
//! These tests run CLI commands against a live NetBox instance and compare
//! output against stored golden files. Run with:
//!
//! ```bash
//! NETBOX_URL=http://localhost:8000 NETBOX_TOKEN=... cargo test -p netbox-cli --test golden -- --ignored
//! ```
//!
//! To update golden files when output intentionally changes:
//!
//! ```bash
//! UPDATE_GOLDEN=1 NETBOX_URL=... NETBOX_TOKEN=... cargo test -p netbox-cli --test golden -- --ignored
//! ```

use std::path::PathBuf;
use std::process::Command;

fn env_var(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.trim().is_empty())
}

fn cli_binary() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("target");
    path.push("debug");
    path.push("netbox-cli");
    path
}

fn golden_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("golden");
    path
}

fn run_cli(args: &[&str]) -> (String, String, i32) {
    let url = env_var("NETBOX_URL").unwrap_or_else(|| "http://localhost:8000".to_string());
    let token = env_var("NETBOX_TOKEN").expect("NETBOX_TOKEN required");

    let output = Command::new(cli_binary())
        .args(["--url", &url, "--token", &token])
        .args(args)
        .output()
        .expect("failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let code = output.status.code().unwrap_or(-1);

    (stdout, stderr, code)
}

/// Normalize output for comparison by removing dynamic fields.
fn normalize_output(output: &str) -> String {
    let mut lines: Vec<String> = output
        .lines()
        .map(|line| {
            // clap includes current env var values in help output, which varies in CI.
            if let Some(prefix) = line.split("[env: NETBOX_URL=").next() {
                if line.contains("[env: NETBOX_URL=") {
                    return format!("{prefix}[env: NETBOX_URL=]");
                }
            }
            if let Some(prefix) = line.split("[env: NETBOX_TOKEN=").next() {
                if line.contains("[env: NETBOX_TOKEN=") {
                    return format!("{prefix}[env: NETBOX_TOKEN=]");
                }
            }
            line.to_string()
        })
        .collect();

    // Remove trailing empty lines
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }

    lines.join("\n")
}

/// Compare output against golden file, updating if UPDATE_GOLDEN=1.
fn check_golden(name: &str, actual: &str) {
    let golden_path = golden_dir().join(format!("{}.txt", name));
    let normalized = normalize_output(actual);

    if env_var("UPDATE_GOLDEN").is_some() {
        std::fs::create_dir_all(golden_dir()).ok();
        std::fs::write(&golden_path, &normalized).expect("failed to write golden file");
        eprintln!("Updated golden file: {}", golden_path.display());
        return;
    }

    if !golden_path.exists() {
        panic!(
            "Golden file not found: {}\nRun with UPDATE_GOLDEN=1 to create it.\nActual output:\n{}",
            golden_path.display(),
            normalized
        );
    }

    let expected = std::fs::read_to_string(&golden_path).expect("failed to read golden file");
    let expected = normalize_output(&expected);

    if normalized != expected {
        panic!(
            "Output mismatch for {}.\n\n--- Expected ---\n{}\n\n--- Actual ---\n{}\n\nRun with UPDATE_GOLDEN=1 to update.",
            name, expected, normalized
        );
    }
}

#[test]
#[ignore]
fn golden_status_json() {
    // Build CLI first
    let status = Command::new("cargo")
        .args(["build", "-p", "netbox-cli"])
        .status()
        .expect("failed to build CLI");
    assert!(status.success(), "CLI build failed");

    let (stdout, stderr, code) = run_cli(&["--output", "json", "status"]);
    assert_eq!(code, 0, "CLI failed: {}", stderr);

    // For status, we just check structure since values are dynamic
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON output");
    assert!(
        json.get("netbox-version").is_some(),
        "missing netbox-version"
    );
    assert!(
        json.get("python-version").is_some(),
        "missing python-version"
    );
}

#[test]
#[ignore]
fn golden_help_output() {
    let status = Command::new("cargo")
        .args(["build", "-p", "netbox-cli"])
        .status()
        .expect("failed to build CLI");
    assert!(status.success(), "CLI build failed");

    let output = Command::new(cli_binary())
        .arg("--help")
        .output()
        .expect("failed to run CLI");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    check_golden("help", &stdout);
}

#[test]
#[ignore]
fn golden_list_json_structure() {
    let status = Command::new("cargo")
        .args(["build", "-p", "netbox-cli"])
        .status()
        .expect("failed to build CLI");
    assert!(status.success(), "CLI build failed");

    let (stdout, stderr, code) = run_cli(&[
        "--output", "json", "dcim", "sites", "list", "--query", "limit=1",
    ]);
    assert_eq!(code, 0, "CLI failed: {}", stderr);

    // Verify JSON structure (content varies by instance)
    let json: serde_json::Value = serde_json::from_str(&stdout).expect("invalid JSON output");
    assert!(json.get("count").is_some(), "missing count field");
    assert!(json.get("results").is_some(), "missing results field");
}

#[test]
#[ignore]
fn golden_table_headers() {
    let status = Command::new("cargo")
        .args(["build", "-p", "netbox-cli"])
        .status()
        .expect("failed to build CLI");
    assert!(status.success(), "CLI build failed");

    let (stdout, stderr, code) = run_cli(&[
        "--output",
        "table",
        "--columns",
        "id,name,slug",
        "extras",
        "tags",
        "list",
        "--query",
        "limit=0",
    ]);
    assert_eq!(code, 0, "CLI failed: {}", stderr);

    // Even with 0 results, table should show headers
    assert!(stdout.contains("id"), "missing id column header");
    assert!(stdout.contains("name"), "missing name column header");
    assert!(stdout.contains("slug"), "missing slug column header");
}
