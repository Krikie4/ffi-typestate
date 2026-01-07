use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let c_lib = manifest_dir.join("../c_lib"); // path to your C library

    println!("cargo:rustc-link-search=native={}", c_lib.display());
    println!("cargo:rustc-link-lib=static=c_lib");
    println!("cargo:rerun-if-changed={}", c_lib.join("c_lib.c").display());
    println!("cargo:rerun-if-changed={}", c_lib.join("c_lib.h").display());
}
