# Homebrew Tap for MCPAtlas

This directory contains the formula used to publish to the [aryasoni98/mcpatlas](https://github.com/aryasoni98/homebrew-mcpatlas) tap.

## Install

```bash
brew tap aryasoni98/mcpatlas
brew install mcp-atlas
```

## Bootstrap (one-time)

If the tap repo is empty, run once:

```bash
git clone https://github.com/aryasoni98/homebrew-mcpatlas /tmp/homebrew-mcpatlas
./deploy/homebrew-tap/bootstrap.sh /tmp/homebrew-mcpatlas
cd /tmp/homebrew-mcpatlas && git push -u origin main
```

## Publishing

The release workflow automatically pushes updates to the tap when a new version is released.
