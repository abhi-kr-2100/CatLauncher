{
  description = "Development environment for CatLauncher";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          lib = pkgs.lib;

          nodejs =
            if builtins.hasAttr "nodejs_24" pkgs then
              pkgs.nodejs_24
            else
              pkgs.nodejs;

          pnpm =
            if builtins.hasAttr "pnpm_10" pkgs then
              pkgs.pnpm_10
            else
              pkgs.pnpm;

          openssl = pkgs.openssl;

          webkitgtk =
            if builtins.hasAttr "webkitgtk_4_1" pkgs then
              pkgs.webkitgtk_4_1
            else
              pkgs.webkitgtk;

          appIndicatorLibs =
            lib.optionals (builtins.hasAttr "libayatana-appindicator" pkgs) [
              pkgs.libayatana-appindicator
            ]
            ++ lib.optionals (builtins.hasAttr "libappindicator-gtk3" pkgs) [
              pkgs."libappindicator-gtk3"
            ];

          tauriNativeLibs =
            (with pkgs; [
              at-spi2-atk
              atk
              cairo
              dbus
              gdk-pixbuf
              glib
              glib-networking
              gsettings-desktop-schemas
              gtk3
              harfbuzz
              hicolor-icon-theme
              libdrm
              libepoxy
              libglvnd
              librsvg
              libsoup_3
              libxkbcommon
              openssl
              pango
              shared-mime-info
              wayland
              webkitgtk
              xdotool
              libX11
              libXcursor
              libXi
              libXrandr
            ])
            ++ appIndicatorLibs;

          xdgDataDirs = lib.concatStringsSep ":" [
            "${pkgs.gsettings-desktop-schemas}/share"
            "${pkgs.gtk3}/share"
            "${pkgs.hicolor-icon-theme}/share"
            "${pkgs.shared-mime-info}/share"
          ];
        in
        {
          default = pkgs.mkShell {
            packages =
              (with pkgs; [
                cargo
                cargo-edit
                clippy
                curl
                file
                gcc
                gnumake
                nodejs
                pkg-config
                pnpm
                rustc
                rustfmt
                uv
                wget
              ])
              ++ tauriNativeLibs;

            env = {
              GIO_EXTRA_MODULES = "${pkgs.glib-networking}/lib/gio/modules";
              LD_LIBRARY_PATH = lib.makeLibraryPath tauriNativeLibs;
              WEBKIT_DISABLE_COMPOSITING_MODE = "1";
              XDG_DATA_DIRS = xdgDataDirs;
            };
          };
        }
      );
    };
}
