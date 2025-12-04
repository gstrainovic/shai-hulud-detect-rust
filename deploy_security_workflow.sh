#!/bin/bash
# deploy_security_workflow.sh - Deploy security scan workflow to all workspace projects
# This script copies the security-scan.yml workflow to specified projects

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKFLOW_SOURCE="$SCRIPT_DIR/.github/workflows/security-scan.yml"

# Projects to deploy to (excluding rust-scanner itself)
PROJECTS=(
    "/c/Users/gstra/Code/barcode-scanner-v2"
    "/c/Users/gstra/Code/gz-ui"
    "/c/Users/gstra/Code/strapi"
    "/c/Users/gstra/Code/backuper"
    "/c/Users/gstra/Code/scripts"
)

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 DEPLOY SECURITY SCAN WORKFLOW"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ ! -f "$WORKFLOW_SOURCE" ]; then
    echo "❌ Workflow source not found: $WORKFLOW_SOURCE"
    exit 1
fi

deployed=0
skipped=0

for project in "${PROJECTS[@]}"; do
    project_name=$(basename "$project")
    
    if [ ! -d "$project" ]; then
        echo "⏭️  Skipped: $project_name (directory not found)"
        skipped=$((skipped + 1))
        continue
    fi
    
    # Check if it's a git repository
    if [ ! -d "$project/.git" ]; then
        echo "⏭️  Skipped: $project_name (not a git repository)"
        skipped=$((skipped + 1))
        continue
    fi
    
    # Create .github/workflows directory
    workflow_dir="$project/.github/workflows"
    mkdir -p "$workflow_dir"
    
    # Copy workflow
    cp "$WORKFLOW_SOURCE" "$workflow_dir/security-scan.yml"
    
    echo "✅ Deployed: $project_name"
    deployed=$((deployed + 1))
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 SUMMARY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "   Deployed: $deployed"
echo "   Skipped:  $skipped"
echo ""

if [ "$deployed" -gt 0 ]; then
    echo "📝 Next steps:"
    echo "   1. cd to each project"
    echo "   2. git add .github/workflows/security-scan.yml"
    echo "   3. git commit -m '✨ Add security scan workflow'"
    echo "   4. git push"
    echo ""
    echo "⚠️  Note: The workflow requires the shai-hulud-detector release to be"
    echo "   available at: https://github.com/Cobenian/shai-hulud-detect-rust/releases"
    echo ""
    echo "   If not available yet, the workflow will build from source (slower)."
fi
