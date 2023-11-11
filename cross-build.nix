# cross-build.nix
# This shell uses the 'cross-rs' tool to cross-compile JCAN.
# Whilst this is done easily on Nix, through it's own cross-compilation support,
# it doesn't produce a python wheel which is useful on non-Nix systems.
#
# Hence, this shell is primarily designed to produce a Python wheel for release to PyPi.
#
{ pkgs ? import <nixpkgs> {} }:                                                    
let
  clean-script = pkgs.writeScript "clean.sh" ''
  #!/usr/bin/env bash

  SCRIPT_DIR=$( cd -- "$( dirname -- "''${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

  # Remove jcan-python/dist, build, **.egg-info folders
  rm -rf "''${SCRIPT_DIR}/out"
  rm -rf "''${SCRIPT_DIR}/jcan_python/dist"
  rm -rf "''${SCRIPT_DIR}/jcan_python/build"
  rm -rf "''${SCRIPT_DIR}/jcan_python/jcan.egg-info"

  # Run cargo clean
  cargo clean
  '';

  build-script = pkgs.writeScript "crossbuild.sh" ''
  #!/usr/bin/env bash
  SCRIPT_DIR=$( cd -- "$( dirname -- "''${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

  # This function takes a TARGET as an argument, and builds the library for that target
  # It then moves the build artifacts to out/<profile>/<target>/jcan/
  function build_target {
      CROSSTARGET="''${1}"
      export CROSS_CONTAINER_ENGINE=docker

      # Very important to clean, incase old crates for x86 are present
      cargo clean
      cross build --package jcan --target $CROSSTARGET --release
      cross build --package scripts_postbuild --target $CROSSTARGET --release

      # python build uses a special pyo3 image
      export CARGO=cross
      export CARGO_BUILD_TARGET=''${CROSSTARGET}
      export CROSS_CONFIG=''${SCRIPT_DIR}/jcan_python/Cross.toml

      cross build --package jcan_python --target $CROSSTARGET --release

      # Run setuptools-rust on jcan_python
      cd jcan_python
      rm -rf ./dist
      rm -rf ./build

      # Change plat-name depending on CROSSTARGET
      if [[ "''${CROSSTARGET}" == "aarch64-unknown-linux-gnu" ]];
      then
          PLATNAME="manylinux2014_aarch64"
      elif [[ "''${CROSSTARGET}" == "x86_64-unknown-linux-gnu" ]];
      then
          PLATNAME="manylinux2014_x86_64"
      else
          echo "Unknown CROSSTARGET: ''${CROSSTARGET}"
          exit 1
      fi

      python setup.py bdist_wheel --plat-name $PLATNAME --py-limited-api=cp38 || exit 1

      cd ..

      # Copy the resulting wheels to out folder
      mkdir -p out/python/
      cp -r jcan_python/dist/*.whl out/python/
  }

  # Build for aarch64
  build_target "aarch64-unknown-linux-gnu"

  # Build for x86_64
  build_target "x86_64-unknown-linux-gnu"
  '';
in
(pkgs.buildFHSEnv {
    name = "jcan-cross-env";

    targetPkgs = pkgs: [
      pkgs.rustup
      pkgs.cargo
      pkgs.python3
      pkgs.python310Packages.pip
      pkgs.python310Packages.wheel
      pkgs.python310Packages.setuptools-rust
      pkgs.python3Packages.toml
      pkgs.docker
      #pkgs.podman
      pkgs.hostname
      pkgs.direnv
    ];

  runScript = pkgs.writeScript "init.sh" ''
    export PYTHONPATH="/lib/python3.10/site-packages/"
    bash ${clean-script}
    bash ${build-script}
    rm -rf ./target
    '';
}).env
