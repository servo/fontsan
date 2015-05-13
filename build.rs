#![deny(warnings)]

extern crate gcc;
extern crate pkg_config;

use std::env;
use std::process::Command;

fn main() {
    let cwd = env::current_dir().unwrap();

    pkg_config::find_library("zlib").unwrap();

    assert!(Command::new("./scripts/build-ots.sh")
        .status().unwrap().success());

    gcc::Config::new()
        .cpp(true)
        .file("src/ots_glue.cc")
        .flag("-O3").flag("-fPIC")
        .include("ots/include/")
        .compile("libots_glue.a");

    println!("cargo:rustc-link-search=native={}/ots\n\
              cargo:rustc-link-lib=static=brotli\n\
              cargo:rustc-link-lib=static=ots\n\
              cargo:rustc-link-lib=dylib=z",
      cwd.to_str().unwrap());
}
