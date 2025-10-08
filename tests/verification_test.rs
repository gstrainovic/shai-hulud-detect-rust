// Integration tests for verification
// These tests ensure 100% compatibility with Bash scanner

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn get_workspace_root() -> PathBuf {
    // Get the current directory (should be in dev-rust-scanner-1)
    let mut path = env::current_dir().expect("Failed to get current directory");

    // Go up to rust-scanner directory
    if path.ends_with("dev-rust-scanner-1") {
        path.pop();
    }

    path
}

#[test]
fn test_single_testcase_infected_project() {
    println!("🧪 Testing single test case: infected-project...");

    let workspace = get_workspace_root();

    // Run bash
    let bash_output = Command::new("bash")
        .arg("-c")
        .arg("cd shai-hulud-detect && ./shai-hulud-detector.sh test-cases/infected-project")
        .current_dir(&workspace)
        .output()
        .expect("Failed to run bash scanner");

    // Run rust
    let rust_output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--release",
            "--",
            "../shai-hulud-detect/test-cases/infected-project",
        ])
        .current_dir(workspace.join("dev-rust-scanner-1"))
        .output()
        .expect("Failed to run rust scanner");

    let bash_stdout = String::from_utf8_lossy(&bash_output.stdout);
    let rust_stdout = String::from_utf8_lossy(&rust_output.stdout);

    // Extract summaries
    let bash_high = extract_number(&bash_stdout, "High Risk Issues:");
    let bash_med = extract_number(&bash_stdout, "Medium Risk Issues:");
    let bash_low = extract_number(&bash_stdout, "Low Risk");

    let rust_high = extract_number(&rust_stdout, "High Risk Issues:");
    let rust_med = extract_number(&rust_stdout, "Medium Risk Issues:");
    let rust_low = extract_number(&rust_stdout, "Low Risk");

    println!("Bash: {}/{}/{}", bash_high, bash_med, bash_low);
    println!("Rust: {}/{}/{}", rust_high, rust_med, rust_low);

    assert_eq!(bash_high, rust_high, "HIGH risk mismatch");
    assert_eq!(bash_med, rust_med, "MEDIUM risk mismatch");
    assert_eq!(bash_low, rust_low, "LOW risk mismatch");
}

fn extract_number(text: &str, pattern: &str) -> u32 {
    // Strip ANSI color codes first
    let stripped = strip_ansi_codes(text);

    stripped
        .lines()
        .find(|line| line.contains(pattern))
        .and_then(|line| line.split_whitespace().last())
        .and_then(|num| num.parse().ok())
        .unwrap_or(0)
}

fn strip_ansi_codes(text: &str) -> String {
    // Remove ANSI escape sequences like \x1b[0;31m
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Skip escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == 'm' {
                        break;
                    }
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}
