class CtfTui < Formula
  desc "CTF TUI launcher for challenge environment workflows"
  homepage "https://github.com/gandli/ctf-tui-launcher"
  version "0.1.2"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/gandli/ctf-tui-launcher/releases/download/v0.1.2/ctf-tui-macos-aarch64.tar.gz"
      sha256 "74f0e427b68b0df07135fc78766fbfe3a75b8d8702af686f0ec16bdd052e7303"
    else
      url "https://github.com/gandli/ctf-tui-launcher/releases/download/v0.1.2/ctf-tui-macos-x86_64.tar.gz"
      sha256 "d4b2cafc5d3f721b41ce03776431e49cb6b5335bcfce9ef5e4642f6d5c89098b"
    end
  end

  on_linux do
    url "https://github.com/gandli/ctf-tui-launcher/releases/download/v0.1.2/ctf-tui-linux-x86_64.tar.gz"
    sha256 "f2e3b64ab7d28263de0f92fa8db4bba80847763dabb9a3ac693d764bcf8197a2"
  end

  depends_on "docker"

  def install
    bin.install "ctf-tui"
  end

  test do
    system "#{bin}/ctf-tui", "help"
  end
end
