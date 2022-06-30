extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rustc-link-lib=static=pg_query");

    let system_path = env::var("LIBPG_QUERY_PATH").map(PathBuf::from);
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

fn build_from_system(system_path: &Path) {
    println!(
        "cargo:rustc-link-search=native={}",
        system_path.join("lib").display()
    );
}

fn build_from_source(out_path: &Path) {
    run_command(
        "cp",
        &[
            "-r",
            "./c_libs/libpg_query",
            &out_path.display().to_string(),
        ],
        None,
    );

    let make_dir = format!("{}/libpg_query", out_path.display());
    run_command("make", &[], Some(make_dir));

    println!(
        "cargo:rustc-link-search=native={}",
        out_path.join("libpg_query").display()
    );
}

fn run_command(exe: &str, args: &[&str], dir: Option<String>) {
    let mut c = Command::new(exe);
    c.args(args);
    if let Some(dir) = dir {
        c.current_dir(dir);
    }

    let output = c.output().expect(&format!(
        "failed to run command: {}",
        command_str(exe, args),
    ));
    let code = output.status.code().unwrap_or(-1);
    if code != 0 {
        let mut msg = format!(
            "Failed to run {} with exit code of {}",
            command_str(exe, args),
            code
        );
        if !output.stdout.is_empty() {
            if let Ok(out) = String::from_utf8(output.stdout) {
                msg.push('\n');
                msg.push_str(&format!("stdout =\n{out}"));
            }
        }
        if !output.stderr.is_empty() {
            if let Ok(out) = String::from_utf8(output.stderr) {
                msg.push('\n');
                msg.push_str(&format!("stderr =\n{out}"));
            }
        }
        panic!("{}", msg);
    }
}

fn command_str(exe: &str, args: &[&str]) -> String {
    format!("{exe} {}", args.join(" "))
}
