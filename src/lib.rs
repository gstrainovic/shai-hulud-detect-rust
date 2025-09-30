pub mod e2e_tests;
pub mod hash_checker;
pub mod output;
pub mod pattern_registry;
pub mod pattern_table;
pub mod patterns;
pub mod scanner;
pub mod temp_file_manager;

// Re-export commonly used types
pub use e2e_tests::E2ETestRunner;
pub use output::{FileResult, ScanResults};
pub use patterns::RiskLevel;
pub use scanner::Scanner;
pub use temp_file_manager::{TempFile, TempFileManager};
