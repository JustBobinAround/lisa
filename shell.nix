# shell.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.clang_16
    pkgs.rustup
  ];

  shellHook = ''
    rustup default stable
  '';

  #uncomment for release testing:
  #PURE_ENV = true;
}

