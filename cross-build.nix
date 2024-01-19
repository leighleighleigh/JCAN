let
  rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";

  pkgs = import <nixpkgs> {
    overlays = [ (import rustOverlay) ];
    crossSystem = {
      config = "aarch64-unknown-linux-gnu";
    };
  };

  rust = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
    ];
  };

  jcan-python = pkgs.python3Packages.callPackage ./jcan_python.nix {pkgs=pkgs;};
in
pkgs.mkShell {
  buildInputs = [rust jcan-python];
  shellHook = ''
    echo "hi"
  '';
}
