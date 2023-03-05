#[allow(unused_must_use)]
fn main() {
    // Note from Leigh 12/02/2023
    // Adding the .file("src/callback.cc") is essential for the build to work!
    // Otherwise the functions are not defined and the linker will fail.
    cxx_build::bridge("src/lib.rs").file("src/callback.cc").flag_if_supported("-std=c++14").compile("jcan");

    // Tell cargo to rerun this script if any of the files we copied change
    println!("cargo:rerun-if-changed=src/lib.rs");

}