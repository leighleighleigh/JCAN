{ pkgs ? import <nixpkgs> {}
, lib ? pkgs.lib
, buildPythonPackage ? pkgs.python3Packages.buildPythonPackage
}:

buildPythonPackage rec {
  name = "jcan-python";

  doCheck = true;
  pythonImportsCheck = [ "jcan" ];


  src = lib.cleanSource ./.;
  sourceRoot = "source/jcan_python";

  preBuild = ''
  # not cleaning causes some issues due to permissions,
  # for some reason.
  cargo clean

  #ls $src
  #exit 0
  #rm -rf ./target/
  '';

  outputs = [ "out" ];
  dontPatchELF = true;

  cargoDeps = pkgs.rustPlatform.importCargoLock {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = with pkgs; [
    rustPlatform.cargoSetupHook
    cargo
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
