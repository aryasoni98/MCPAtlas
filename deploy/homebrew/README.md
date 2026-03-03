# Homebrew Formula

This directory contains the Homebrew formula for MCPAtlas (kept in sync for reference). The canonical formula is published to the [aryasoni98/mcpatlas](https://github.com/aryasoni98/homebrew-mcpatlas) tap.

## Install

```bash
brew tap aryasoni98/mcpatlas
brew install mcp-atlas
```

## Automation

1. Push to `main` with a version bump in `Cargo.toml` → `tag-on-main` creates a tag (e.g. `v0.1.0`)
2. Tag push triggers the **Release** workflow → builds binaries, creates GitHub Release
3. **Update Homebrew formula** job updates `deploy/homebrew/mcp-atlas.rb` in this repo
4. **Update Homebrew tap** job pushes the formula to `aryasoni98/homebrew-mcpatlas`

No manual `url`/`sha256` updates are required for new releases.

## Tap bootstrap

If the tap repo is empty, run once: see `deploy/homebrew-tap/README.md`.
