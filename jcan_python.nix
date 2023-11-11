{ pkgs ? import <nixpkgs> {}
, lib ? pkgs.lib
, buildPythonPackage ? pkgs.python3Packages.buildPythonPackage
, rustPlatform ? pkgs.rustPlatform
, cargo ? pkgs.cargo
, rustc ? pkgs.rustc
, setuptools-rust ? pkgs.python3Packages.setuptools-rust
, toml ? pkgs.python3Packages.toml
}:

buildPythonPackage rec {
  name = "jcan-python";

  doCheck = false;

  outputs = [ "out" ];

  src = lib.cleanSource ./.;
  sourceRoot = "source/jcan_python";

  dontPatchELF = true;

  cargoDeps = rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    cargo
    rustPlatform.cargoSetupHook
    rustc
    setuptools-rust
    toml
  ];

  postPatch = ''
    chmod u+w ..
    ln -s ../Cargo.lock .
  '';
}
