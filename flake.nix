{
  description = "Development environment for CatLauncher";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        tauriDependencies = with pkgs; [
          pkg-config
          webkitgtk_4_1
        ];
      in
      {
        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              cargo-edit
              clippy
              nodejs
              pnpm
              rustc
              rustfmt
              uv
            ] ++ tauriDependencies;
          };
        };
      }
    );
}
