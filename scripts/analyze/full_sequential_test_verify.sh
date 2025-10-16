#!/bin/bash
# Wrapper for verify mode (normal + --verify) - calls main script with --verify flag
exec "$(dirname "${BASH_SOURCE[0]}")/full_sequential_test.sh" --verify "$@"
