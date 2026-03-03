#!/usr/bin/env bash
# Bootstrap the homebrew-mcpatlas tap repo with an initial commit.
# Run once from the MCPAtlas repo root after cloning the empty tap:
#
#   git clone https://github.com/aryasoni98/homebrew-mcpatlas /tmp/homebrew-mcpatlas
#   ./deploy/homebrew-tap/bootstrap.sh /tmp/homebrew-mcpatlas
#   cd /tmp/homebrew-mcpatlas && git push origin main
#
set -e
TAP_DIR="${1:?Usage: $0 <path-to-homebrew-mcpatlas-clone>}"
mkdir -p "$TAP_DIR/Formula"
cp "$(dirname "$0")/Formula/mcp-atlas.rb" "$TAP_DIR/Formula/"
cp "$(dirname "$0")/README.md" "$TAP_DIR/"
cd "$TAP_DIR"
git checkout -b main 2>/dev/null || true
git add .
git diff --staged --quiet && echo "Nothing to commit" || git commit -m "Initial formula (placeholder; release workflow will update)"
echo "Done. Push with: cd $TAP_DIR && git push -u origin main"
