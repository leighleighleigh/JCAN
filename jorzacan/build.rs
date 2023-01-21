#[allow(unused_must_use)]
// use cxx_build::CFG;
use std::path::Path;
use std::{env, fs};

fn main() {

    // Only run this script AFTER the regular build has happened
    // This is because we need to copy the generated library and headers to the generated directory
    cxx_build::bridge("src/lib.rs").compile("jorzacan");


    // Whenever we build, we will copy the static library and the cxx-bridge generated headers to the `out` directory.
    // The input files are
    //   target/<profile>/libjorzacan.a
    //   target/cxxbridge/jorzacan/src/lib.rs.h
    //   target/cxxbridge/jorzacan/src/lib.rs.cc
    //   target/cxxbridge/rust/cxx.h
    // These files are moved to the `out` directory, which is the crate root
    // out/<target>/jorzacan
    //    - libjorzacan.a
    //    - jorzacan.h
    //    - jorzacan.cc
    //    - cxx.h
    // This directry can then easily be include-ed in C++ projects

    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&manifest_dir).join("out").join(&target).join("jorzacan");

    // If is_debug, copy the debug library, otherwise copy the release library
    let lib_build_path = Path::new(&manifest_dir)
        .join("target")
        .join(profile)
        .join("libjorzacan.a");

    // If path or file doesnt exist, skip build rs
    if !lib_build_path.exists() {
        // Print path
        println!("cargo:warning=libjorzacan.a not found at: {}", lib_build_path.display());
        return;
    }

    // Create the out directory
    fs::create_dir_all(&out_dir).unwrap();

    // Copy the library to the out directory
    fs::copy(&lib_build_path, &out_dir.join("libjorzacan.a")).unwrap();

    // Copy the cxxbridge generated headers to the out directory
    fs::copy(
        Path::new(&manifest_dir)
            .join("target")
            .join("cxxbridge")
            .join("jorzacan")
            .join("src")
            .join("lib.rs.h"),
        &out_dir.join("jorzacan.h"),
    ).unwrap();

    // Copy the cxxbridge generated source to the out directory
    fs::copy(
        Path::new(&manifest_dir)
            .join("target")
            .join("cxxbridge")
            .join("jorzacan")
            .join("src")
            .join("lib.rs.cc"),
        &out_dir.join("jorzacan.cc"),
    ).unwrap();

    // Copy the cxxbridge generated source to the out directory
    fs::copy(
        Path::new(&manifest_dir)
            .join("target")
            .join("cxxbridge")
            .join("rust")
            .join("cxx.h"),
        &out_dir.join("cxx.h"),
    ).unwrap();

    // Tell cargo to rerun this script if any of the files we copied change
    println!("cargo:rerun-if-changed=src/lib.rs");
}