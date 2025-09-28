#!/usr/bin/env python3
import json
import re
from pathlib import Path

# Script to compare Rust JSON with Bash log for Shai-Hulud detector

# Paths
RUST_JSON = Path(__file__).resolve().parent.parent / 'scan_results.json'
BASH_LOG = Path(__file__).resolve().parent.parent / 'logs' / 'bash' / 'bash-testcase.log'

# Sections we will compare
CHECK_KEYS = [
    'workflow_files', 'malicious_hashes', 'compromised_packages', 'postinstall_hooks',
    'suspicious_content', 'crypto_patterns', 'trufflehog_activity', 'git_branches',
    'shai_hulud_repos', 'package_integrity', 'typosquatting', 'network_exfiltration'
]

POSIX_PATH_RE = re.compile(r'(/[^:]*?\.(?:js|json|sh|yml|yaml|py|lock|txt|md))')
ANSI_RE = re.compile(r'\x1b\[[0-9;]*m')

def normalize_message(message: str) -> str:
    return message

def normalize_report_path(raw: str) -> str:
    return raw

def extract_bash_findings(bash_path: Path):
    # Similar to the original script
    if not bash_path.exists():
        print(f"Warning: bash output {bash_path} not found — returning empty findings")
        return {k: {} for k in CHECK_KEYS}
    raw_text = bash_path.read_text(encoding='utf-8', errors='ignore')
    text = ANSI_RE.sub('', raw_text)
    findings = {k: {} for k in CHECK_KEYS}

    section_map = {
        'Malicious workflow files detected': 'workflow_files',
        'Files with known malicious hashes': 'malicious_hashes',
        'Compromised package versions detected': 'compromised_packages',
        'Suspicious postinstall hooks detected': 'postinstall_hooks',
        'Suspicious content patterns': 'suspicious_content',
        'Cryptocurrency theft patterns detected': 'crypto_patterns',
        'Trufflehog/secret scanning activity detected': 'trufflehog_activity',
        "Suspicious git branches": 'git_branches',
        'Shai-Hulud repositories detected': 'shai_hulud_repos',
        'Package integrity issues detected': 'package_integrity',
        'Potential typosquatting/homoglyph attacks detected': 'typosquatting',
        'Network exfiltration patterns detected': 'network_exfiltration'
    }

    lines = text.splitlines()
    for i, line in enumerate(lines):
        m = POSIX_PATH_RE.search(line)
        if m:
            raw = m.group(1)
            norm = normalize_report_path(raw)
            sect = None
            for j in range(max(0, i-8), i+1):
                l = lines[j]
                for header, key in section_map.items():
                    if header in l:
                        sect = key
                        break
                if sect:
                    break
            if not sect:
                if 'postinstall' in line.lower() or 'postinstall' in ''.join(lines[max(0, i-2):i+3]).lower():
                    sect = 'postinstall_hooks'
            if not sect:
                sect = 'suspicious_content'
            findings.setdefault(sect, {}).setdefault(norm, [])
            msg = ANSI_RE.sub('', line).strip()
            if msg not in findings[sect][norm]:
                findings[sect][norm].append(msg)
    return findings

def load_rust_json(path: Path):
    if not path.exists():
        print(f"Warning: rust JSON {path} not found — returning empty normalized map")
        return {k: {} for k in CHECK_KEYS}
    data = json.loads(path.read_text(encoding='utf-8'))
    findings = {k: {} for k in CHECK_KEYS}
    results = data.get('results', [])
    for result in results:
        file_path = result.get('file', '')
        comment = result.get('comment', '')
        patterns = result.get('patterns_detected', [])
        norm = normalize_report_path(file_path)
        # Determine section based on comment or patterns
        sect = None
        if 'integrity' in comment.lower() or 'package_integrity_issue' in patterns:
            sect = 'package_integrity'
        elif 'Suspicious package version' in comment or 'compromised' in comment.lower():
            sect = 'compromised_packages'
        elif 'crypto' in comment.lower() or 'cryptocurrency' in comment.lower() or 'xmlhttprequest' in comment.lower() or 'wallet address' in comment.lower() or 'phishing domain' in comment.lower():
            sect = 'crypto_patterns'
        elif 'postinstall' in comment.lower():
            sect = 'postinstall_hooks'
        elif 'workflow' in comment.lower() or 'malicious workflow' in comment.lower() or ('found in' in comment.lower() and 'package.json' in file_path):
            sect = 'workflow_files'
        elif 'hash' in comment.lower() or 'malicious hash' in comment.lower():
            sect = 'malicious_hashes'
        elif 'typosquatting' in comment.lower():
            sect = 'typosquatting'
        elif 'network' in comment.lower() or 'exfiltration' in comment.lower():
            sect = 'network_exfiltration'
        elif 'trufflehog' in comment.lower():
            sect = 'trufflehog_activity'
        elif 'git' in comment.lower() or 'branch' in comment.lower():
            sect = 'git_branches'
        elif 'shai-hulud' in comment.lower():
            sect = 'shai_hulud_repos'
        elif 'found in' in comment.lower():
            sect = 'suspicious_content'
        else:
            sect = 'suspicious_content'
        if sect == 'workflow_files':
            print(f"Workflow file: {file_path}, {comment}, sect: {sect}")
        findings.setdefault(sect, {}).setdefault(norm, [])
        msg = comment
        msg = normalize_message(msg)
        if msg not in findings[sect][norm]:
            findings[sect][norm].append(msg)
    return findings

def compare_findings(bash_findings, rust_findings):
    report = {}
    for key in CHECK_KEYS:
        bash_map = bash_findings.get(key, {})
        rust_map = rust_findings.get(key, {})
        all_paths = set(bash_map.keys()) | set(rust_map.keys())
        diffs = []
        for p in sorted(all_paths):
            b_orig = bash_map.get(key, {}).get(p, [])
            r_orig = rust_map.get(key, {}).get(p, [])
            b = [normalize_message(msg) for msg in b_orig]
            r = [normalize_message(msg) for msg in r_orig]
            if b != r:
                diffs.append({'path': p, 'bash': b, 'rust': r})
        report[key] = {
            'bash_count': sum(len(v) for v in bash_map.values()),
            'rust_count': sum(len(v) for v in rust_map.values()),
            'diffs': diffs
        }
    return report

def summarize_comparison(report):
    summary = {
        'bash_only': {},
        'rust_only': {},
        'differences': {},
        'total_bash': 0,
        'total_rust': 0
    }
    for key, data in report.items():
        bash_only = []
        rust_only = []
        diffs = []
        for diff in data['diffs']:
            p = diff['path']
            b = diff['bash']
            r = diff['rust']
            if b and not r:
                bash_only.append({'path': p, 'details': b})
            elif r and not b:
                rust_only.append({'path': p, 'details': r})
            else:
                diffs.append(diff)
        summary['bash_only'][key] = bash_only
        summary['rust_only'][key] = rust_only
        summary['differences'][key] = diffs
        summary['total_bash'] += data['bash_count']
        summary['total_rust'] += data['rust_count']
    return summary

if __name__ == '__main__':
    bash_findings = extract_bash_findings(BASH_LOG)
    rust_findings = load_rust_json(RUST_JSON)
    report = compare_findings(bash_findings, rust_findings)
    summary = summarize_comparison(report)

    print('=== RUST JSON vs BASH LOG COMPARISON SUMMARY ===')
    print(f"Total findings - Bash: {summary['total_bash']}, Rust: {summary['total_rust']}")
    print("\n--- Was ist in Bash vorhanden und fehlt in Rust? (Bash-only) ---")
    for key, items in summary['bash_only'].items():
        if items:
            print(f"{key}: {len(items)} items")
            for item in items[:5]:  # Show first 5
                print(f"  - {item['path']}: {item['details']}")
            if len(items) > 5:
                print(f"  ... and {len(items)-5} more")
    print("\n--- Was hat Rust, was Bash nicht hat? (Rust-only) ---")
    for key, items in summary['rust_only'].items():
        if items:
            print(f"{key}: {len(items)} items")
            for item in items[:5]:
                print(f"  - {item['path']}: {item['details']}")
            if len(items) > 5:
                print(f"  ... and {len(items)-5} more")
    print("\n--- Unterschiede (Pseudofehler / Differences) ---")
    for key, diffs in summary['differences'].items():
        if diffs:
            print(f"{key}: {len(diffs)} diffs")
            for diff in diffs[:3]:
                print(f"  - {diff['path']}: Bash={diff['bash']}, Rust={diff['rust']}")
            if len(diffs) > 3:
                print(f"  ... and {len(diffs)-3} more")
    print("\n--- Patterns Rust vs Bash ---")
    for key in CHECK_KEYS:
        bash_c = report[key]['bash_count']
        rust_c = report[key]['rust_count']
        print(f"{key}: Bash={bash_c}, Rust={rust_c}")

    # Save full report
    output_path = Path(__file__).resolve().parent / 'json_vs_log_comparison.json'
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump({'report': report, 'summary': summary}, f, indent=2, ensure_ascii=False)
    print(f"\nFull comparison saved to {output_path}")