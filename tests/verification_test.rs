// Integration tests for verification
// These tests ensure 100% compatibility with Bash scanner

use std::process::Command;

#[test]
fn test_verify_normal_mode_100_percent() {
    println!("🧪 Running normal mode verification...");
    
    let output = Command::new("bash")
        .arg("dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan.sh")
        .current_dir("/c/Users/gstra/Code/rust-scanner")
        .output()
        .expect("Failed to run parallel_testcase_scan.sh");
    
    assert!(output.status.success(), "Parallel scan failed");
    
    let verify = Command::new("bash")
        .arg("dev-rust-scanner-1/scripts/analyze/verify_100_percent.sh")
        .current_dir("/c/Users/gstra/Code/rust-scanner")
        .output()
        .expect("Failed to run verify_100_percent.sh");
    
    let stdout = String::from_utf8_lossy(&verify.stdout);
    println!("{}", stdout);
    
    assert!(verify.status.success(), "Normal mode verification failed: {}", stdout);
    assert!(stdout.contains("100% MATCH ACHIEVED"), "Did not achieve 100% match");
}

#[test]
fn test_verify_paranoid_mode_100_percent() {
    println!("🧪 Running paranoid mode verification...");
    
    let output = Command::new("bash")
        .arg("dev-rust-scanner-1/scripts/analyze/parallel_testcase_scan_paranoid.sh")
        .current_dir("/c/Users/gstra/Code/rust-scanner")
        .output()
        .expect("Failed to run parallel_testcase_scan_paranoid.sh");
    
    assert!(output.status.success(), "Parallel paranoid scan failed");
    
    let verify = Command::new("bash")
        .arg("dev-rust-scanner-1/scripts/analyze/verify_100_percent_paranoid.sh")
        .current_dir("/c/Users/gstra/Code/rust-scanner")
        .output()
        .expect("Failed to run verify_100_percent_paranoid.sh");
    
    let stdout = String::from_utf8_lossy(&verify.stdout);
    println!("{}", stdout);
    
    assert!(verify.status.success(), "Paranoid mode verification failed: {}", stdout);
    assert!(stdout.contains("100% MATCH ACHIEVED"), "Did not achieve 100% match in paranoid mode");
}

#[test]
fn test_single_testcase_infected_project() {
    println!("🧪 Testing single test case: infected-project...");
    
    // Run bash
    let bash_output = Command::new("bash")
        .arg("-c")
        .arg("cd shai-hulud-detect && ./shai-hulud-detector.sh test-cases/infected-project")
        .current_dir("/c/Users/gstra/Code/rust-scanner")
        .output()
        .expect("Failed to run bash scanner");
    
    // Run rust
    let rust_output = Command::new("cargo")
        .args(&["run", "--quiet", "--release", "--", "../shai-hulud-detect/test-cases/infected-project"])
        .current_dir("/c/Users/gstra/Code/rust-scanner/dev-rust-scanner-1")
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
    
    assert_eq!(bash_high, rust_high, "HIGH risk mismatch");
    assert_eq!(bash_med, rust_med, "MEDIUM risk mismatch");
    assert_eq!(bash_low, rust_low, "LOW risk mismatch");
}

fn extract_number(text: &str, pattern: &str) -> u32 {
    text.lines()
        .find(|line| line.contains(pattern))
        .and_then(|line| line.split_whitespace().last())
        .and_then(|num| num.parse().ok())
        .unwrap_or(0)
}
