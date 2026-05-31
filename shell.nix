
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
    ];
  in {
    devShell = with pkgs; mkShell {
      buildInputs = [
        cargo
        rustc
        rust-analyzer
        pkg-config
      ];
      
      RUST_LOG = "debug";
      RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      
      LD_LIBRARY_PATH = libPath;

    };
  }