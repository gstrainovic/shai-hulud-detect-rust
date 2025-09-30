# Shai-Hulud Repository Detection Test

This test case simulates a repository with Shai-Hulud-related naming patterns and migration indicators.

## Test Purpose
- Tests detection of repositories with "shai-hulud" in the name
- Tests detection of migration patterns ("-migration" repos)
- Tests Git remote URL scanning for shai-hulud references

## Expected Detection
- Repository name contains migration pattern
- Git remotes contain 'shai-hulud' and 'Shai-Hulud' references
- Should be classified as MEDIUM RISK