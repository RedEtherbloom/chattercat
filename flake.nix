{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk-lib = pkgs.callPackage naersk {};
      in {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            pipewire.dev
            clang
          ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [
            pkgs.llvmPackages_latest.libclang.lib
          ];
        };
      }
    );
}
