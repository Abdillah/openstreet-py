{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.gcc
    pkgs.python38Full
    pkgs.python38Packages.pip
    pkgs.python38Packages.setuptools

    # keep this line if you use bash
    pkgs.bashInteractive
  ];
}
