extern crate bindgen;

use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    Command::new(format!("cp -r ./c_libs/libpg_query {}/.", out_path))
        .output()
        .expect("Failed to copy libpg_query to OUT_DIR {}", out_path);

    Command::new("make")
        .current_dir(format!("{}/libpg_query", out_path))
        .output()
        .expect("Failed to build libpg_query");

    println!("cargo:rustc-link-search=native={}", "./c_libs/libpg_query");
    println!("cargo:rustc-link-lib=static={}", "pg_query");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("./c_libs/libpg_query/pg_query.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
