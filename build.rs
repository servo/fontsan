#![deny(warnings)]

extern crate cmake;

use std::env;

fn main() {
    let dst = cmake::Config::new("src").build();

    println!("cargo:rustc-link-search=native={}\n\
              cargo:rustc-link-search=native={}/lib\n\
              cargo:rustc-link-lib=static=miniz\n\
              cargo:rustc-link-lib=static=brotli\n\
              cargo:rustc-link-lib=static=woff2\n\
              cargo:rustc-link-lib=static=ots\n\
              cargo:rustc-link-lib=stdc++",
        env::var("DEP_MINIZ_ROOT").unwrap(),
        dst.display());
}
