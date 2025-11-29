#!/bin/bash
# Wrapper for paranoid mode
 exec "$(dirname "${BASH_SOURCE[0]}")/full_sequential_test.sh" --paranoid "$@"
