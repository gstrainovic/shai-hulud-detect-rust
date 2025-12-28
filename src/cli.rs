// CLI argument parsing
// Corresponds to bash argument parsing in main()

use anyhow::{bail, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "shai-hulud-detector")]
#[command(version = "3.0.5")]
#[command(about = "Shai-Hulud NPM Supply Chain Attack Detector", long_about = None)]
pub struct Cli {
    /// Directory to scan for indicators of compromise
    pub scan_dir: PathBuf,

    /// Enable additional security checks (typosquatting, network patterns)
    /// These are general security features, not specific to Shai-Hulud
    #[arg(long)]
    pub paranoid: bool,

    /// Enable intelligent verification to reduce false positives
    /// Checks lockfiles and code patterns to identify legitimate packages
    #[arg(long)]
    pub verify: bool,

    /// Set the number of threads to use for parallelized steps
    #[arg(long, default_value = "4")]
    pub parallelism: usize,

    /// Check if package.json semver ranges (^, ~) could resolve to
    /// compromised versions. Reports LOW risk (informational) since
    /// packages are largely unpublished from npm.
    #[arg(long)]
    pub check_semver_ranges: bool,

    /// Save all detected file paths to FILE, grouped by severity.
    /// Output format: # HIGH / # MEDIUM / # LOW headers with file paths
    #[arg(long)]
    pub save_log: Option<PathBuf>,
}

impl Cli {
    // Function: validate
    // Purpose: Validate CLI arguments before processing
    // Args: self
    // Returns: Result indicating if arguments are valid
    pub fn validate(&mut self) -> Result<()> {
        if !self.scan_dir.exists() {
            bail!(
                "Error: Directory '{}' does not exist.",
                self.scan_dir.display()
            );
        }

        if !self.scan_dir.is_dir() {
            bail!("Error: '{}' is not a directory.", self.scan_dir.display());
        }

        // Convert to absolute path for bash-identical output
        self.scan_dir = self.scan_dir.canonicalize()?;

        Ok(())
    }
}
