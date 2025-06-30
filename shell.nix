{ pkgs ? import (fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/08f22084e6085d19bcfb4be30d1ca76ecb96fe54.tar.gz";
    sha256 = "sha256-XE/lFNhz5lsriMm/yjXkvSZz5DfvKJLUjsS6pP8EC50=";
  }) {}
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    pkg-config
    rust-analyzer
    rustfmt
    clippy
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  shellHook = ''
    echo "Welcome to Rust Shell."
  '';
}
