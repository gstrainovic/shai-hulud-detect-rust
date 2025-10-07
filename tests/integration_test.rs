use std::process::Command;

#[test]
fn test_normal_mode_100_percent_match() {
    // Test that we maintain 100% match with Bash in normal mode
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "../shai-hulud-detect/test-cases"])
        .output()
        .expect("Failed to run scanner");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify exact counts
    assert!(stdout.contains("High Risk Issues: 19"), "HIGH should be 19");
    assert!(
        stdout.contains("Medium Risk Issues: 61"),
        "MEDIUM should be 61"
    );
    assert!(
        stdout.contains("Low Risk (informational): 9"),
        "LOW should be 9"
    );
}

#[test]
fn test_infected_project_normal_mode() {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--",
            "../shai-hulud-detect/test-cases/infected-project",
        ])
        .output()
        .expect("Failed to run scanner");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Normal mode (no --paranoid) should give 8/16/2
    assert!(stdout.contains("High Risk Issues: 8"), "HIGH should be 8");
    assert!(
        stdout.contains("Medium Risk Issues: 16"),
        "MEDIUM should be 16"
    );
    assert!(
        stdout.contains("Low Risk (informational): 2"),
        "LOW should be 2"
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
fn test_paranoid_mode_enhanced_security() {
    // Test that paranoid mode finds more issues (network + typosquatting)
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--release",
            "--",
            "../shai-hulud-detect/test-cases/infected-project",
            "--paranoid",
        ])
        .output()
        .expect("Failed to run scanner");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Paranoid mode should find more MEDIUM (19 vs 16 in normal)
    assert!(stdout.contains("High Risk Issues: 8"), "HIGH should be 8");
    assert!(
        stdout.contains("Medium Risk Issues: 19"),
        "MEDIUM should be 19 in paranoid"
    );
    assert!(
        stdout.contains("Low Risk (informational): 2"),
        "LOW should be 2"
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
