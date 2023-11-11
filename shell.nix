# shell.nix
# This installs the jcan library (C++ and Python) from the source code,
# by building it with Nix. You are then dropped into a bash shell, where these libraries are
# available to use in your application.
#
# This shell is also useful for JCAN development.
#
{ pkgs ? import <nixpkgs> {} }:                                                    
let
  # rust/C++ library, and python wrapper
  jcan = pkgs.callPackage ./jcan.nix {};
  jcan-python = pkgs.python3Packages.callPackage ./jcan_python.nix {};

  # utility scripts
  mk-vcan = pkgs.writeShellScriptBin "mk-vcan" (builtins.readFile ./utils/mk-vcan.sh);
  rm-vcan = pkgs.writeShellScriptBin "rm-vcan" (builtins.readFile ./utils/rm-vcan.sh);
in
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      rustup
      cargo
      jcan
      jcan-python
      python3
      python310Packages.pip
      can-utils
      mk-vcan
      rm-vcan
      podman
    ];

  shellHook = 
    ''
    echo "Run 'mk-vcan' and 'rm-vcan' to use virtual CAN interfaces!"
    '';
}
