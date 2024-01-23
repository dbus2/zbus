{ pkgs ? import <nixpkgs> {}
, rustToolchain ? (
    # Legacy Fallback
    let
      fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
    in
      fenix.complete.withComponents [
        "cargo" "clippy" "rust-src" "rustc" "rustfmt"
      ]
  )
, ...
}:
pkgs.mkShell {
  name = "zbus";

  nativeBuildInputs = with pkgs; [
    glib
    pkg-config
    rustToolchain
  ];
}
