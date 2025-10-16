// Unit tests for RuntimeResolver
// Tests parsing of pnpm/npm list JSON output

#[cfg(test)]
mod runtime_resolver_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_pnpm_json_parsing() {
        // Sample pnpm list --json output (simplified)
        let json_data = r#"
        [
          {
            "name": "test-project",
            "version": "1.0.0",
            "dependencies": {
              "debug": {
                "from": "debug",
                "version": "4.3.4",
                "dependencies": {
                  "ms": {
                    "from": "ms",
                    "version": "2.1.3",
                    "dependencies": {}
                  }
                }
              },
              "chalk": {
                "from": "chalk",
                "version": "5.0.1",
                "dependencies": {}
              }
            },
            "devDependencies": {
              "jest": {
                "from": "jest",
                "version": "27.0.0",
                "dependencies": {}
              }
            }
          }
        ]
        "#;

        let results: Vec<PnpmListOutput> = serde_json::from_str(json_data).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].dependencies.len(), 2);
        assert_eq!(results[0].dev_dependencies.len(), 1);

        // Check debug version
        let debug = results[0].dependencies.get("debug").unwrap();
        assert_eq!(debug.version, "4.3.4");
        assert_eq!(debug.dependencies.len(), 1);

        // Check nested ms
        let ms = debug.dependencies.get("ms").unwrap();
        assert_eq!(ms.version, "2.1.3");
    }

    #[test]
    fn test_flatten_pnpm_deps() {
        let mut deps = HashMap::new();

        // Create nested structure: debug -> ms
        let mut ms_deps = HashMap::new();
        let ms_pkg = PnpmPackageInfo {
            version: "2.1.3".to_string(),
            dependencies: HashMap::new(),
        };
        ms_deps.insert("ms".to_string(), ms_pkg);

        let debug_pkg = PnpmPackageInfo {
            version: "4.3.4".to_string(),
            dependencies: ms_deps,
        };
        deps.insert("debug".to_string(), debug_pkg);

        // Add chalk at root level
        let chalk_pkg = PnpmPackageInfo {
            version: "5.0.1".to_string(),
            dependencies: HashMap::new(),
        };
        deps.insert("chalk".to_string(), chalk_pkg);

        // Flatten
        let mut output = HashMap::new();
        RuntimeResolver::flatten_pnpm_deps(&deps, &mut output);

        // Verify all packages are flattened
        assert_eq!(output.len(), 3);
        assert_eq!(output.get("debug"), Some(&"4.3.4".to_string()));
        assert_eq!(output.get("ms"), Some(&"2.1.3".to_string()));
        assert_eq!(output.get("chalk"), Some(&"5.0.1".to_string()));
    }

    #[test]
    fn test_flatten_handles_duplicate_versions() {
        // When same package appears multiple times, first version wins
        let mut deps = HashMap::new();

        let debug1 = PnpmPackageInfo {
            version: "4.3.4".to_string(),
            dependencies: HashMap::new(),
        };
        deps.insert("debug".to_string(), debug1);

        let chalk_deps = HashMap::new();
        // Chalk also depends on debug (different version)
        let mut chalk_subdeps = HashMap::new();
        let debug2 = PnpmPackageInfo {
            version: "4.3.5".to_string(),
            dependencies: HashMap::new(),
        };
        chalk_subdeps.insert("debug".to_string(), debug2);

        let chalk = PnpmPackageInfo {
            version: "5.0.1".to_string(),
            dependencies: chalk_subdeps,
        };
        deps.insert("chalk".to_string(), chalk);

        let mut output = HashMap::new();
        RuntimeResolver::flatten_pnpm_deps(&deps, &mut output);

        // First version (4.3.4) should be kept
        assert_eq!(output.get("debug"), Some(&"4.3.4".to_string()));
        assert_eq!(output.get("chalk"), Some(&"5.0.1".to_string()));
    }

    #[test]
    fn test_npm_json_parsing() {
        // Sample npm list --json output
        let json_data = r#"
        {
          "name": "test-project",
          "version": "1.0.0",
          "dependencies": {
            "debug": {
              "version": "4.3.4",
              "dependencies": {
                "ms": {
                  "version": "2.1.3",
                  "dependencies": {}
                }
              }
            }
          }
        }
        "#;

        let result: NpmListOutput = serde_json::from_str(json_data).unwrap();

        assert_eq!(result.dependencies.len(), 1);

        let debug = result.dependencies.get("debug").unwrap();
        assert_eq!(debug.version, "4.3.4");
        assert_eq!(debug.dependencies.len(), 1);

        let ms = debug.dependencies.get("ms").unwrap();
        assert_eq!(ms.version, "2.1.3");
    }

    #[test]
    fn test_get_version() {
        let mut packages = HashMap::new();
        packages.insert("debug".to_string(), "4.3.4".to_string());
        packages.insert("chalk".to_string(), "5.0.1".to_string());

        let resolver = RuntimeResolver { packages };

        assert_eq!(resolver.get_version("debug"), Some("4.3.4"));
        assert_eq!(resolver.get_version("chalk"), Some("5.0.1"));
        assert_eq!(resolver.get_version("unknown"), None);
    }

    #[test]
    fn test_has_packages() {
        let empty_resolver = RuntimeResolver {
            packages: HashMap::new(),
        };
        assert!(!empty_resolver.has_packages());

        let mut packages = HashMap::new();
        packages.insert("debug".to_string(), "4.3.4".to_string());
        let resolver = RuntimeResolver { packages };
        assert!(resolver.has_packages());
    }

    #[test]
    fn test_flatten_deeply_nested() {
        // Test deeply nested dependencies (3 levels)
        let mut level3 = HashMap::new();
        level3.insert(
            "supports-color".to_string(),
            PnpmPackageInfo {
                version: "7.2.0".to_string(),
                dependencies: HashMap::new(),
            },
        );

        let mut level2 = HashMap::new();
        level2.insert(
            "ms".to_string(),
            PnpmPackageInfo {
                version: "2.1.3".to_string(),
                dependencies: level3,
            },
        );

        let mut level1 = HashMap::new();
        level1.insert(
            "debug".to_string(),
            PnpmPackageInfo {
                version: "4.3.4".to_string(),
                dependencies: level2,
            },
        );

        let mut output = HashMap::new();
        RuntimeResolver::flatten_pnpm_deps(&level1, &mut output);

        assert_eq!(output.len(), 3);
        assert_eq!(output.get("debug"), Some(&"4.3.4".to_string()));
        assert_eq!(output.get("ms"), Some(&"2.1.3".to_string()));
        assert_eq!(output.get("supports-color"), Some(&"7.2.0".to_string()));
    }
}
