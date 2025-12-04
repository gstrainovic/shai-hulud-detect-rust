#!/bin/bash
# scan_workspace.sh - Scan all folders in workspace with Rust scanner
# Excludes: rust-scanner folder itself

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCANNER="$SCRIPT_DIR/target/release/shai-hulud-detector"
WORKSPACE="/c/Users/gstra/Code"
LOG_DIR="$SCRIPT_DIR/workspace-scan-logs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Create log directory
mkdir -p "$LOG_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 WORKSPACE SECURITY SCAN"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Started: $(date)"
echo "📁 Workspace: $WORKSPACE"
echo "📂 Logs: $LOG_DIR"
echo ""

# Build scanner if needed
if [ ! -f "$SCANNER" ]; then
    echo "🔨 Building scanner..."
    cd "$SCRIPT_DIR"
    cargo build --release
fi

# Get all directories (excluding rust-scanner and hidden folders)
FOLDERS=$(find "$WORKSPACE" -maxdepth 1 -type d ! -name "rust-scanner" ! -name ".*" ! -name "_*" | tail -n +2 | sort)

echo "📋 Folders to scan:"
echo "$FOLDERS" | while read -r folder; do
    echo "   - $(basename "$folder")"
done
echo ""

# Scan each folder
total=0
high_risk=0
medium_risk=0
clean=0

for folder in $FOLDERS; do
    folder_name=$(basename "$folder")
    log_file="$LOG_DIR/${folder_name}_${TIMESTAMP}.log"
    
    echo -n "🔍 Scanning: $folder_name ... "
    
    # Run scanner and capture output
    if "$SCANNER" "$folder" > "$log_file" 2>&1; then
        exit_code=$?
    else
        exit_code=$?
    fi
    
    # Extract counts from log
    h=$(grep -o "High Risk Issues: [0-9]*" "$log_file" | grep -o "[0-9]*" || echo "0")
    m=$(grep -o "Medium Risk Issues: [0-9]*" "$log_file" | grep -o "[0-9]*" || echo "0")
    l=$(grep -o "Low Risk Issues: [0-9]*" "$log_file" | grep -o "[0-9]*" || echo "0")
    
    # Default to 0 if empty
    h=${h:-0}
    m=${m:-0}
    l=${l:-0}
    
    total=$((total + 1))
    
    if [ "$h" -gt 0 ]; then
        echo "🚨 HIGH RISK ($h/$m/$l)"
        high_risk=$((high_risk + 1))
    elif [ "$m" -gt 0 ]; then
        echo "⚠️  MEDIUM ($h/$m/$l)"
        medium_risk=$((medium_risk + 1))
    else
        echo "✅ CLEAN"
        clean=$((clean + 1))
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   Total Scanned: $total"
echo "   🚨 High Risk:  $high_risk"
echo "   ⚠️  Medium:     $medium_risk"
echo "   ✅ Clean:       $clean"
echo ""
echo "⏱️  Finished: $(date)"
echo "💾 Logs saved: $LOG_DIR"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$high_risk" -gt 0 ]; then
    echo ""
    echo "⚠️  WARNING: $high_risk folder(s) have HIGH RISK findings!"
    echo "   Check logs for details."
fi
