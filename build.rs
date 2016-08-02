#![deny(warnings)]

extern crate gcc;

use std::env;
use std::process::Command;

fn main() {
    assert!(Command::new("./scripts/build-ots.sh")
        .status().unwrap().success());

    gcc::Config::new()
        .cpp(true)
        .file("src/ots_glue.cc")
        .flag("-O3").flag("-fPIC")
        .include("ots/include/")
        .compile("libots_glue.a");

    println!("cargo:rustc-link-search=native={}\n\
              cargo:rustc-link-search=native={}/ots\n\
              cargo:rustc-link-lib=static=miniz\n\
              cargo:rustc-link-lib=static=brotli\n\
              cargo:rustc-link-lib=static=woff2\n\
              cargo:rustc-link-lib=static=ots",
        env::var("DEP_MINIZ_ROOT").unwrap(),
        env::var("CARGO_MANIFEST_DIR").unwrap());
}
