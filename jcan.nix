{ lib
, rustPlatform
}:

rustPlatform.buildRustPackage {
  name = "jcan";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  buildAndTestSubdir = "jcan";

  outputs = [ "out" "dev" ];

  postInstall = ''
    mkdir -p "$dev/include/jcan"
    cp target/cxxbridge/jcan/src/lib.rs.h "$dev/include/jcan/jcan.h"
    cp target/cxxbridge/jcan/src/lib.rs.cc "$dev/include/jcan/jcan.cc"
    cp target/cxxbridge/rust/cxx.h "$dev/include/jcan"
    cp jcan/include/* "$dev/include/jcan"
    cp jcan/src/callback.cc "$dev/include/jcan"
    find "$dev/include/jcan" -type f \
      -exec sed -i 's/#include "jcan\/include/#include "jcan/g' {} \; \
      -exec sed -i 's/src\/lib.rs/jcan/g' {} \;
  '';
}
