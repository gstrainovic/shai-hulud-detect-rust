#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Pattern-level verification tool
Compares Bash scanner .log output with Rust scanner JSON output
"""

import json
import re
import sys
import io
from pathlib import Path
from collections import defaultdict
from typing import List, Dict, Set, Tuple

# Fix Windows console encoding
if sys.platform == 'win32':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
    sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')


class Finding:
    """Represents a single finding"""
    def __init__(self, file_path: str, message: str, risk_level: str, category: str):
        self.file_path = self.normalize_path(file_path)
        self.message = message.strip()
        self.risk_level = risk_level.upper()
        self.category = self.normalize_category(category)
    
    @staticmethod
    def normalize_category(category: str) -> str:
        """Normalize category names for comparison"""
        # Map similar categories together
        category_map = {
            'crypto_theft': 'crypto_patterns',  # Bash uses crypto_theft, Rust uses crypto_patterns
            'crypto_attacker_wallet': 'crypto_patterns',
            'crypto_xhr_hijack': 'crypto_patterns',
            'crypto_wallet_pattern': 'crypto_patterns',
            'crypto_phishing': 'crypto_patterns',
            'trufflehog_binary': 'trufflehog',
            'trufflehog_reference': 'trufflehog',
            'credential_patterns': 'trufflehog',
            'credential_exfiltration': 'trufflehog',
            'env_suspicious': 'trufflehog',
        }
        return category_map.get(category, category)
    
    @staticmethod
    def normalize_path(path: str) -> str:
        """Normalize path for comparison"""
        # Remove Windows UNC prefix
        path = path.replace("\\\\?\\", "")
        # Convert backslashes to forward slashes
        path = path.replace("\\", "/")
        # Lowercase for case-insensitive comparison
        path = path.lower()
        # Remove c:/ or /c/ or c/ prefix variations (any combination)
        path = re.sub(r'^/?c:?/?', '', path, flags=re.IGNORECASE)
        return path
    
    def fingerprint(self) -> str:
        """Create unique fingerprint for comparison (without category)"""
        # Don't include category in fingerprint - categories may differ between implementations
        return f"{self.file_path}|{self.message.lower()}|{self.risk_level}"
    
    def __hash__(self):
        return hash(self.fingerprint())
    
    def __eq__(self, other):
        return self.fingerprint() == other.fingerprint()
    
    def __repr__(self):
        return f"Finding({self.risk_level}: {self.file_path} - {self.message[:50]}...)"


def parse_bash_log(log_file: Path) -> List[Finding]:
    """Parse Bash scanner .log file and extract findings"""
    findings = []
    
    with open(log_file, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    # Remove ANSI escape codes
    ansi_escape = re.compile(r'\x1B(?:[@-Z\\-_]|\[[0-9;]*[ -/]*[@-~])')
    content = ansi_escape.sub('', content)
    
    current_risk = None
    current_category = None
    
    lines = content.split('\n')
    i = 0
    
    while i < len(lines):
        line = lines[i].strip()
        
        # Detect risk level sections
        if '🚨 HIGH RISK:' in line:
            current_risk = 'HIGH'
            # Extract category from section header
            if 'Malicious workflow files' in line:
                current_category = 'workflow'
            elif 'Compromised package' in line:
                current_category = 'compromised_package'
            elif 'Cryptocurrency theft' in line:
                current_category = 'crypto_theft'
            elif 'Trufflehog' in line or 'secret scanning' in line:
                current_category = 'trufflehog'
            else:
                current_category = 'unknown'
        
        elif '⚠️  MEDIUM RISK:' in line or '⚠️ MEDIUM RISK:' in line:
            current_risk = 'MEDIUM'
            if 'Suspicious content' in line:
                current_category = 'suspicious_content'
            elif 'cryptocurrency manipulation' in line:
                current_category = 'crypto_patterns'
            elif 'secret scanning' in line:
                current_category = 'trufflehog'
            else:
                current_category = 'unknown'
        
        elif 'ℹ️  LOW RISK:' in line or 'ℹ️ LOW RISK:' in line:
            current_risk = 'LOW'
            if 'namespace' in line.lower():
                current_category = 'namespace_warning'
            else:
                current_category = 'unknown'
        
        # Parse findings - Pattern 1: Simple file path (for workflow files - MUST BE FIRST!)
        # Workflows are just "- /path/to/file" without colon or message
        if line.strip().startswith('- /') and current_risk and current_category == 'workflow' and ':' not in line:
            file_path = line.strip().replace('- ', '')
            findings.append(Finding(file_path, 'Known malicious workflow filename', current_risk, current_category))
        
        # Parse findings - Pattern 2: "- Pattern: XXX\n     Found in: YYY"
        elif line.startswith('- Pattern:') and current_risk:
            pattern = line.replace('- Pattern:', '').strip()
            # Look ahead for "Found in:" line
            if i + 1 < len(lines) and 'Found in:' in lines[i + 1]:
                file_path = lines[i + 1].replace('Found in:', '').strip()
                findings.append(Finding(file_path, pattern, current_risk, current_category))
                i += 1  # Skip next line
        
        # Parse findings - Pattern 3: "- Package: XXX\n     Found in: YYY"
        elif line.startswith('- Package:') and current_risk:
            package = line.replace('- Package:', '').strip()
            # Look ahead for "Found in:" line
            if i + 1 < len(lines) and 'Found in:' in lines[i + 1]:
                file_path = lines[i + 1].replace('Found in:', '').strip()
                findings.append(Finding(file_path, package, current_risk, current_category))
                i += 1  # Skip next line
        
        # Parse findings - Pattern 4: "- Activity: XXX\n     Found in: YYY"
        elif line.startswith('- Activity:') and current_risk:
            activity = line.replace('- Activity:', '').strip()
            # Look ahead for "Found in:" line
            if i + 1 < len(lines) and 'Found in:' in lines[i + 1]:
                file_path = lines[i + 1].replace('Found in:', '').strip()
                findings.append(Finding(file_path, activity, current_risk, current_category))
                i += 1  # Skip next line
        
        # Parse findings - Pattern 5: "- /path/to/file:message"
        elif line.startswith('- /') or line.startswith('- C:/') or line.startswith('- c:/'):
            # Extract file path and message
            match = re.match(r'-\s+(.+?):(.*)', line)
            if match and current_risk:
                file_path = match.group(1).strip()
                message = match.group(2).strip()
                findings.append(Finding(file_path, message, current_risk, current_category))
        
        i += 1
    
    return findings


def load_rust_json(json_file: Path) -> List[Finding]:
    """Load Rust scanner JSON and convert to Finding objects"""
    with open(json_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    findings = []
    
    # Map category names
    category_map = {
        'workflow_files': 'workflow',
        'malicious_hashes': 'malicious_hash',
        'compromised_found': 'compromised_package',
        'suspicious_found': 'suspicious_package',
        'lockfile_safe_versions': 'lockfile_safe',
        'suspicious_content': 'suspicious_content',
        'crypto_patterns': 'crypto_patterns',
        'git_branches': 'git_branch',
        'postinstall_hooks': 'postinstall',
        'trufflehog_activity': 'trufflehog',
        'shai_hulud_repos': 'shai_hulud_repo',
        'namespace_warnings': 'namespace_warning',
        'integrity_issues': 'integrity',
        'typosquatting_warnings': 'typosquatting',
        'network_exfiltration_warnings': 'network_exfiltration'
    }
    
    for category_key, category_name in category_map.items():
        if category_key in data:
            for item in data[category_key]:
                findings.append(Finding(
                    item['file_path'],
                    item['message'],
                    item['risk_level'],
                    category_name
                ))
    
    return findings


def compare_findings(bash_findings: List[Finding], rust_findings: List[Finding]) -> Tuple[Set[Finding], Set[Finding], Set[Finding]]:
    """Compare findings and return (matches, missing_in_rust, extra_in_rust)"""
    bash_set = set(bash_findings)
    rust_set = set(rust_findings)
    
    matches = bash_set & rust_set
    missing_in_rust = bash_set - rust_set
    extra_in_rust = rust_set - bash_set
    
    return matches, missing_in_rust, extra_in_rust


def print_findings_by_risk(findings: Set[Finding], title: str):
    """Print findings grouped by risk level"""
    by_risk = defaultdict(list)
    for f in findings:
        by_risk[f.risk_level].append(f)
    
    count = len(findings)
    print(f"\n{title} ({count} findings):")
    print("   (found in Bash but NOT in Rust)" if "Missing" in title else "   (found in Rust but NOT in Bash)")
    print()
    
    for risk in ['HIGH', 'MEDIUM', 'LOW']:
        if risk in by_risk:
            items = sorted(by_risk[risk], key=lambda x: (x.file_path, x.message))
            print(f"   {risk} RISK ({len(items)} findings):")
            for f in items:
                print(f"   📄 {f.file_path}: {f.message}")
                print(f"      Category: {f.category}")
                print()


def main():
    if len(sys.argv) != 3:
        print("Usage: python verify_pattern_match.py <bash.log> <rust.json>")
        print()
        print("Example:")
        print("  python verify_pattern_match.py bash_infected.log rust_infected.json")
        sys.exit(1)
    
    bash_log_path = Path(sys.argv[1])
    rust_json_path = Path(sys.argv[2])
    
    if not bash_log_path.exists():
        print(f"❌ Error: {bash_log_path} not found")
        sys.exit(1)
    
    if not rust_json_path.exists():
        print(f"❌ Error: {rust_json_path} not found")
        sys.exit(1)
    
    print("=" * 80)
    print("🔍 PATTERN-LEVEL VERIFICATION")
    print("=" * 80)
    print()
    print(f"📄 Bash results: {bash_log_path}")
    print(f"📄 Rust results: {rust_json_path}")
    print()
    
    # Parse files
    print("🔍 Parsing Bash .log file...")
    bash_findings = parse_bash_log(bash_log_path)
    
    print("🔍 Parsing Rust .json file...")
    rust_findings = load_rust_json(rust_json_path)
    
    print()
    print("📊 Findings Summary:")
    print(f"   Bash scanner: {len(bash_findings)} findings")
    print(f"   Rust scanner: {len(rust_findings)} findings")
    print()
    
    # Compare
    matches, missing_in_rust, extra_in_rust = compare_findings(bash_findings, rust_findings)
    
    # Check if only difference is LOW RISK namespace warnings (expected - Bash doesn't show these)
    only_low_risk_namespace = (
        len(missing_in_rust) == 0 and
        all(f.risk_level == 'LOW' and f.category == 'namespace_warning' for f in extra_in_rust)
    )
    
    perfect_match = len(missing_in_rust) == 0 and len(extra_in_rust) == 0
    acceptable_match = only_low_risk_namespace
    
    if perfect_match or acceptable_match:
        # SUCCESS
        print("=" * 80)
        print("✅ PERFECT MATCH!")
        print("=" * 80)
        print()
        print(f"   ✓ All {len(bash_findings)} HIGH/MEDIUM findings matched exactly")
        print("   ✓ No missing patterns")
        
        if acceptable_match:
            print(f"   ℹ️  {len(extra_in_rust)} LOW RISK namespace warnings (Rust-only, expected)")
            print("      Bash scanner doesn't show LOW RISK findings individually")
        else:
            print("   ✓ No extra patterns")
        
        print()
        
        # Breakdown by risk
        by_risk = defaultdict(int)
        for f in bash_findings:
            by_risk[f.risk_level] += 1
        
        # Add Rust-only LOW RISK
        for f in extra_in_rust:
            if f.risk_level == 'LOW':
                by_risk['LOW'] += 1
        
        print("📈 Breakdown by Risk Level:")
        for risk in ['HIGH', 'MEDIUM', 'LOW']:
            if by_risk[risk] > 0:
                print(f"   {risk}: {by_risk[risk]} findings ✅")
        
        print()
        print("=" * 80)
        sys.exit(0)
    else:
        # MISMATCH
        print("=" * 80)
        print("❌ MISMATCH DETECTED")
        print("=" * 80)
        
        if missing_in_rust:
            print_findings_by_risk(missing_in_rust, "🔴 Missing in Rust")
        
        if extra_in_rust:
            print_findings_by_risk(extra_in_rust, "🟡 Extra in Rust")
        
        print("=" * 80)
        print("⚠️  Scanners produced different results!")
        print("    Review findings above for discrepancies.")
        print("=" * 80)
        sys.exit(1)


if __name__ == "__main__":
    main()
