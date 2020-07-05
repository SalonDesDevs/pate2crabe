with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "pate2crabe";
    buildInputs = with xorg; [
        pkg-config
        cmake expat freetype
        libxcb libX11 libXcursor libXrandr libXi
        libGL alsaLib udev
    ];
    shellHook = ''
        LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CMAKE_LIBRARY_PATH cargo run --release
        exit
    '';
}
