{
  description = "Rust API for D-Bus communications";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils/main";
    nixpkgs.url = "github:NixOS/nixpkgs/master";
  };

  outputs = { fenix, flake-utils, nixpkgs, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      rustToolchain = fenix.packages.${system}.complete.withComponents [
        "cargo" "clippy" "rust-src" "rustc" "rustfmt"
      ];
    in
    {
      devShells.default = import ./shell.nix { inherit pkgs rustToolchain; };
    }
  );
}
