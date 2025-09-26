use anyhow::Result;
use std::collections::HashMap;
use std::fs;

/// Pattern information loaded from CSV
#[derive(Debug, Clone)]
pub struct PatternInfo {
    pub pattern_name: String,
    pub risk_level: String,
    pub description: String,
    pub category: String,
    pub notes: String,
}

/// Pattern registry that loads from CSV file
pub struct PatternRegistry {
    patterns: HashMap<String, PatternInfo>,
}

impl PatternRegistry {
    /// Load pattern mappings from CSV file
    pub fn load_from_csv(csv_path: &str) -> Result<Self> {
        let content = fs::read_to_string(csv_path)?;
        let mut patterns = HashMap::new();

        for (i, line) in content.lines().enumerate() {
            if i == 0 {
                continue; // Skip header
            }

            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 5 {
                let pattern_info = PatternInfo {
                    pattern_name: parts[0].to_string(),
                    risk_level: parts[1].to_string(),
                    description: parts[2].to_string(),
                    category: parts[3].to_string(),
                    notes: parts[4].to_string(),
                };

                patterns.insert(parts[0].to_string(), pattern_info);
            }
        }

        Ok(PatternRegistry { patterns })
    }

    /// Get pattern information by name
    pub fn get_pattern_info(&self, pattern_name: &str) -> Option<&PatternInfo> {
        self.patterns.get(pattern_name)
    }

    /// Get all patterns in a category
    pub fn get_patterns_by_category(&self, category: &str) -> Vec<&PatternInfo> {
        self.patterns
            .values()
            .filter(|p| p.category == category)
            .collect()
    }

    /// Get all available pattern names
    pub fn get_all_pattern_names(&self) -> Vec<&String> {
        self.patterns.keys().collect()
    }

    /// Check if a pattern exists
    pub fn has_pattern(&self, pattern_name: &str) -> bool {
        self.patterns.contains_key(pattern_name)
    }
}
