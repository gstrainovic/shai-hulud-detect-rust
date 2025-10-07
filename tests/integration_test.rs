use std::process::Command;

#[test]
fn test_normal_mode_100_percent_match() {
    // Run the dynamic verification script (compares Rust vs Bash live)
    let output = Command::new("bash")
        .args(&["dev-rust-scanner-1/scripts/verification/verify_normal_mode.sh"])
        .current_dir("../")
        .output()
        .expect("Failed to run verification script");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout); // Show output for debugging

    // Script exits 0 if match, 1 if mismatch
    assert!(
        output.status.success(),
        "Normal mode verification failed! Rust doesn't match Bash.\n{}",
        stdout
    );
}

#[test]
fn test_paranoid_mode_100_percent_match() {
    // Run the dynamic verification script (compares Rust vs Bash live)
    let output = Command::new("bash")
        .args(&["dev-rust-scanner-1/scripts/verification/verify_paranoid_mode.sh"])
        .current_dir("../")
        .output()
        .expect("Failed to run verification script");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout); // Show output for debugging

    // Script exits 0 if match, 1 if mismatch
    assert!(
        output.status.success(),
        "Paranoid mode verification failed! Rust doesn't match Bash.\n{}",
        stdout
    );
}

#[test]
fn test_clean_project() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--",
            "../shai-hulud-detect/test-cases/clean-project",
        ])
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
fn test_homoglyph_detection() {
    // Test that we detect Unicode homoglyphs (better than Bash)
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--",
            "../shai-hulud-detect/test-cases/comprehensive-test",
            "--paranoid",
        ])
        .output()
        .expect("Failed to run scanner");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect Unicode homoglyph in "reаct" (Cyrillic 'а')
    assert!(
        stdout.contains("Unicode/homoglyph") || stdout.contains("reаct"),
        "Should detect Unicode homoglyphs"
    );
}
