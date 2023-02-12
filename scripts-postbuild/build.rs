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
    //   target/<profile>/libjcan.a
    //   target/cxxbridge/jcan/src/lib.rs.h
    //   target/cxxbridge/jcan/src/lib.rs.cc
    //   target/cxxbridge/rust/cxx.h
    // These files are moved to the `out` directory, which is the crate root
    // out/<target>/jcan
    //    - libjcan.a
    //    - jcan.h
    //    - jcan.cc
    //    - cxx.h
    // This directry can then easily be include-ed in C++ projects

    let target = env::var("TARGET").unwrap();
    println!("cargo:warning=target: {}", target);

    let profile = env::var("PROFILE").unwrap();
    println!("cargo:warning=profile: {}", profile);

    let project_dir = project_root();
    println!("cargo:warning=project_dir: {}", project_dir.display());

    // We will be targeting the 'jcan' subdirectory - this is hard-coded.
    let manifest_dir = project_dir;

    // Delete the project-level out directory
    let out_dir = Path::new(&manifest_dir).join("out");
    println!("cargo:warning=project_out_dir: {}", out_dir.display());

    let out_dir = Path::new(&manifest_dir).join("out").join(&profile).join(&target).join("jcan");
    println!("cargo:warning=out_dir: {}", out_dir.display());

    // If is_debug, copy the debug library, otherwise copy the release library
    // If iscrossbuild, another 'target' subdir is added
    let lib_build_path = Path::new(&manifest_dir)
        .join("target")
        .join(target)
        .join(profile)
        .join("libjcan.a");

    // Create the out directory
    fs::create_dir_all(&out_dir).unwrap();

    // Copy the library to the out directory
    fs::copy(&lib_build_path, &out_dir.join("libjcan.a")).unwrap();

    // Copy the cxxbridge generated headers to the out directory
    fs::copy(
        Path::new(&manifest_dir)
            .join("target")
            .join("cxxbridge")
            .join("jcan")
            .join("src")
            .join("lib.rs.h"),
        &out_dir.join("jcan.h"),
    ).unwrap();

    // Copy the cxxbridge generated source to the out directory
    fs::copy(
        Path::new(&manifest_dir)
            .join("target")
            .join("cxxbridge")
            .join("jcan")
            .join("src")
            .join("lib.rs.cc"),
        &out_dir.join("jcan.cc"),
    ).unwrap();

    // // In the output files jcan.cc and jcan.h, replace the first instance of the string `#include "jcan/include/callback.h" with `#include "callback.h"`
    // let mut jcan_h = fs::read_to_string(&out_dir.join("jcan.h")).unwrap();
    // jcan_h = jcan_h.replace("#include \"jcan/include/callback.h\"", "#include \"callback.h\"");
    // fs::write(&out_dir.join("jcan.h"), jcan_h).unwrap();

    // let mut jcan_cc = fs::read_to_string(&out_dir.join("jcan.cc")).unwrap();
    // jcan_cc = jcan_cc.replace("#include \"jcan/include/callback.h\"", "#include \"callback.h\"");
    // fs::write(&out_dir.join("jcan.cc"), jcan_cc).unwrap();


    // fs::copy(
    //     Path::new(&manifest_dir)
    //         .join("jcan")
    //         .join("include")
    //         .join("callback.h"),
    //     &out_dir.join("callback.h"),
    // ).unwrap();

    // fs::copy(
    //     Path::new(&manifest_dir)
    //         .join("jcan")
    //         .join("src")
    //         .join("callback.cc"),
    //     &out_dir.join("callback.cc"),
    // ).unwrap();

    // // As before with jcan.cc, replace the first instance of the string `#include "jcan/include/callback.h" with `#include "callback.h"`
    // let mut callback_cc = fs::read_to_string(&out_dir.join("callback.cc")).unwrap();
    // callback_cc = callback_cc.replace("#include \"jcan/include/callback.h\"", "#include \"callback.h\"");
    // fs::write(&out_dir.join("callback.cc"), callback_cc).unwrap();


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
