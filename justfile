default: clean
  # Release
  nix-shell cross-build.nix

clean:
  # Run cargo clean
  cargo clean

  # Remove jcan-python/dist, build, **.egg-info folders
  rm -rf "{{justfile_directory()}}/out"
  rm -rf "{{justfile_directory()}}/jcan_python/dist"
  rm -rf "{{justfile_directory()}}/jcan_python/build"
  rm -rf "{{justfile_directory()}}/jcan_python/jcan.egg-info"

