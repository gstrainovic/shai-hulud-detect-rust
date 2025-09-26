use crate::pattern_registry::PatternRegistry;
use anyhow::Result;

/// Print the pattern mappings table
pub fn print_pattern_table() -> Result<()> {
    let registry = PatternRegistry::load_from_csv("pattern_mappings.csv")?;

    println!("Pattern Mappings Table:");
    println!("======================");
    println!();
    println!(
        "{:<25} | {:<8} | {:<50} | {:<20}",
        "Pattern Name", "Risk", "Description", "Category"
    );
    println!("{}", "-".repeat(110));

    let mut pattern_names = registry.get_all_pattern_names();
    pattern_names.sort();

    for pattern_name in pattern_names {
        if let Some(info) = registry.get_pattern_info(pattern_name) {
            println!(
                "{:<25} | {:<8} | {:<50} | {:<20}",
                info.pattern_name, info.risk_level, info.description, info.category
            );
        }
    }

    println!();
    println!("Categories:");
    let categories = [
        "Exfiltration",
        "Crypto Theft",
        "Phishing",
        "Credential Harvesting",
        "Crypto Detection",
        "Network Exfiltration",
        "Obfuscation",
    ];
    for category in categories {
        let patterns = registry.get_patterns_by_category(category);
        println!("  {}: {} patterns", category, patterns.len());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_registry() {
        let registry = PatternRegistry::load_from_csv("pattern_mappings.csv").unwrap();

        // Test specific pattern lookup
        assert!(registry.has_pattern("webhook_site_reference"));
        assert!(registry.has_pattern("xmlhttprequest_modification"));

        // Test category filtering
        let crypto_patterns = registry.get_patterns_by_category("Crypto Theft");
        assert!(!crypto_patterns.is_empty());
    }
}
