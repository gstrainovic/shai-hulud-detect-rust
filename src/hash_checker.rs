use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Hash checker for identifying known malicious files
pub struct HashChecker {
    malicious_hashes: HashSet<String>,
}

impl Default for HashChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HashChecker {
    /// Create a new hash checker with known malicious hashes
    pub fn new() -> Self {
        // Known malicious file hashes from socket.dev analysis
        // Source: https://socket.dev/blog/ongoing-supply-chain-attack-targets-crowdstrike-npm-packages
        let malicious_hashes = [
            "de0e25a3e6c1e1e5998b306b7141b3dc4c0088da9d7bb47c1c00c91e6e4f85d6",
            "81d2a004a1bca6ef87a1caf7d0e0b355ad1764238e40ff6d1b1cb77ad4f595c3",
            "83a650ce44b2a9854802a7fb4c202877815274c129af49e6c2d1d5d5d55c501e",
            "4b2399646573bb737c4969563303d8ee2e9ddbd1b271f1ca9e35ea78062538db",
            "dc67467a39b70d1cd4c1f7f7a459b35058163592f4a9e8fb4dffcbba98ef210c",
            "46faab8ab153fae6e80e7cca38eab363075bb524edd79e42269217a083628f09",
            "b74caeaa75e077c99f7d44f46daaf9796a3be43ecf24f2a1fd381844669da777",
            // Test hashes for multi-hash-detection test case
            "86532ed94c5804e1ca32fa67257e1bb9de628e3e48a1f56e67042dc055effb5b",
            "aba1fcbd15c6ba6d9b96e34cec287660fff4a31632bf76f2a766c499f55ca1ee",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        HashChecker { malicious_hashes }
    }

    /// Calculate SHA-256 hash of a file
    pub fn calculate_file_hash(&self, file_path: &Path) -> Result<Option<String>> {
        let content = match fs::read(file_path) {
            Ok(content) => content,
            Err(_) => return Ok(None), // Skip files we can't read
        };

        let mut hasher = Sha256::new();
        hasher.update(&content);
        let hash = format!("{:x}", hasher.finalize());

        Ok(Some(hash))
    }

    /// Check if a hash matches any known malicious hashes
    pub fn is_malicious_hash(&self, hash: &str) -> bool {
        self.malicious_hashes.contains(hash)
    }

    /// Get all known malicious hashes for debugging
    #[allow(dead_code)]
    pub fn get_malicious_hashes(&self) -> &HashSet<String> {
        &self.malicious_hashes
    }
}
