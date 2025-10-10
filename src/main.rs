// Shai-Hulud NPM Supply Chain Attack Detection Tool - Rust Implementation
// 1:1 Port of shai-hulud-detector.sh
//
// This is a complete Rust port maintaining exact bash script logic and comments
// Each function corresponds to a bash function from the original script

mod cli;
mod colors;
mod data;
mod detectors;
mod report;
mod semver;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use std::time::Instant;

// Function: main
// Purpose: Main entry point - parse arguments, load data, run all checks, generate report
// Args: Command line arguments (--paranoid, --help, --parallelism N, directory_path)
// Modifies: All global arrays via detection functions
// Returns: Exit code 0 for clean, 1 for high-risk findings, 2 for medium-risk findings
fn main() -> Result<()> {
    let start_time = Instant::now();
    let start_timestamp = chrono::Local::now();

    let mut args = Cli::parse();
    args.validate()?;

    // Load compromised packages from external file
    // Try multiple locations: same dir as exe, parent dir, or fallback
    let script_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();

    // First try: exe_dir/../../../shai-hulud-detect/compromised-packages.txt (for dev)
    // Second try: exe_dir/../compromised-packages.txt (for release)
    // Third try: ./compromised-packages.txt (fallback)
    let packages_file = script_dir
        .parent()
        .and_then(|target| target.parent()) // dev-rust-scanner-1
        .and_then(|project| project.parent()) // rust-scanner
        .map(|root| root.join("shai-hulud-detect/compromised-packages.txt"))
        .filter(|p| p.exists())
        .or_else(|| {
            script_dir
                .parent()
                .map(|p| p.join("compromised-packages.txt"))
                .filter(|p| p.exists())
        })
        .unwrap_or_else(|| std::path::PathBuf::from("compromised-packages.txt"));

    let (compromised_packages, malicious_hashes) = data::load_detection_data(&packages_file)?;

    colors::print_status(
        colors::Color::Green,
        "Starting Shai-Hulud detection scan...",
    );

    let paranoid_msg = if args.paranoid {
        format!(
            "Scanning directory: {} (with paranoid mode enabled)",
            utils::normalize_path(&args.scan_dir)
        )
    } else {
        format!(
            "Scanning directory: {}",
            utils::normalize_path(&args.scan_dir)
        )
    };
    colors::print_status(colors::Color::Blue, &paranoid_msg);
    println!();

    // Create results container
    let mut results = detectors::ScanResults::new();

    // Run core Shai-Hulud detection checks (matching bash function execution order)
    // 1. check_workflow_files
    results.workflow_files = detectors::workflow::check_workflow_files(&args.scan_dir);

    // 2. check_file_hashes
    results.malicious_hashes =
        detectors::hashes::check_file_hashes(&args.scan_dir, &malicious_hashes, args.parallelism);

    // 3. check_packages
    let (comp, susp, lockfile_safe, ns) =
        detectors::packages::check_packages(&args.scan_dir, &compromised_packages);
    results.compromised_found = comp;
    results.suspicious_found = susp;
    results.lockfile_safe_versions = lockfile_safe;
    results.namespace_warnings = ns;

    // 4. check_postinstall_hooks
    results.postinstall_hooks = detectors::postinstall::check_postinstall_hooks(&args.scan_dir);

    // 5. check_content
    results.suspicious_content = detectors::content::check_content(&args.scan_dir);

    // 6. check_crypto_theft_patterns
    results.crypto_patterns = detectors::crypto::check_crypto_theft_patterns(&args.scan_dir);

    // 7. check_trufflehog_activity
    results.trufflehog_activity = detectors::trufflehog::check_trufflehog_activity(&args.scan_dir);

    // 8. check_git_branches
    results.git_branches = detectors::git::check_git_branches(&args.scan_dir);

    // 9. check_shai_hulud_repos
    results.shai_hulud_repos = detectors::repos::check_shai_hulud_repos(&args.scan_dir);

    // 10. check_package_integrity
    results.integrity_issues =
        detectors::integrity::check_package_integrity(&args.scan_dir, &compromised_packages);

    // Run additional security checks only in paranoid mode
    if args.paranoid {
        colors::print_status(
            colors::Color::Blue,
            "🔍+ Checking for typosquatting and homoglyph attacks...",
        );
        results.typosquatting_warnings =
            detectors::typosquatting::check_typosquatting(&args.scan_dir);

        colors::print_status(
            colors::Color::Blue,
            "🔍+ Checking for network exfiltration patterns...",
        );
        results.network_exfiltration_warnings =
            detectors::network::check_network_exfiltration(&args.scan_dir);
    }

    // Generate report
    report::generate_report(&results, args.paranoid);

    // Save JSON output for pattern-level verification
    // Save in current directory by default (can be redirected in scripts)
    let json_output_path = "scan_results.json";
    let json_output = serde_json::to_string_pretty(&results)?;
    std::fs::write(&json_output_path, json_output)?;
    colors::print_status(
        colors::Color::Green,
        &format!("💾 JSON results saved: {}", json_output_path),
    );

    // Print timing information
    let end_timestamp = chrono::Local::now();
    let duration = start_time.elapsed();

    println!();
    colors::print_status(
        colors::Color::Blue,
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    );
    colors::print_status(colors::Color::Blue, "⏱️  TIMING");
    colors::print_status(
        colors::Color::Blue,
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    );
    println!(
        "   Started:  {}",
        start_timestamp.format("%Y-%m-%d %H:%M:%S")
    );
    println!("   Finished: {}", end_timestamp.format("%Y-%m-%d %H:%M:%S"));
    println!("   Duration: {:.2}s", duration.as_secs_f64());
    colors::print_status(
        colors::Color::Blue,
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    );
    println!();

    // IMPORTANT: Bash script DOES NOT exit with error codes based on findings!
    // It always exits with 0, even if HIGH/MEDIUM risk issues are found.
    // This matches the original bash behavior for 100% compatibility.

    Ok(())
}
