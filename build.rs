#![deny(warnings)]

extern crate cmake;

fn main() {
    let dst = cmake::Config::new("src").build();

    println!("cargo:rustc-link-search=native={}/lib\n\
              cargo:rustc-link-lib=static=brotli\n\
              cargo:rustc-link-lib=static=woff2\n\
              cargo:rustc-link-lib=static=ots\n\
              cargo:rustc-link-lib=stdc++",
        dst.display());
}
