{ lib
, buildPythonPackage
, rustPlatform
, cargo
, rustc
, setuptools-rust
, toml
}:

buildPythonPackage rec {
  name = "jcan-python";

  src = lib.cleanSource ./.;
  sourceRoot = "source/jcan-python";

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
