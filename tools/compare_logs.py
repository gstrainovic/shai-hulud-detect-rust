#!/usr/bin/env python3
import json
import re
from pathlib import Path

# Script to compare Rust JSON with Bash log for Shai-Hulud detector

# Paths
RUST_JSON = Path(__file__).resolve().parent.parent / 'logs' /  'rust' / 'test-cases' / 'scan_results.json'
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
    msg = message.lower()
    msg = msg.replace("high risk: ", "")
    msg = msg.replace("malicious workflow file detected: ", "")
    msg = msg.replace("suspicious patterns detected: ", "")
    msg = msg.replace("ethereum wallet address patterns detected", "ethereum wallet address patterns detected")
    # Add more normalizations as needed
    return msg.strip()

def normalize_report_path(raw: str) -> str:
    return raw

def extract_bash_findings(bash_path: Path):
    if not bash_path.exists():
        print(f"Warning: bash output {bash_path} not found — returning empty findings")
        return {k: {} for k in CHECK_KEYS}
    raw_text = bash_path.read_text(encoding='utf-8', errors='ignore')
    text = ANSI_RE.sub('', raw_text)
    findings = {k: {} for k in CHECK_KEYS}

    lines = text.splitlines()
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("- "):
            content = stripped[2:]
            if ":" in content:
                raw, comment_part = content.split(":", 1)
                comment = comment_part.strip()
            else:
                raw = content
                comment = ""
            if raw.startswith("/"):
                norm = normalize_report_path(raw)
                # If comment not set, find │ line
                if not comment:
                    for j in range(i + 1, min(i + 10, len(lines))):
                        if "│  Context:" in lines[j]:
                            comment = lines[j].split("│  Context:")[1].strip()
                            break
                sect = None
                if 'workflow' in comment.lower() or 'malicious workflow' in comment.lower():
                    sect = 'workflow_files'
                elif 'postinstall' in comment.lower():
                    sect = 'postinstall_hooks'
                elif 'compromised' in comment.lower() or 'integrity' in comment.lower():
                    sect = 'package_integrity'
                elif 'crypto' in comment.lower() or 'wallet' in comment.lower() or 'phishing' in comment.lower() or 'xmlhttprequest' in comment.lower():
                    sect = 'crypto_patterns'
                elif 'trufflehog' in comment.lower():
                    sect = 'trufflehog_activity'
                elif 'git' in comment.lower() or 'branch' in comment.lower():
                    sect = 'git_branches'
                elif 'shai-hulud' in comment.lower():
                    sect = 'shai_hulud_repos'
                elif 'typosquatting' in comment.lower():
                    sect = 'typosquatting'
                elif 'network' in comment.lower() or 'exfiltration' in comment.lower() or 'webhook' in comment.lower():
                    sect = 'network_exfiltration'
                elif 'hash' in comment.lower():
                    sect = 'malicious_hashes'
                else:
                    sect = 'suspicious_content'
                if sect:
                    findings.setdefault(sect, {}).setdefault(norm, [])
                    msg = comment
                    if msg and msg not in findings[sect][norm]:
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
            b_orig = bash_map.get(p, [])
            r_orig = rust_map.get(p, [])
            b = [normalize_message(msg) for msg in b_orig]
            r = [normalize_message(msg) for msg in r_orig]
            diffs.append({'path': p, 'bash': b, 'rust': r})
        report[key] = {
            'bash_count': sum(len(v) for v in bash_map.values()),
            'rust_count': sum(len(v) for v in rust_map.values()),
            'diffs': diffs
        }
    return report

def summarize_comparison(report):
    summary = {
        'matches': {},
        'bash_only': {},
        'rust_only': {},
        'differences': {},
        'total_bash': 0,
        'total_rust': 0
    }
    for key, data in report.items():
        matches = []
        bash_only = []
        rust_only = []
        diffs = []
        for diff in data['diffs']:
            p = diff['path']
            b = diff['bash']
            r = diff['rust']
            if b == r and b:
                matches.append({'path': p, 'details': b})
            elif b and not r:
                bash_only.append({'path': p, 'details': b})
            elif r and not b:
                rust_only.append({'path': p, 'details': r})
            else:
                diffs.append(diff)
        summary['matches'][key] = matches
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
    print("\n--- Gemeinsamkeiten ---")
    for key, items in summary['matches'].items():
        if items:
            print(f"{key}: {len(items)} matches")
            for item in items[:3]:  # Show first 3
                print(f"  - {item['path']}: {item['details']}")
            if len(items) > 3:
                print(f"  ... and {len(items)-3} more")
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