// Integration tests for Shai-Hulud detector
// Basic smoke tests for common scenarios

use std::process::Command;
use std::env;
use std::path::PathBuf;

fn get_workspace_root() -> PathBuf {
    let mut path = env::current_dir().expect("Failed to get current directory");
    if path.ends_with("dev-rust-scanner-1") {
        path.pop();
    }
    path
}

#[test]
fn test_clean_project() {
    let workspace = get_workspace_root();
    
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--release",
            "--",
            "../shai-hulud-detect/test-cases/clean-project",
        ])
        .current_dir(workspace.join("dev-rust-scanner-1"))
        .output()
        .expect("Failed to run scanner");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Clean project should have 0 issues
    assert!(
        stdout.contains("No indicators of Shai-Hulud compromise detected"),
        "Clean project should have no issues"
    );
}

#[test]
fn test_infected_project_detection() {
    let workspace = get_workspace_root();
    
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--release",
            "--",
            "../shai-hulud-detect/test-cases/infected-project",
        ])
        .current_dir(workspace.join("dev-rust-scanner-1"))
        .output()
        .expect("Failed to run scanner");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Infected project should detect issues
    assert!(
        stdout.contains("High Risk Issues:"),
        "Should detect high risk issues in infected project"
    );
    
    // Should find compromised packages
    assert!(
        stdout.contains("@ctrl") || stdout.contains("@nativescript"),
        "Should detect compromised namespaces"
    );
}

#[test]
fn test_homoglyph_detection() {
    let workspace = get_workspace_root();
    
    // Test that we detect Unicode homoglyphs in paranoid mode
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--quiet",
            "--release",
            "--",
            "--paranoid",
            "../shai-hulud-detect/test-cases/comprehensive-test",
        ])
        .current_dir(workspace.join("dev-rust-scanner-1"))
        .output()
        .expect("Failed to run scanner");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect Unicode homoglyph patterns in paranoid mode
    assert!(
        stdout.contains("Unicode") || stdout.contains("homoglyph") || stdout.contains("Typosquatting"),
        "Should detect Unicode homoglyphs in paranoid mode"
    );
}
