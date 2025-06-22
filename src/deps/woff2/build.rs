use std::path::Path;

const BROTLI_INCLUDE_DIR: &str = "brotli/c/include";

fn build_woff2() {
    let woff2_src_dir = Path::new("src");
    let file_names = [
        "table_tags.cc",
        "variable_length.cc",
        "woff2_common.cc",
        "woff2_dec.cc",
        "woff2_out.cc",
    ];
    let woff2_sources = file_names.iter().map(|name| woff2_src_dir.join(name));

    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .files(woff2_sources)
        .include("include")
        .include(BROTLI_INCLUDE_DIR)
        .warnings(false);

    if !builder.get_compiler().is_like_msvc() {
        // MSVC does not support C++11 and defaults to C++14.
        // The c++11 requirement is taken from the upstream woff2 build system.
        builder.std("c++11");
    }

    builder.compile("woff2");
}

fn main() {
    build_woff2()
}