#!/bin/bash
# Wrapper for paranoid mode - calls main script with --paranoid flag
exec "$(dirname "${BASH_SOURCE[0]}")/parallel_testcase_scan.sh" --paranoid "$@"
