let pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    SDL2
    SDL2_ttf
    lld
  ];
}
