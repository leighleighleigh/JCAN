#[allow(unused_must_use)]
// use cxx_build::CFG;
use std::path::Path;
use std::{env, fs};

fn main() {

    // Only run this script AFTER the regular build has happened
    // This is because we need to copy the generated library and headers to the generated directory
    cxx_build::bridge("src/lib.rs").compile("jorzacan");

    // Check if we are runing a debug or a release build
    // let target = env::var("TARGET").unwrap();
    // let profile = env::var("PROFILE").unwrap();
    // let is_release = profile == "release";
    // let is_debug = profile == "debug";

    // For either release or debug, we need to copy the static compiled library to the target directory
    // These are built into ./target/<debug/release>/libjorzacan.a 
    // , and copied into ./generated/
    // We will also copy the generated headers from ./target/cxxbridge to ./generated/
    // Generates is in the crate root, which we can get from env variable CARGO_MANIFEST_DIR
    // let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let generated_dir = Path::new(&manifest_dir).join("generated");

    // // If is_debug, copy the debug library, otherwise copy the release library
    // let lib_build_path = Path::new(&manifest_dir)
    //     .join("target")
    //     .join(profile)
    //     .join("libjorzacan.a");

    // // If path or file doesnt exist, skip build rs
    // if !lib_build_path.exists() {
    //     // Print path
    //     println!("cargo:warning=libjorzacan.a not found at: {}", lib_build_path.display());
    //     return;
    // }

    // // Copy the library to the generated directory
    // fs::copy(&lib_build_path, generated_dir.join("libjorzacan.a")).unwrap();

    // // Copy the headers to the generated directory
    // let cxxbridge_dir = Path::new(&manifest_dir)
    //     .join("target")
    //     .join("cxxbridge");

    // // Copy the headers recursively to the generated directory, using a shell command
    // std::process::Command::new("cp")
    //     .arg("-r")
    //     .arg(cxxbridge_dir)
    //     .arg(generated_dir)
    //     .status()
    //     .expect("Failed to copy cxxbridge headers");

    // Clean ./build/, and rerun CMake to build the C++ examples
    // std::fs::remove_dir_all("./build/*");
    // Make ./build/
    // std::fs::create_dir("./build")?;

    // // cd into ./build/, then run `cmake ..`
    // std::process::Command::new("cmake")
    //     .arg("..")
    //     .current_dir("./build")
    //     .status()
    //     .expect("Failed to run cmake");
    
    // // Then `cmake --build .`
    // std::process::Command::new("cmake")
    //     .arg("--build")
    //     .arg(".")
    //     .current_dir("./build")
    //     .status()
    //     .expect("Failed to run cmake --build");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/main.cpp");
}