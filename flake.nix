{
  description = "Development environment for CatLauncher";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        llvm = pkgs.llvmPackages.llvm;

        tauriDependencies = with pkgs; [
          pkg-config
          webkitgtk_4_1
        ];
      in
      {
        devShells = {
          default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                cargo
                cargo-edit
                cargo-llvm-cov
                clippy
                go-task
                lldb
                llvm
                nodejs
                pnpm
                python3
                python313Packages.jedi-language-server
                python313Packages.python-lsp-server
                ruff
                rust-analyzer
                rust-code-analysis
                rustc
                rustfmt
                ty
                typescript-language-server
                uv
              ]
              ++ tauriDependencies;

            shellHook = ''
              export LLVM_COV="${llvm}/bin/llvm-cov"
              export LLVM_PROFDATA="${llvm}/bin/llvm-profdata"
            '';
          };
        };
      }
    );
}
