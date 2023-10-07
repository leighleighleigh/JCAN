{ pkgs ? import <nixpkgs> {} }:                                                    
let
  # rust/C++ library, and python wrapper
  jcan = pkgs.callPackage ./jcan.nix {};
  jcan_python = pkgs.python3Packages.callPackage ./jcan_python.nix {};
in
jcan
