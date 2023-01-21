use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use man::prelude::*;

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

dist            builds application and man pages
"
    )
}

fn dist() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir());
    fs::create_dir_all(&dist_dir())?;

    dist_binary()?;
    // dist_manpage()?;
    dist_artifacts()?;

    Ok(())
}

fn dist_binary() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    let dst = project_root().join("target/release/hello-world");

    fs::copy(&dst, dist_dir().join("hello-world"))?;

    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip").arg(&dst).status()?;
        if !status.success() {
            Err("strip failed")?;
        }
    } else {
        eprintln!("no `strip` utility found")
    }

    Ok(())
}

fn dist_artifacts() -> Result<(), DynError> {
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

    /* Print the above resolved paths */
    println!("cargo:warning=manifest_dir: {}", manifest_dir);
    println!("cargo:warning=out_dir: {}", out_dir.display());
    println!("cargo:warning=target: {}", target);
    println!("cargo:warning=profile: {}", profile);

    // If is_debug, copy the debug library, otherwise copy the release library
    let lib_build_path = Path::new(&manifest_dir)
        .join("target")
        .join(profile)
        .join("libjorzacan.a");

    // If path or file doesnt exist, skip build rs
    if !lib_build_path.exists() {
        // Print path
        println!("cargo:warning=libjorzacan.a not found at: {}", lib_build_path.display());
        return Ok(());
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

    Ok(())
}

fn dist_manpage() -> Result<(), DynError> {
    let page = Manual::new("hello-world")
        .about("Greets the world")
        .render();
    fs::write(dist_dir().join("hello-world.man"), &page.to_string())?;
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir() -> PathBuf {
    project_root().join("target/dist")
}
