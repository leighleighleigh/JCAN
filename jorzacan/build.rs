// build.rs
use std::env;
use std::path::Path;
use std::process::Command;


fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/demo.cc")
        .cpp(true)
        .cpp_link_stdlib(None) // linked via link-cplusplus crate
        .flag_if_supported("-std=c++14")
        .warnings_into_errors(cfg!(deny_warnings))
        .compile("jorzacan-cpp-demo");
        // .out_dir("build/")

        // cc::Build::new()
        // .file("src/demo.cc")
        // .cpp(true)
        // .cpp_link_stdlib(None) // linked via link-cplusplus crate
        // .flag_if_supported(cxxbridge_flags::STD)
        // .warnings_into_errors(cfg!(deny_warnings))
        // .compile("jorzacan-cpp-demo");

        println!("cargo:rerun-if-changed=src/demo.cc");
        println!("cargo:rerun-if-changed=include/demo.h");
        println!("cargo:rustc-cfg=built_with_cargo");
        println!("cargo:rerun-if-changed=src/main.rs");
        println!("cargo:rerun-if-changed=src/lib.rs");
}
