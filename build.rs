#![deny(warnings)]

extern crate cmake;
extern crate gcc;

use std::env;

fn main() {
    let dst = cmake::Config::new("src").build();

    gcc::Config::new()
        .cpp(true)
        .file("src/ots_glue.cc")
        .flag("-O3").flag("-fPIC")
        .include("src/ots/include/")
        .compile("libots_glue.a");

    println!("cargo:rustc-link-search=native={}\n\
              cargo:rustc-link-search=native={}/lib\n\
              cargo:rustc-link-lib=static=miniz\n\
              cargo:rustc-link-lib=static=brotli\n\
              cargo:rustc-link-lib=static=woff2\n\
              cargo:rustc-link-lib=static=ots",
        env::var("DEP_MINIZ_ROOT").unwrap(),
        dst.display());
}
