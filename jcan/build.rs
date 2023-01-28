use cxx_build::CFG;
use std::path::Path;
use std::{env, fs};

#[allow(unused_must_use)]
fn main() {
    cxx_build::bridge("src/lib.rs").flag_if_supported("-std=c++14").compile("jcan");

    // Tell cargo to rerun this script if any of the files we copied change
    println!("cargo:rerun-if-changed=src/lib.rs");

}