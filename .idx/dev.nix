{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    gcc
  ];
  
  shellHook = ''
    echo "Rust development environment loaded"
  '';
}