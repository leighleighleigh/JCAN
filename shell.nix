{ pkgs ? import <nixpkgs> {} }:                                                    
let
  # rust/C++ library, and python wrapper
  jcan = pkgs.callPackage ./jcan.nix {};
  jcan_python = pkgs.python3Packages.callPackage ./jcan_python.nix {};

  # utility scripts
  mk-vcan = pkgs.writeShellScriptBin "mk-vcan" (builtins.readFile ./utils/mk-vcan.sh);
  rm-vcan = pkgs.writeShellScriptBin "rm-vcan" (builtins.readFile ./utils/rm-vcan.sh);
in
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
      jcan
      jcan_python
      python3
      python310Packages.pip
      can-utils
      mk-vcan
      rm-vcan
    ];

  shellHook = 
    ''
    echo "Run 'mk-vcan' and 'rm-vcan' to use virtual CAN interfaces!"
    '';

  LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
}
