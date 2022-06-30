extern crate bindgen;

use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

fn exit_ok(status: ExitStatus) -> Result<(), io::Error> {
    if !status.success() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("error status: {}", status),
        ))
    } else {
        Ok(())
    }
}

fn build_from_source(out_path: &Path) {
    Command::new("cp")
        .args(&[
            "-r",
            "./c_libs/libpg_query",
            &out_path.display().to_string(),
        ])
        .status()
        .and_then(|s| exit_ok(s))
        .expect(&format!(
            "Failed to copy libpg_query to OUT_DIR {}",
            out_path.join("libpg_query").display()
        ));

    let make_dir = format!("{}/libpg_query", out_path.display());
    eprintln!("make -C {}", make_dir);
    Command::new("make")
        .current_dir(make_dir)
        .status()
        .and_then(|s| exit_ok(s))
        .expect("Failed to build libpg_query");

    println!(
        "cargo:rustc-link-search=native={}",
        out_path.join("libpg_query").display()
    );
}

fn build_from_system(system_path: &Path) {
    println!(
        "cargo:rustc-link-search=native={}",
        system_path.join("lib").display()
    );
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-lib=static=pg_query");

    let system_path = env::var("LIBPG_QUERY").map(PathBuf::from);
    let header_path = if let Ok(system_path) = system_path {
        build_from_system(&system_path);
        system_path.join("include/pg_query.h").display().to_string()
    } else {
        build_from_source(&out_path);
        out_path
            .join("libpg_query/pg_query.h")
            .display()
            .to_string()
    };

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(header_path)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
