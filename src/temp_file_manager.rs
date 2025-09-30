/// Temporary File Management Module
///
/// # Purpose
/// Provides RAII-based temporary file cleanup that is more robust than bash trap handlers
/// Ensures temp files are cleaned up even on panic or unexpected exit
///
/// # Safety
/// Uses Rust's Drop trait to guarantee cleanup, unlike bash traps which can be bypassed
use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Thread-safe temporary file manager with automatic cleanup
///
/// # Design
/// - Uses RAII (Resource Acquisition Is Initialization) pattern
/// - Automatically cleans up on Drop, even during panics
/// - Thread-safe for concurrent file operations
/// - More robust than bash trap handlers
#[derive(Clone)]
pub struct TempFileManager {
    /// Thread-safe set of temporary files to track
    temp_files: Arc<Mutex<HashSet<PathBuf>>>,
}

impl TempFileManager {
    /// Create a new temporary file manager
    ///
    /// # Returns
    /// * `TempFileManager` - New instance ready for file tracking
    pub fn new() -> Self {
        Self {
            temp_files: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Register a temporary file for cleanup
    ///
    /// # Purpose
    /// Add file to cleanup list - will be automatically removed on Drop
    ///
    /// # Arguments
    /// * `path` - Path to temporary file that should be cleaned up
    ///
    /// # Thread Safety
    /// Safe to call from multiple threads concurrently
    pub fn register_temp_file<P: AsRef<Path>>(&self, path: P) {
        if let Ok(mut files) = self.temp_files.lock() {
            files.insert(path.as_ref().to_path_buf());
        }
    }

    /// Create and register a temporary file with content
    ///
    /// # Purpose
    /// Create temporary file with specified content and register for cleanup
    /// Useful for transforming lockfiles (like pnpm -> package-lock conversion)
    ///
    /// # Arguments
    /// * `prefix` - Filename prefix for temp file
    /// * `content` - Content to write to temporary file
    ///
    /// # Returns
    /// * `Result<PathBuf>` - Path to created temporary file
    ///
    /// # Cleanup
    /// File will be automatically removed when TempFileManager is dropped
    pub fn create_temp_file_with_content(&self, prefix: &str, content: &str) -> Result<PathBuf> {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("{}.{}", prefix, uuid::Uuid::new_v4()));

        fs::write(&temp_path, content)?;
        self.register_temp_file(&temp_path);

        Ok(temp_path)
    }

    /// Manually cleanup specific temporary file
    ///
    /// # Purpose
    /// Remove specific file from filesystem and tracking list
    /// Useful for early cleanup before manager is dropped
    ///
    /// # Arguments
    /// * `path` - Path to temporary file to remove
    ///
    /// # Returns
    /// * `Result<()>` - Success or error information
    pub fn cleanup_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();

        if path.exists() {
            fs::remove_file(path)?;
        }

        if let Ok(mut files) = self.temp_files.lock() {
            files.remove(path);
        }

        Ok(())
    }

    /// Get count of tracked temporary files
    ///
    /// # Purpose
    /// Diagnostic function to check how many temp files are being tracked
    ///
    /// # Returns
    /// * `usize` - Number of temporary files currently tracked
    pub fn temp_file_count(&self) -> usize {
        self.temp_files.lock().map(|files| files.len()).unwrap_or(0)
    }
}

impl Drop for TempFileManager {
    /// Automatic cleanup on Drop (RAII pattern)
    ///
    /// # Purpose
    /// Ensure all temporary files are cleaned up when manager goes out of scope
    /// More reliable than bash trap handlers - works even during panics
    ///
    /// # Behavior
    /// * Removes all tracked temporary files from filesystem
    /// * Logs errors but doesn't panic on cleanup failures
    /// * Always attempts to clean all files, even if some fail
    fn drop(&mut self) {
        if let Ok(files) = self.temp_files.lock() {
            for temp_file in files.iter() {
                if temp_file.exists() {
                    if let Err(e) = fs::remove_file(temp_file) {
                        eprintln!(
                            "Warning: Failed to cleanup temp file {}: {}",
                            temp_file.display(),
                            e
                        );
                    }
                }
            }
        }
    }
}

/// Safe temporary file wrapper with automatic cleanup
///
/// # Purpose
/// Individual temporary file with guaranteed cleanup via RAII
/// Alternative to TempFileManager for single-file use cases
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Create new temporary file with unique name
    ///
    /// # Arguments
    /// * `prefix` - Filename prefix
    /// * `content` - Initial file content
    ///
    /// # Returns
    /// * `Result<TempFile>` - Temporary file wrapper with cleanup guarantee
    pub fn new_with_content(prefix: &str, content: &str) -> Result<Self> {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join(format!("{}.{}", prefix, uuid::Uuid::new_v4()));

        fs::write(&path, content)?;

        Ok(Self { path })
    }

    /// Get path to temporary file
    ///
    /// # Returns
    /// * `&Path` - Reference to temporary file path
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempFile {
    /// Automatic cleanup when TempFile goes out of scope
    fn drop(&mut self) {
        if self.path.exists() {
            if let Err(e) = fs::remove_file(&self.path) {
                eprintln!(
                    "Warning: Failed to cleanup temp file {}: {}",
                    self.path.display(),
                    e
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_file_manager_cleanup() {
        let manager = TempFileManager::new();
        let temp_path = std::env::temp_dir().join("test_temp_file.txt");

        // Create test file
        fs::write(&temp_path, "test content").unwrap();
        assert!(temp_path.exists());

        // Register for cleanup
        manager.register_temp_file(&temp_path);
        assert_eq!(manager.temp_file_count(), 1);

        // Cleanup
        drop(manager);

        // File should be removed
        assert!(!temp_path.exists());
    }

    #[test]
    fn test_temp_file_raii() {
        let temp_path;

        {
            let temp_file = TempFile::new_with_content("test", "content").unwrap();
            temp_path = temp_file.path().to_path_buf();
            assert!(temp_path.exists());
        } // temp_file goes out of scope here

        // File should be automatically cleaned up
        assert!(!temp_path.exists());
    }
}
