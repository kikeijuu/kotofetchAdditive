{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation rec {
  pname = "kotofetch";
  version = "0.2.16";

  src = pkgs.fetchurl {
    url = "https://github.com/hxpe-dev/kotofetch/releases/download/v${version}/kotofetch-v${version}-x86_64-unknown-linux-gnu.tar.gz";
    sha256 = "sha256-RrA85NTAmWXkhPepo03E0509asjA6Fyp1hCd159L4u0="; # fill in
  };

  installPhase = ''
    mkdir -p $out/bin
    tar -xzf $src --strip-components=1 -C $out/bin
  '';

  meta = with pkgs.lib; {
    description = "Minimalist fetch tool for Japanese quotes (written in Rust)";
    homepage = "https://github.com/hxpe-dev/kotofetch";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.unix;
  };
}
