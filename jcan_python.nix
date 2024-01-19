{ pkgs ? import <nixpkgs> {}
, lib ? pkgs.lib
, buildPythonPackage ? pkgs.python3Packages.buildPythonPackage
}:

buildPythonPackage rec {
  name = "jcan-python";

  doCheck = true;

  outputs = [ "out" ];

  src = lib.cleanSource ./.;
  sourceRoot = "source/jcan_python";

  dontPatchELF = true;

  cargoDeps = pkgs.rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    cargo
    rustPlatform.cargoSetupHook
    rustc
    python3Packages.setuptools-rust
    python3Packages.toml
    python3Packages.pip
    python3Packages.pytest
  ];

  postPatch = ''
    chmod u+w ..
    ln -s ../Cargo.lock .
  '';
}
