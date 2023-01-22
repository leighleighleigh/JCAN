#[allow(unused_must_use)]
use std::{env, fs, path::{Path, PathBuf}};

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn main() {
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
    println!("cargo:warning=target: {}", target);

    // Set an 'iscrossbuild' variable, which is true if TARGET != HOST
    let host = env::var("HOST").unwrap();
    let iscrossbuild = if target == host { "false" } else { "true" };
    println!("cargo:warning=iscrossbuild: {}", iscrossbuild);

    let profile = env::var("PROFILE").unwrap();
    println!("cargo:warning=profile: {}", profile);

    let project_dir = project_root();
    println!("cargo:warning=project_dir: {}", project_dir.display());

    // We will be targeting the 'jorzacan' subdirectory - this is hard-coded.
    let manifest_dir = project_dir;

    let out_dir = Path::new(&manifest_dir).join("out").join(&profile).join(&target).join("jorzacan");
    println!("cargo:warning=out_dir: {}", out_dir.display());

    // If is_debug, copy the debug library, otherwise copy the release library
    // If iscrossbuild, another 'target' subdir is added
    let lib_build_path = Path::new(&manifest_dir)
        .join("target")
        .join(if iscrossbuild == "true" { &target } else { "" })
        .join(profile)
        .join("libjorzacan.a");

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
}
