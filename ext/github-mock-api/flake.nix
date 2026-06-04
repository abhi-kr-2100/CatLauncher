{
  description = "GitHub Mock API devshell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; };
      llvm = pkgs.llvmPackages.llvm;
      libclang = pkgs.llvmPackages.libclang;
      jna = pkgs.jna;
    in {
      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          cargo-llvm-cov
          catch2
          check
          clang
          dart
          jna
          kotlin
          libclang
          llvm
          pkg-config
          python3
          python3Packages.pytest
          ruby
          (ruby.withPackages (ps: with ps; [ rspec ]))
          rustc
          skills
        ];

        shellHook = ''
          export LLVM_COV="${llvm}/bin/llvm-cov"
          export LLVM_PROFDATA="${llvm}/bin/llvm-profdata"
          export LIBCLANG_PATH="${libclang.lib}/lib"
          export JNA_JAR="${jna}/share/java/jna.jar"
        '';
      };
    });
}
