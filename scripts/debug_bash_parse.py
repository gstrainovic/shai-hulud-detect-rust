#!/usr/bin/env python3
# Debug bash log parsing

import sys
sys.path.insert(0, 'scripts')

from verify_pattern_match import parse_bash_log
from pathlib import Path

bash_log = Path("scripts/analyze/per-testcase-logs/20251008_234043/bash_infected-project.log")
findings = parse_bash_log(bash_log)

print(f"Total findings: {len(findings)}\n")

# Show workflow findings
workflow_findings = [f for f in findings if f.category == 'workflow']
print(f"Workflow findings: {len(workflow_findings)}")
for f in workflow_findings:
    print(f"  {f.file_path}")
    print(f"  Message: {f.message}")
    print(f"  Fingerprint: {f.fingerprint()}")
    print()
