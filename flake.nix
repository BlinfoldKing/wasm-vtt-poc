{
  description = "a (mostly) hackable and (hopefully) game agnostic virtual table top";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };


  outputs =
    { self
    , nixpkgs
    , utils
    } @inputs:
    let
      inherit (self) outputs;
    in
    utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.default = with pkgs; mkShell
        rec {
          packages = [
            just
            nodejs
            yarn
            cargo-watch
            concurrently
            wasm-pack
          ];
          nativeBuildInputs = [
            pkg-config
          ];
          buildInputs = [
            udev
            alsa-lib
            vulkan-loader

            # To use the x11 feature
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr

            # To use the wayland feature
            libxkbcommon
            wayland
          ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
    });
}
