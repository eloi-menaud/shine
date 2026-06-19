{ pkgs ? import <nixpkgs> {} }:
let
  libPath = with pkgs; lib.makeLibraryPath [
    libGL
    vulkan-loader
    libxkbcommon
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    a2ps
  ];
in {
  devShell = with pkgs; mkShell {
    buildInputs = [
      cargo
      rustc
      rust-analyzer
      pkg-config
    ];

    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    
    LD_LIBRARY_PATH = libPath;
  };
}
