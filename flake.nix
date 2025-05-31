# SPDX-FileCopyrightText: 2025 2025 Fundament Software SPC <https://fundament.software>
#
# SPDX-License-Identifier: Apache-2.0

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = args:
    args.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import args.nixpkgs) {
          inherit system;
        };

        runtimeDeps = [
          pkgs.xorg.libxcb
          pkgs.xorg.libX11
          pkgs.xorg.libXcursor
          pkgs.xorg.libXrandr
          pkgs.xorg.libXi
          pkgs.freetype
          pkgs.fontconfig
          pkgs.gtk3
          pkgs.python3
          pkgs.libGL
          pkgs.libGLU
          pkgs.wayland
          pkgs.libxkbcommon
          pkgs.pkg-config
          pkgs.openssl
          pkgs.openssl.dev
          pkgs.kdePackages.kdialog
          pkgs.yad
          pkgs.at-spi2-atk
        ];

        LD_LIBRARY_PATH = "/run/opengl-driver/lib/:${pkgs.lib.makeLibraryPath runtimeDeps}";

        devShellPkgs = [
          pkgs.cargo-deny
          pkgs.cargo-bloat
          pkgs.cargo-flamegraph
          pkgs.cargo-udeps
					pkgs.cargo-machete
					pkgs.reuse
          pkgs.rustfmt
          pkgs.pkg-config
          pkgs.just
          pkgs.cmake
        ] ++ runtimeDeps;

        fenix = args.fenix.packages.${system};

        toolchain = with fenix;
          combine [
            complete.rustc
            complete.cargo
            targets.x86_64-pc-windows-gnu.latest.rust-std
          ];

        naersk = args.naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };
        self = {
          packages.default = self.packages.x86_64-pc-windows-gnu;

          devShells.default = self.devShells.rustup-dev;

          devShells.rustup-dev = pkgs.stdenv.mkDerivation {
            inherit LD_LIBRARY_PATH;
            name = "rustup-dev-shell";

            shellHook = ''
              export CC=
              export NIX_CFLAGS_COMPILE=
              export NIX_CFLAGS_COMPILE_FOR_TARGET=
            '';

            depsBuildBuild = with pkgs; [
              pkg-config
            ];

            nativeBuildInputs = with pkgs; [
              mold
              lld
              bubblewrap
            ];

            GLIBC_PATH = "${pkgs.glibc_multi}/lib";

            buildInputs = with pkgs; [
              glibc_multi
              rustup
              libunwind
              stdenv.cc
            ] ++ devShellPkgs;
          };
        };
      in
      self
    );
}
