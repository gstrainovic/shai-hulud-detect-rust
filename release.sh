#!/bin/bash
set -euo pipefail

echo ""
# Already in the correct directory (shai-hulud-detect-rust)

echo "🚀 Automated Release Script"
echo "============================"
echo ""

# Extract version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
TAG="v$VERSION"

echo "📦 Current version: $VERSION"
echo "🏷️  Tag will be: $TAG"
echo ""

# Check if tag already exists
# Git remote name
REMOTE="master-remote"

if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "⚠️  Tag $TAG already exists!"
    read -p "Delete and recreate? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Deleting old tag..."
         git tag -d "$TAG"
         git push "$REMOTE" ":refs/tags/$TAG" || true
    else
        echo "❌ Aborted"
        exit 1
    fi
fi

echo ""
echo "1️⃣ Checking git status..."
STATUS=$(git status --short)
if [ -n "$STATUS" ]; then
    echo "⚠️  Uncommitted changes detected:"
    echo "$STATUS"
    echo ""
    read -p "Commit changes first? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
         git add -A
        read -p "Commit message: " COMMIT_MSG
         git commit -m "$COMMIT_MSG"
         git push "$REMOTE" master
    fi
fi

echo ""
echo "2️⃣ Running tests..."
 cargo test
if [ $? -ne 0 ]; then
    echo "❌ Tests failed! Fix before releasing."
    exit 1
fi

echo ""
echo "3️⃣ Building release binary..."
 cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed! Fix before releasing."
    exit 1
fi

echo ""
echo "4️⃣ Creating git tag $TAG..."
 git tag -a "$TAG" -m "Release $TAG

✅ Version: $VERSION
⚡ 230x faster than Bash
🛡️ Memory-safe Rust implementation
🧪 100% Bash-compatible"

echo ""
echo "5️⃣ Pushing tag to GitHub (triggers CI)..."
 git push "$REMOTE" "$TAG"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Release $TAG triggered!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "GitHub Actions will now:"
echo "  1. Build for 5 platforms"
echo "  2. Run tests"
echo "  3. Create release"
echo "  4. Upload binaries"
echo ""
echo "Monitor: https://github.com/gstrainovic/shai-hulud-detect-rust/actions"
echo "Release: https://github.com/gstrainovic/shai-hulud-detect-rust/releases/tag/$TAG"
