#![deny(warnings)]

extern crate cmake;

use std::env;

fn main() {
    let dst = cmake::Config::new("src").build();

    println!("cargo:rustc-link-search=native={}/lib\n\
              cargo:rustc-link-lib=static=brotli\n\
              cargo:rustc-link-lib=static=woff2\n\
              cargo:rustc-link-lib=static=ots",
        dst.display());
    let target = env::var("TARGET").unwrap();
    if !target.contains("msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
