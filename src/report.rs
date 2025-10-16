// Report Generation
// Rust port of: generate_report()

use crate::colors::{print_status, Color};
use crate::detectors::{verification, RiskLevel, ScanResults};
use std::path::Path;

// Helper: show_file_preview
// Purpose: Display file context for HIGH RISK findings only (Bash exact match)
// Args: file_path - path to file, context - description
fn show_file_preview(file_path: &Path, context: &str) {
    // Only show file preview for HIGH RISK items to reduce noise
    if context.contains("HIGH RISK") {
        let normalized = crate::utils::normalize_path(file_path);
        println!("   \x1b[34m┌─ File: {}\x1b[0m", normalized);
        println!("   \x1b[34m│  Context: {}\x1b[0m", context);
        println!("   \x1b[34m└─\x1b[0m");
        println!();
    }
}

// Function: generate_report
// Purpose: Generate comprehensive security report with risk stratification and findings
// Args: results - scan results, paranoid_mode - whether paranoid mode is enabled
// Modifies: None (reads all global finding arrays)
// Returns: Outputs formatted report to stdout with HIGH/MEDIUM/LOW risk sections
pub fn generate_report(results: &ScanResults, paranoid_mode: bool) {
    println!();
    print_status(
        Color::Blue,
        "==============================================",
    );

    if paranoid_mode {
        print_status(Color::Blue, "  SHAI-HULUD + PARANOID SECURITY REPORT");
    } else {
        print_status(Color::Blue, "      SHAI-HULUD DETECTION REPORT");
    }

    print_status(
        Color::Blue,
        "==============================================",
    );
    println!();

    let high_risk = results.high_risk_count();
    let medium_risk = results.medium_risk_count(paranoid_mode);
    let low_risk = results.low_risk_count();
    let total_issues = high_risk + medium_risk;

    // Report malicious workflow files
    if !results.workflow_files.is_empty() {
        print_status(Color::Red, "HIGH RISK: Malicious workflow files detected:");
        for finding in &results.workflow_files {
            println!("   - {}", crate::utils::normalize_path(&finding.file_path));
            show_file_preview(
                &finding.file_path,
                "HIGH RISK: Known malicious workflow filename",
            );
        }
    }

    // Report malicious file hashes
    if !results.malicious_hashes.is_empty() {
        print_status(Color::Red, "HIGH RISK: Files with known malicious hashes:");
        for finding in &results.malicious_hashes {
            println!("   - {}", crate::utils::normalize_path(&finding.file_path));
            println!("     {}", finding.message);
        }
        println!();
    }

    // Report compromised packages
    if !results.compromised_found.is_empty() {
        print_status(
            Color::Red,
            "HIGH RISK: Compromised package versions detected:",
        );

        // Sort by package name for consistent output (bash does this too)
        let mut sorted_findings = results.compromised_found.clone();
        sorted_findings.sort_by(|a, b| a.message.cmp(&b.message));

        for finding in &sorted_findings {
            println!("   - Package: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
            show_file_preview(
                &finding.file_path,
                &format!(
                    "HIGH RISK: Contains compromised package version: {}",
                    finding.message
                ),
            );
        }
        print_status(
            Color::Yellow,
            "   NOTE: These specific package versions are known to be compromised.",
        );
        print_status(
            Color::Yellow,
            "   You should immediately update or remove these packages.",
        );
        println!();
    }

    // Report suspicious packages
    if !results.suspicious_found.is_empty() {
        print_status(
            Color::Yellow,
            "MEDIUM RISK: Suspicious package versions detected:",
        );
        for finding in &results.suspicious_found {
            println!("   - Package: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );

            // Show verification status if present
            if let Some(verification_status) = &finding.verification {
                match verification_status {
                    verification::VerificationStatus::Verified {
                        reason, confidence, ..
                    } => {
                        print_status(
                            Color::Green,
                            &format!(
                                "     [VERIFIED SAFE - {:?} confidence]: {}",
                                confidence, reason
                            ),
                        );
                    }
                    verification::VerificationStatus::Compromised { reason } => {
                        print_status(
                            Color::Red,
                            &format!("     [VERIFIED COMPROMISED]: {}", reason),
                        );
                    }
                    verification::VerificationStatus::Suspicious { reason } => {
                        print_status(Color::Yellow, &format!("     [SUSPICIOUS]: {}", reason));
                    }
                    verification::VerificationStatus::Unknown => {}
                }
            }
        }
        print_status(
            Color::Yellow,
            "   NOTE: Manual review required to determine if these are malicious.",
        );
        println!();
    }

    // Report lockfile-safe packages (BASH LINE 1440-1453)
    if !results.lockfile_safe_versions.is_empty() {
        print_status(
            Color::Blue,
            "ℹ️  LOW RISK: Packages with safe lockfile versions:",
        );
        for finding in &results.lockfile_safe_versions {
            println!("   - Package: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
        }
        print_status(
            Color::Blue,
            "   NOTE: These package.json ranges could match compromised versions, but lockfiles pin to safe versions.",
        );
        print_status(
            Color::Blue,
            "   Your current installation is safe. Avoid running 'npm update' without reviewing changes.",
        );
        println!();
    }

    // Report suspicious content
    if !results.suspicious_content.is_empty() {
        print_status(Color::Yellow, "MEDIUM RISK: Suspicious content patterns:");
        for finding in &results.suspicious_content {
            println!("   - Pattern: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
        }
        print_status(
            Color::Yellow,
            "   NOTE: Manual review required to determine if these are malicious.",
        );
        println!();
    }

    // Report cryptocurrency theft patterns (separated by risk level)
    let crypto_high: Vec<_> = results
        .crypto_patterns
        .iter()
        .filter(|f| f.risk_level == RiskLevel::High)
        .collect();
    let crypto_medium: Vec<_> = results
        .crypto_patterns
        .iter()
        .filter(|f| f.risk_level == RiskLevel::Medium)
        .collect();

    if !crypto_high.is_empty() {
        print_status(
            Color::Red,
            "HIGH RISK: Cryptocurrency theft patterns detected:",
        );
        for finding in crypto_high {
            println!(
                "   - {}:{}",
                crate::utils::normalize_path(&finding.file_path),
                finding.message
            );
        }
        print_status(
            Color::Red,
            "   NOTE: These patterns strongly indicate crypto theft malware from the September 8 attack.",
        );
        print_status(
            Color::Red,
            "   Immediate investigation and remediation required.",
        );
        println!();
    }

    if !crypto_medium.is_empty() {
        print_status(
            Color::Yellow,
            "MEDIUM RISK: Potential cryptocurrency manipulation patterns:",
        );
        for finding in crypto_medium {
            println!(
                "   - {}:{}",
                crate::utils::normalize_path(&finding.file_path),
                finding.message
            );

            // Show verification status if present
            if let Some(verification_status) = &finding.verification {
                match verification_status {
                    verification::VerificationStatus::Verified {
                        reason, confidence, ..
                    } => {
                        print_status(
                            Color::Green,
                            &format!(
                                "     [VERIFIED SAFE - {:?} confidence]: {}",
                                confidence, reason
                            ),
                        );
                    }
                    verification::VerificationStatus::Compromised { reason } => {
                        print_status(
                            Color::Red,
                            &format!("     [VERIFIED COMPROMISED]: {}", reason),
                        );
                    }
                    verification::VerificationStatus::Suspicious { reason } => {
                        print_status(Color::Yellow, &format!("     [SUSPICIOUS]: {}", reason));
                    }
                    verification::VerificationStatus::Unknown => {}
                }
            }
        }
        print_status(
            Color::Yellow,
            "   NOTE: These may be legitimate crypto tools or framework code.",
        );
        print_status(Color::Yellow, "   Manual review required.");
        println!();
    }

    // Report git branches
    if !results.git_branches.is_empty() {
        print_status(Color::Yellow, "MEDIUM RISK: Suspicious git branches:");
        for finding in &results.git_branches {
            println!(
                "   - Repository: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
            println!("     {}", finding.message);
        }
        print_status(
            Color::Yellow,
            "   NOTE: 'shai-hulud' branches may indicate compromise.",
        );
        println!();
    }

    // Report suspicious postinstall hooks
    if !results.postinstall_hooks.is_empty() {
        print_status(
            Color::Red,
            "HIGH RISK: Suspicious postinstall hooks detected:",
        );
        for finding in &results.postinstall_hooks {
            println!("   - Hook: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );

            // Show verification status if present
            if let Some(verification_status) = &finding.verification {
                match verification_status {
                    verification::VerificationStatus::Verified {
                        reason, confidence, ..
                    } => {
                        print_status(
                            Color::Green,
                            &format!(
                                "     [VERIFIED SAFE - {:?} confidence]: {}",
                                confidence, reason
                            ),
                        );
                    }
                    verification::VerificationStatus::Compromised { reason } => {
                        print_status(
                            Color::Red,
                            &format!("     [VERIFIED COMPROMISED]: {}", reason),
                        );
                    }
                    verification::VerificationStatus::Suspicious { reason } => {
                        print_status(Color::Yellow, &format!("     [SUSPICIOUS]: {}", reason));
                    }
                    verification::VerificationStatus::Unknown => {
                        // Don't print anything for unknown
                    }
                }
            }
        }
        print_status(
            Color::Yellow,
            "   NOTE: Postinstall hooks can execute arbitrary code during package installation.",
        );
        print_status(
            Color::Yellow,
            "   Review these hooks carefully for malicious behavior.",
        );
        println!();
    }

    // Report Shai-Hulud repositories
    if !results.shai_hulud_repos.is_empty() {
        print_status(Color::Red, "HIGH RISK: Shai-Hulud repositories detected:");
        for finding in &results.shai_hulud_repos {
            println!(
                "   - Repository: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
            println!("     {}", finding.message);
        }
        print_status(
            Color::Yellow,
            "   NOTE: 'Shai-Hulud' repositories are created by the malware for exfiltration.",
        );
        print_status(
            Color::Yellow,
            "   These should be deleted immediately after investigation.",
        );
        println!();
    }

    // Report package integrity issues
    if !results.integrity_issues.is_empty() {
        print_status(
            Color::Yellow,
            "MEDIUM RISK: Package integrity issues detected:",
        );
        for finding in &results.integrity_issues {
            println!("   - Issue: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
        }
        print_status(
            Color::Yellow,
            "   NOTE: These issues may indicate tampering with package dependencies.",
        );
        println!();
    }

    // Report Trufflehog activity (separated by risk level)
    let trufflehog_high: Vec<_> = results
        .trufflehog_activity
        .iter()
        .filter(|f| f.risk_level == RiskLevel::High)
        .collect();
    let trufflehog_medium: Vec<_> = results
        .trufflehog_activity
        .iter()
        .filter(|f| f.risk_level == RiskLevel::Medium)
        .collect();

    if !trufflehog_high.is_empty() {
        print_status(
            Color::Red,
            "HIGH RISK: Trufflehog/secret scanning activity detected:",
        );
        for finding in trufflehog_high {
            println!("   - Activity: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
            show_file_preview(
                &finding.file_path,
                &format!("HIGH RISK: {}", finding.message),
            );
        }
        print_status(
            Color::Red,
            "   NOTE: These patterns indicate likely malicious credential harvesting.",
        );
        print_status(
            Color::Red,
            "   Immediate investigation and remediation required.",
        );
        println!();
    }

    if !trufflehog_medium.is_empty() {
        print_status(
            Color::Yellow,
            "MEDIUM RISK: Potentially suspicious secret scanning patterns:",
        );
        for finding in trufflehog_medium {
            println!("   - Pattern: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
        }
        print_status(
            Color::Yellow,
            "   NOTE: These may be legitimate security tools or framework code.",
        );
        print_status(
            Color::Yellow,
            "   Manual review recommended to determine if they are malicious.",
        );
        println!();
    }

    // BASH LINE 1513-1534: Report typosquatting warnings (only in paranoid mode)
    if paranoid_mode && !results.typosquatting_warnings.is_empty() {
        print_status(
            Color::Yellow,
            "MEDIUM RISK (PARANOID): Potential typosquatting/homoglyph attacks detected:",
        );
        for finding in results.typosquatting_warnings.iter().take(5) {
            println!("   - Warning: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
        }
        if results.typosquatting_warnings.len() > 5 {
            println!(
                "   - ... and {} more typosquatting warnings (truncated for brevity)",
                results.typosquatting_warnings.len() - 5
            );
        }
        print_status(
            Color::Yellow,
            "   NOTE: These packages may be impersonating legitimate packages.",
        );
        print_status(
            Color::Yellow,
            "   Verify package names carefully and check if they should be legitimate packages.",
        );
        println!();
    }

    // BASH LINE 1535-1556: Report network exfiltration warnings (only in paranoid mode)
    if paranoid_mode && !results.network_exfiltration_warnings.is_empty() {
        print_status(
            Color::Yellow,
            "MEDIUM RISK (PARANOID): Network exfiltration patterns detected:",
        );
        for finding in results.network_exfiltration_warnings.iter().take(5) {
            println!("   - Warning: {}", finding.message);
            println!(
                "     Found in: {}",
                crate::utils::normalize_path(&finding.file_path)
            );
        }
        if results.network_exfiltration_warnings.len() > 5 {
            println!(
                "   - ... and {} more network warnings (truncated for brevity)",
                results.network_exfiltration_warnings.len() - 5
            );
        }
        print_status(
            Color::Yellow,
            "   NOTE: These patterns may indicate data exfiltration or communication with C2 servers.",
        );
        print_status(
            Color::Yellow,
            "   Review network connections and data flows carefully.",
        );
        println!();
    }

    // Summary
    print_status(
        Color::Blue,
        "==============================================",
    );

    if total_issues == 0 {
        print_status(
            Color::Green,
            "✅ No indicators of Shai-Hulud compromise detected.",
        );
        print_status(
            Color::Green,
            "Your system appears clean from this specific attack.",
        );

        // Show low risk findings if any (informational only)
        if low_risk > 0 {
            println!();
            print_status(Color::Blue, "ℹ️  LOW RISK FINDINGS (informational only):");
            for finding in &results.namespace_warnings {
                println!("   - {}", finding.message);
            }
            print_status(
                Color::Blue,
                "   NOTE: These are likely legitimate framework code or dependencies.",
            );
        }
    } else {
        print_status(Color::Red, "SUMMARY:");
        print_status(Color::Red, &format!("   High Risk Issues: {}", high_risk));
        print_status(
            Color::Yellow,
            &format!("   Medium Risk Issues: {}", medium_risk),
        );
        if low_risk > 0 {
            print_status(
                Color::Blue,
                &format!("   Low Risk (informational): {}", low_risk),
            );
        }
        print_status(
            Color::Blue,
            &format!("   Total Critical Issues: {}", total_issues),
        );
        println!();
        print_status(Color::Yellow, "IMPORTANT:");
        print_status(
            Color::Yellow,
            "   - High risk issues likely indicate actual compromise",
        );
        print_status(
            Color::Yellow,
            "   - Medium risk issues require manual investigation",
        );
        print_status(
            Color::Yellow,
            "   - Low risk issues are likely false positives from legitimate code",
        );
        if paranoid_mode {
            print_status(
                Color::Yellow,
                "   - Issues marked (PARANOID) are general security checks, not Shai-Hulud specific",
            );
        }
        print_status(
            Color::Yellow,
            "   - Consider running additional security scans",
        );
        print_status(
            Color::Yellow,
            "   - Review your npm audit logs and package history",
        );

        // BASH EXACT: Show LOW risk findings in detail only if total_issues < 5
        if low_risk > 0 && total_issues < 5 {
            println!();
            print_status(
                Color::Blue,
                "ℹ️  LOW RISK FINDINGS (likely false positives):",
            );

            // Show all LOW risk findings
            for finding in &results.namespace_warnings {
                println!("   - {}", finding.message);
            }

            for finding in &results.crypto_patterns {
                if finding.risk_level == RiskLevel::Low {
                    println!("   - Crypto pattern: {}", finding.message);
                }
            }

            // Show trufflehog activity (environment variable access)
            for finding in &results.trufflehog_activity {
                println!("   - {}", finding.message);
            }

            println!("   \x1b[34mNOTE: These are typically legitimate framework patterns.\x1b[0m");
        }
    }

    // Add verification summary if any findings were verified
    print_verification_summary(results, paranoid_mode);

    print_status(
        Color::Blue,
        "==============================================",
    );
}

/// Print verification summary showing false positive analysis
fn print_verification_summary(results: &ScanResults, _paranoid_mode: bool) {
    // Check if any findings have verification
    let has_verification = results
        .postinstall_hooks
        .iter()
        .any(|f| f.verification.is_some())
        || results
            .suspicious_found
            .iter()
            .any(|f| f.verification.is_some())
        || results
            .crypto_patterns
            .iter()
            .any(|f| f.verification.is_some());

    if !has_verification {
        return; // No verification data, skip summary
    }

    println!();
    print_status(
        Color::Blue,
        "==============================================",
    );
    print_status(Color::Blue, "🔍 VERIFICATION SUMMARY (--verify mode)");
    print_status(
        Color::Blue,
        "==============================================",
    );
    println!();

    // Count verifications by category
    let mut high_total = 0;
    let mut high_verified_safe = 0;
    let mut high_needs_review = 0;

    let mut medium_total = 0;
    let mut medium_verified_safe = 0;
    let mut medium_needs_review = 0;

    // HIGH RISK: Postinstall hooks
    for finding in &results.postinstall_hooks {
        high_total += 1;
        match &finding.verification {
            Some(verification::VerificationStatus::Verified { .. }) => high_verified_safe += 1,
            _ => high_needs_review += 1,
        }
    }

    // MEDIUM RISK: Suspicious packages
    for finding in &results.suspicious_found {
        medium_total += 1;
        match &finding.verification {
            Some(verification::VerificationStatus::Verified { .. }) => medium_verified_safe += 1,
            _ => medium_needs_review += 1,
        }
    }

    // MEDIUM RISK: Crypto patterns
    for finding in &results.crypto_patterns {
        if finding.risk_level == RiskLevel::Medium {
            medium_total += 1;
            match &finding.verification {
                Some(verification::VerificationStatus::Verified { .. }) => {
                    medium_verified_safe += 1
                }
                _ => medium_needs_review += 1,
            }
        }
    }

    // Print HIGH RISK summary
    if high_total > 0 {
        print_status(Color::Red, "🔴 HIGH RISK VERIFICATION:");
        println!("   Total findings: {}", high_total);
        if high_verified_safe > 0 {
            print_status(
                Color::Green,
                &format!(
                    "   ✅ Verified SAFE: {} ({:.0}%)",
                    high_verified_safe,
                    (high_verified_safe as f32 / high_total as f32) * 100.0
                ),
            );
        }
        if high_needs_review > 0 {
            print_status(
                Color::Yellow,
                &format!(
                    "   ⚠️  Needs review: {} ({:.0}%)",
                    high_needs_review,
                    (high_needs_review as f32 / high_total as f32) * 100.0
                ),
            );
        }
        println!();
    }

    // Print MEDIUM RISK summary
    if medium_total > 0 {
        print_status(Color::Yellow, "🟡 MEDIUM RISK VERIFICATION:");
        println!("   Total findings: {}", medium_total);
        if medium_verified_safe > 0 {
            print_status(
                Color::Green,
                &format!(
                    "   ✅ Verified SAFE: {} ({:.0}%)",
                    medium_verified_safe,
                    (medium_verified_safe as f32 / medium_total as f32) * 100.0
                ),
            );
        }
        if medium_needs_review > 0 {
            print_status(
                Color::Yellow,
                &format!(
                    "   ⚠️  Needs review: {} ({:.0}%)",
                    medium_needs_review,
                    (medium_needs_review as f32 / medium_total as f32) * 100.0
                ),
            );
        }
        println!();
    }

    // Overall conclusion
    let total_findings = high_total + medium_total;
    let total_verified = high_verified_safe + medium_verified_safe;
    let false_positive_rate = if total_findings > 0 {
        (total_verified as f32 / total_findings as f32) * 100.0
    } else {
        0.0
    };

    print_status(Color::Blue, "📊 VERIFICATION STATISTICS:");
    println!("   Total critical findings analyzed: {}", total_findings);
    println!(
        "   Verified as false positives: {} ({:.0}%)",
        total_verified, false_positive_rate
    );

    if high_needs_review + medium_needs_review == 0 {
        println!();
        print_status(
            Color::Green,
            "✅ CONCLUSION: All findings verified as SAFE (false positives)",
        );
        print_status(
            Color::Green,
            "   No malicious activity detected. Project appears clean.",
        );
    } else {
        println!();
        print_status(
            Color::Yellow,
            &format!(
                "⚠️  CONCLUSION: {} findings still need manual review",
                high_needs_review + medium_needs_review
            ),
        );
        print_status(
            Color::Yellow,
            "   Review items marked without [VERIFIED SAFE] tags above.",
        );
    }
    println!();
}
