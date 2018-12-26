extern crate bindgen;

use std::env;
use std::process::Command;
use std::path::PathBuf;

fn main() {
    Command::new("make")
        .current_dir("./c_libs/libpg_query")
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

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
