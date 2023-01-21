#[allow(unused_must_use)]
fn main() {
    // Run 'maturin build' shell command. This builds the python wheels for distribution
    // This is a bit of a hack, but it works
    // Make sure to first change the directory to that of this cargo crate
    // This is because maturin will look for the Cargo.toml file in the current directory
    // and will fail if it is not there
    // std::env::set_current_dir(std::path::Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str())).unwrap();
    // std::process::Command::new("maturin").arg("build").output().unwrap();
    // Tell cargo to rerun this script if any of the files we copied change
    // println!("cargo:rerun-if-changed=src/lib.rs");
}