#![deny(warnings)]

use std::path::PathBuf;

const LZ4_INCLUDE_DIR: &str = "src/deps/lz4/lib";
const OTS_INCLUDE_DIR: &str = "src/deps/ots/include";
const WOFF2_INCLUDE_DIR: &str = "src/deps/woff2/include";
const ZLIB_INCLUDE_DIR: &str = "src/fake-zlib";

fn build_lz4() {
    cc::Build::new()
        .cpp(false)
        .file("src/deps/lz4/lib/lz4.c")
        .compile("lz4");
}

fn build_ots() {
    let ots_sources = glob::glob("src/deps/ots/src/*.cc")
        .expect("Invalid glob pattern")
        .collect::<Result<Vec<PathBuf>, _>>()
        .expect("vendored ots sources not found");

    let mut builder = cc::Build::new();

    builder
        .cpp(true)
        .files(ots_sources)
        .include(OTS_INCLUDE_DIR)
        .include(LZ4_INCLUDE_DIR)
        .include(WOFF2_INCLUDE_DIR)
        .include(ZLIB_INCLUDE_DIR);

    if !builder.get_compiler().is_like_msvc() {
        // MSVC does not support C++11 and defaults to C++14.
        // The c++11 requirement is taken from the upstream ots meson build.
        builder.std("c++11");
    }
    builder.compile("ots");
}

fn build_ots_glue() {
    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .file("src/ots_glue.cc")
        .include(OTS_INCLUDE_DIR);

    if !builder.get_compiler().is_like_msvc() {
        builder.std("c++11");
    }

    builder.compile("ots_glue");
}

fn main() {
    build_lz4();
    build_ots();
    build_ots_glue();
}
