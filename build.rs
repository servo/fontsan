#![deny(warnings)]

use std::path::{Path, PathBuf};

const BROTLI_INCLUDE_DIR: &str = "src/deps/brotli/c/include";
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

    cc::Build::new()
        .cpp(true)
        .files(ots_sources)
        .include(OTS_INCLUDE_DIR)
        .include(LZ4_INCLUDE_DIR)
        .include(WOFF2_INCLUDE_DIR)
        .include(ZLIB_INCLUDE_DIR)
        .std("c++11")
        .compile("ots");
}

fn build_brotli() {
    let brotli_sources = glob::glob("src/deps/brotli/c/**/*.c")
        .expect("Invalid glob pattern")
        .collect::<Result<Vec<PathBuf>, _>>()
        .expect("vendored brotli sources not found");

    cc::Build::new()
        .cpp(false)
        .files(brotli_sources)
        .include(BROTLI_INCLUDE_DIR)
        .compile("brotli");
}

fn build_woff2() {
    let woff2_dir = Path::new("src/deps/woff2/src");
    let file_names = [
        "table_tags.cc",
        "variable_length.cc",
        "woff2_common.cc",
        "woff2_dec.cc",
        "woff2_out.cc",
    ];
    let woff2_sources = file_names.iter().map(|name| woff2_dir.join(name));

    cc::Build::new()
        .cpp(true)
        .files(woff2_sources)
        .include(WOFF2_INCLUDE_DIR)
        .include(BROTLI_INCLUDE_DIR)
        .std("c++11")
        .warnings(false)
        .compile("woff2");
}

fn build_ots_glue() {
    cc::Build::new()
        .cpp(true)
        .file("src/ots_glue.cc")
        .include(OTS_INCLUDE_DIR)
        .std("c++11")
        .compile("ots_glue");
}

fn main() {
    build_lz4();
    build_ots();
    build_brotli();
    build_woff2();
    build_ots_glue();
}
