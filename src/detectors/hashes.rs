// File Hashes Detector
// Rust port of: check_file_hashes()

use crate::detectors::{Finding, RiskLevel};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Function: check_file_hashes
// Purpose: Scan files and compare SHA256 hashes against known malicious hash list
// Args: $1 = scan_dir (directory to scan)
// Modifies: MALICIOUS_HASHES (global array)
// Returns: Populates MALICIOUS_HASHES array with "file:hash" entries for matches
pub fn check_file_hashes<P: AsRef<Path>>(
    scan_dir: P,
    malicious_hashes: &HashSet<String>,
    parallelism: usize,
) -> Vec<Finding> {
    let scan_dir = scan_dir.as_ref();
    let extensions = &["js", "ts", "json"];

    let files_count = crate::utils::count_files(scan_dir, extensions);

    crate::colors::print_status(
        crate::colors::Color::Blue,
        &format!(
            "Checking {} files for known malicious content...",
            files_count
        ),
    );

    // Collect all files to process
    let files: Vec<_> = WalkDir::new(scan_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| extensions.contains(&ext))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    // Configure rayon thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(parallelism)
        .build()
        .unwrap();

    // Process files in parallel
    let findings: Vec<Finding> = pool.install(|| {
        files
            .par_iter()
            .enumerate()
            .filter_map(|(idx, path)| {
                // Show progress (not thread-safe, but close enough for user feedback)
                if idx % 100 == 0 {
                    crate::utils::show_progress(idx, files_count);
                }

                if let Ok(content) = fs::read(path) {
                    let hash = format!("{:x}", Sha256::digest(&content));

                    // Check for malicious files
                    if malicious_hashes.contains(&hash) {
                        return Some(Finding::new(
                            path.clone(),
                            format!("Hash: {}", hash),
                            RiskLevel::High,
                            "malicious_hash",
                        ));
                    }
                }
                None
            })
            .collect()
    });

    crate::utils::clear_progress();

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_known_malicious_hash() {
        let temp = TempDir::new().unwrap();

        // Create a file with a known malicious content
        // This is the actual malicious content that produces the first hash in the list
        let malicious_file = temp.path().join("bundle.js");
        fs::write(&malicious_file, "test content").unwrap();

        let mut hashes = HashSet::new();
        // Add the hash of "test content"
        let test_hash = "6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72";
        hashes.insert(test_hash.to_string());

        let findings = check_file_hashes(temp.path(), &hashes, 1);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].risk_level, RiskLevel::High);
    }
}
