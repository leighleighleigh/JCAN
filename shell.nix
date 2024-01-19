# shell.nix
# This shell is useful for JCAN development.
let
  # rust development environment stuff
  # makes rust-analyzer work, and uses nightly rust
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";

  pkgs = import <nixpkgs> {
    overlays = [ (import rustOverlay) ];
  };

  rust = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
    ];
  };

  # rust/C++ library, and python wrapper
  jcan = pkgs.callPackage ./jcan.nix {};
  jcan-python = pkgs.python3Packages.callPackage ./jcan_python.nix {};

  # utility scripts
  mk-vcan = pkgs.writeShellScriptBin "mk-vcan" (builtins.readFile ./utils/mk-vcan.sh);
  rm-vcan = pkgs.writeShellScriptBin "rm-vcan" (builtins.readFile ./utils/rm-vcan.sh);
in
pkgs.mkShell rec {
    buildInputs = [ rust ] ++ (with pkgs; [
      bacon
      can-utils
      cargo
      gcc
      jcan
      jcan-python
      mk-vcan
      pkg-config
      podman
      python3
      python3Packages.pip
      python3Packages.pytest
      rm-vcan
      rust-analyzer
      rustup
      stdenv.cc
    ]);
  
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";

  shellHook = ''
      export PS1="''${debian_chroot:+($debian_chroot)}\[\033[01;39m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\W\[\033[00m\]\$ "
      export PS1="(jcan)$PS1"
      export LD_LIBRARY_PATH="''${LD_LIBRARY_PATH}:${LD_LIBRARY_PATH}"
      echo "Run 'mk-vcan' and 'rm-vcan' to use virtual CAN interfaces!"
  '';
}
