// Utility functions
// Corresponds to bash utility functions like show_progress, count_files, etc.

use std::path::Path;
use walkdir::WalkDir;

// Function: count_files
// Purpose: Count files matching find criteria, returns clean integer
// Args: path - directory to search, extensions - file extensions to match
// Modifies: None
// Returns: Integer count of matching files
pub fn count_files<P: AsRef<Path>>(path: P, extensions: &[&str]) -> usize {
    WalkDir::new(path)
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
        .count()
}

// Function: count_files_by_name
// Purpose: Count files with specific filename
// Args: path - directory to search, filename - exact filename to match
// Modifies: None
// Returns: Integer count of matching files
pub fn count_files_by_name<P: AsRef<Path>>(path: P, filename: &str) -> usize {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name() == filename)
        .count()
}

// Function: show_progress
// Purpose: Display real-time progress indicator for file scanning operations
// Args: current - current files processed, total - total files to process
// Modifies: None (outputs to stderr with ANSI escape codes)
// Returns: Prints "X / Y checked (Z %)" with line clearing
pub fn show_progress(current: usize, total: usize) {
    let percent = if total > 0 {
        (current * 100) / total
    } else {
        0
    };
    eprint!("\r\x1b[K{} / {} checked ({} %)", current, total, percent);
}

// Function: clear_progress
// Purpose: Clear progress line
// Args: None
// Modifies: None (outputs to stderr)
// Returns: Clears current line
pub fn clear_progress() {
    eprint!("\r\x1b[K");
}

// Function: get_file_context
// Purpose: Classify file context for risk assessment (node_modules, source, build, etc.)
// Args: file_path - path to file
// Modifies: None
// Returns: Context string
#[allow(dead_code)]
pub fn get_file_context(file_path: &Path) -> &'static str {
    let path_str = file_path.to_string_lossy();

    // Check if file is in node_modules
    if path_str.contains("/node_modules/") || path_str.contains("\\node_modules\\") {
        return "node_modules";
    }

    // Check if file is documentation
    if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
        if matches!(ext, "md" | "txt" | "rst") {
            return "documentation";
        }

        // Check if file is TypeScript definitions
        if path_str.ends_with(".d.ts") {
            return "type_definitions";
        }
    }

    // Check if file is in build/dist directories
    if path_str.contains("/dist/")
        || path_str.contains("\\dist\\")
        || path_str.contains("/build/")
        || path_str.contains("\\build\\")
        || path_str.contains("/public/")
        || path_str.contains("\\public\\")
    {
        return "build_output";
    }

    // Check if it's a config file
    if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
        if filename.contains("config") {
            return "configuration";
        }
    }

    "source_code"
}

// Function: is_legitimate_pattern
// Purpose: Identify legitimate framework/build tool patterns to reduce false positives
// Args: file_path - path to file, content_sample - text snippet from file
// Modifies: None
// Returns: true for legitimate, false for potentially suspicious
#[allow(dead_code)]
pub fn is_legitimate_pattern(_file_path: &Path, content_sample: &str) -> bool {
    // Vue.js development patterns
    if content_sample.contains("process.env.NODE_ENV") && content_sample.contains("production") {
        return true; // legitimate
    }

    // Common framework patterns
    if content_sample.contains("createApp") || content_sample.contains("Vue") {
        return true; // legitimate
    }

    // Package manager and build tool patterns
    if content_sample.contains("webpack")
        || content_sample.contains("vite")
        || content_sample.contains("rollup")
    {
        return true; // legitimate
    }

    false // potentially suspicious
}
