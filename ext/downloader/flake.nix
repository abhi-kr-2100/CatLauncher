{
  description = "Downloader - A simple way to download things via HTTP/HTTPS";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cargo
            cargo-edit
            rustc
            rustfmt
            clippy
          ];

          buildInputs = with pkgs; [
            openssl
          ];
        };
      });
}
