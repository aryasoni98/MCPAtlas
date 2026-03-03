# Homebrew formula for MCPAtlas.
# Tap: brew tap aryasoni98/mcpatlas && brew install mcp-atlas
# Placeholders __VERSION__, __REPO__, __SHA_INTEL__, __SHA_ARM__ are replaced by the release workflow.
class McpAtlas < Formula
  desc "MCP server for the CNCF Landscape"
  homepage "https://github.com/aryasoni98/MCPAtlas"
  license "Apache-2.0"

  on_macos do
    on_intel do
      url "https://github.com/__REPO__/releases/download/v__VERSION__/mcp-atlas-__VERSION__-x86_64-apple-darwin.tar.gz"
      sha256 "__SHA_INTEL__"
    end
    on_arm do
      url "https://github.com/__REPO__/releases/download/v__VERSION__/mcp-atlas-__VERSION__-aarch64-apple-darwin.tar.gz"
      sha256 "__SHA_ARM__"
    end
  end

  def install
    bin.install "mcp-atlas"
    bin.install "mcp-atlas-cli" if File.exist?("mcp-atlas-cli")
  end

  test do
    assert_match "mcp-atlas", shell_output("#{bin}/mcp-atlas --help", 1)
  end
end
