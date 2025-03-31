{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust.url = "github:oxalica/rust-overlay";
  };
  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust,
    }:
    utils.lib.eachDefaultSystem (
      system:

      let
        overlays = [ (import rust) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        buildInputs = with pkgs; [
          pkg-config
          bacon
          watchexec
          vulkan-tools
          vulkan-headers
          vulkan-loader
          vulkan-validation-layers
          clang
          lld
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          alsa-lib
          udev
          vulkan-loader
          libxkbcommon
          mold-wrapped
        ];
        libraries = with pkgs; [
          alsa-lib
          udev
          vulkan-loader
          libxkbcommon
          xorg.libX11
          xorg.libXi
          xorg.libXcursor
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          inherit buildInputs;

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${lib.makeLibraryPath libraries}"
          '';

          stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.stdenv;
        };
      }
    );
}
