{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.gcc
    pkgs.python38Full
    pkgs.python38Packages.pip
    pkgs.python38Packages.setuptools
    pkgs.python38Packages.sphinx
    pkgs.python38Packages.readthedocs-sphinx-ext
    pkgs.python38Packages.sphinx_rtd_theme

    # keep this line if you use bash
    pkgs.bashInteractive
  ];

  shellHook = ''
    export PYTHONPATH=/home/fazbdillah/.local/lib/python3.8/site-packages:$PYTHONPATH
    #set -gp PYTHONPATH $HOME/.local/lib/python3.8/site-packages
  '';
}
