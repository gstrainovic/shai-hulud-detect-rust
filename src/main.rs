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
#[allow(clippy::too_many_lines)]
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

    // Load verification resolvers (if --verify flag is set)
    // NOTE: Status messages are Rust-only feature, tests ignore them via strip_verification_data
    let (lockfile_resolver, mut runtime_resolver) = if args.verify {
        // Try lockfile first (static analysis)
        let lockfile =
            match detectors::lockfile_resolver::LockfileResolver::load_from_dir(&args.scan_dir) {
                Ok(resolver) if resolver.has_lockfile() => {
                    colors::print_status(
                        colors::Color::Green,
                        &format!(
                            "✅ Lockfile loaded ({:?} format, {} packages)",
                            resolver.lockfile_type.as_ref().unwrap(),
                            resolver.packages.len()
                        ),
                    );
                    Some(resolver)
                }
                _ => None,
            };

        // Try runtime resolution (actual installed packages)
        colors::print_status(
            colors::Color::Blue,
            "🔍 Querying package manager for installed versions...",
        );
        let runtime =
            match detectors::runtime_resolver::RuntimeResolver::from_runtime(&args.scan_dir) {
                Ok(resolver) if resolver.has_packages() => {
                    colors::print_status(
                        colors::Color::Green,
                        &format!(
                            "✅ Runtime resolver: {} packages found",
                            resolver.packages.len()
                        ),
                    );
                    Some(resolver)
                }
                Ok(_) => {
                    colors::print_status(
                        colors::Color::Yellow,
                        "⚠️  Runtime resolution failed - using lockfile only",
                    );
                    None
                }
                Err(e) => {
                    colors::print_status(
                        colors::Color::Yellow,
                        &format!("⚠️  Runtime resolution error: {e} - using lockfile only"),
                    );
                    None
                }
            };

        (lockfile, runtime)
    } else {
        (None, None)
    };

    // Create results container
    let mut results = detectors::ScanResults::new();

    // Run core Shai-Hulud detection checks (matching bash function execution order)
    // 1. check_workflow_files
    results.workflow_files = detectors::workflow::check_workflow_files(&args.scan_dir);

    // 2. check_file_hashes
    results.malicious_hashes =
        detectors::hashes::check_file_hashes(&args.scan_dir, &malicious_hashes, args.parallelism);

    // 3. check_packages
    let (comp, susp, lockfile_safe, ns) = detectors::packages::check_packages(
        &args.scan_dir,
        &compromised_packages,
        lockfile_resolver.as_ref(),
        runtime_resolver.as_mut(),
    );
    results.compromised_found = comp;
    results.suspicious_found = susp;
    results.lockfile_safe_versions = lockfile_safe;
    results.namespace_warnings = ns;

    // 3.5. check_semver_ranges (if enabled)
    if args.check_semver_ranges {
        let semver_findings = detectors::packages::check_semver_ranges(
            &args.scan_dir,
            &compromised_packages,
            lockfile_resolver.as_ref(),
        );
        results.lockfile_safe_versions.extend(semver_findings);
    }

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

    // 11-20. November 2025 "Shai-Hulud: The Second Coming" Attack detectors
    // check_bun_attack_files (combines setup_bun.js and bun_environment.js)
    let bun_findings = detectors::bun_attack::check_bun_attack_files(&args.scan_dir);
    for finding in bun_findings {
        if finding.category == "bun_setup_files" {
            results.bun_setup_files.push(finding);
        } else if finding.category == "bun_environment_files" {
            results.bun_environment_files.push(finding);
        }
    }

    // check_new_workflow_patterns
    let new_workflow_findings =
        detectors::workflows_new::check_new_workflow_patterns(&args.scan_dir);
    for finding in new_workflow_findings {
        if finding.category == "new_workflow_files" {
            results.new_workflow_files.push(finding);
        } else if finding.category == "actions_secrets_files" {
            results.actions_secrets_files.push(finding);
        }
    }

    // check_discussion_workflows
    results.discussion_workflows =
        detectors::discussion_workflows::check_discussion_workflows(&args.scan_dir);

    // check_github_runners
    results.github_runners = detectors::github_runners::check_github_runners(&args.scan_dir);

    // check_destructive_patterns
    results.destructive_patterns =
        detectors::destructive_patterns::check_destructive_patterns(&args.scan_dir);

    // check_preinstall_bun_patterns
    results.preinstall_bun_patterns =
        detectors::preinstall_bun::check_preinstall_bun_patterns(&args.scan_dir);

    // check_github_actions_runner (SHA1HULUD)
    results.github_sha1hulud_runners =
        detectors::sha1hulud_runner::check_github_actions_runner(&args.scan_dir);

    // check_second_coming_repos
    results.second_coming_repos =
        detectors::second_coming::check_second_coming_repos(&args.scan_dir);

    // Run additional security checks only in paranoid mode
    if args.paranoid {
        colors::print_status(
            colors::Color::Blue,
            "Checking for typosquatting and homoglyph attacks...",
        );
        results.typosquatting_warnings =
            detectors::typosquatting::check_typosquatting(&args.scan_dir);

        colors::print_status(
            colors::Color::Blue,
            "Checking for network exfiltration patterns...",
        );
        results.network_exfiltration_warnings =
            detectors::network::check_network_exfiltration(&args.scan_dir);
    }

    // Calculate total_issues using ScanResults methods which include all detectors
    let high_risk = results.high_risk_count();
    let medium_risk = results.medium_risk_count(args.paranoid);
    let total_issues = high_risk + medium_risk;

    // BASH EXACT: Apply namespace warning logic - only include in results if they would be shown
    // Bash shows namespace warnings in detail only when total_issues == 0 OR total_issues < 5
    if total_issues >= 5 {
        // Store count for bash compatibility before filtering
        results.suppressed_namespace_count = results.namespace_warnings.len();
        // Too many critical issues - don't include namespace warnings in detailed output/JSON
        results.namespace_warnings = Vec::new();
    }
    // If total_issues == 0 or < 5, keep namespace warnings as-is

    // Generate report
    report::generate_report(&results, args.paranoid);

    // BASH COMPATIBILITY: Remove LOW RISK findings from JSON if total_issues >= 5
    // (Bash doesn't show them in output, so they shouldn't be in our JSON either)
    let mut results_for_json = results.clone();
    if total_issues >= 5 {
        // Remove LOW RISK findings (namespace warnings and crypto patterns with LOW risk)
        results_for_json.namespace_warnings.clear();
        results_for_json
            .crypto_patterns
            .retain(|f| f.risk_level != detectors::RiskLevel::Low);
    }

    // BASH COMPATIBILITY: Truncate paranoid mode findings to max 5 (like Bash does)
    // Bash only shows first 5 typosquatting and first 5 network exfiltration warnings
    if args.paranoid {
        results_for_json.typosquatting_warnings.truncate(5);
        results_for_json.network_exfiltration_warnings.truncate(5);
    }

    // Save JSON output for pattern-level verification
    // Save in current directory by default (can be redirected in scripts)
    let json_output_path = "scan_results.json";
    let json_output = serde_json::to_string_pretty(&results_for_json)?;
    std::fs::write(json_output_path, json_output)?;
    colors::print_status(
        colors::Color::Green,
        &format!("💾 JSON results saved: {json_output_path}"),
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

    // Return appropriate exit code based on findings (matching bash script)
    if high_risk > 0 {
        std::process::exit(1); // High risk findings detected
    } else if medium_risk > 0 {
        std::process::exit(2); // Medium risk findings detected
    } else {
        Ok(()) // Clean - no significant findings (exit code 0)
    }
}
