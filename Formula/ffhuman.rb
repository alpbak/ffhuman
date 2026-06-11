class Ffhuman < Formula
  desc "FFmpeg for humans - a human-friendly CLI wrapper around FFmpeg"
  homepage "https://github.com/alpbak/ffhuman"
  version "1.0.7"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/alpbak/ffhuman/releases/download/v#{version}/ffhuman-macos-x86_64.tar.gz"
      sha256 "eeec3259ad848c54b27fd481a43d58711475c8fd3ef92943385dc9d4f3048e69"
    elsif Hardware::CPU.arm?
      url "https://github.com/alpbak/ffhuman/releases/download/v#{version}/ffhuman-macos-aarch64.tar.gz"
      sha256 "663a58a2023451dedf83469a9143b81bf55e83313615ff9fc330fd63c6f419b4"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/alpbak/ffhuman/releases/download/v#{version}/ffhuman-linux-x86_64.tar.gz"
      sha256 "9c95fbfc7ca6f4cf733c285b51dd427d0e98bfd2dabd560d49f3b379bb1c2889"
    end
  end

  depends_on "ffmpeg"

  def install
    if OS.mac?
      arch_dir = if Hardware::CPU.intel?
                   "ffhuman-macos-x86_64"
                 else
                   "ffhuman-macos-aarch64"
                 end
    else
      arch_dir = "ffhuman-linux-x86_64"
    end

    bin.install "#{arch_dir}/ffhuman"
  end

  test do
    assert_match "FFmpeg for humans", shell_output("#{bin}/ffhuman --help")
  end
end
