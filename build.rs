#![deny(warnings)]

extern crate cmake;

use std::env;
use std::path::Path;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    for (key, value) in env::vars() {
        p!("{key}: {value}");
    }

    let dst = cmake::Config::new("src").build();
    let lib_dest_path = Path::new(&dst)
        .join("lib")
        .into_os_string()
        .into_string()
        .expect("Could not turn path into string.");

    println!("cargo:rustc-link-search=native={lib_dest_path}\n\
              cargo:rustc-link-lib=static=brotli\n\
              cargo:rustc-link-lib=static=woff2\n\
              cargo:rustc-link-lib=static=ots");
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else if !target.contains("msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
