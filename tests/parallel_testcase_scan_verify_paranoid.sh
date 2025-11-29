#!/bin/bash
# Wrapper for paranoid + verify mode - calls main script with both flags
exec "$(dirname "${BASH_SOURCE[0]}")/parallel_testcase_scan.sh" --paranoid --verify "$@"
