#[allow(unused_must_use)]
fn main() {

    // Only run this script AFTER the regular build has happened
    // This is because we need to copy the generated library and headers to the generated directory
    cxx_build::bridge("src/lib.rs").file("src/callback.cc").flag_if_supported("-std=c++14").compile("jcan");

    // Tell cargo to rerun this script if any of the files we copied change
    println!("cargo:rerun-if-changed=src/lib.rs");

}