let pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    lld
    pkg-config
    alsaLib
    x11
    xorg.libXi
    libGL
  ];
}
