#![deny(warnings)]

extern crate cmake;

use std::env;

fn main() {
    let dst = cmake::Config::new("src").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=ots");
    println!("cargo:rustc-link-lib=static=woff2");
    println!("cargo:rustc-link-lib=static=brotli");
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    } else if !target.contains("msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
