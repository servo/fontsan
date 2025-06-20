use std::path::Path;

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

    cc::Build::new()
        .cpp(true)
        .files(woff2_sources)
        .include("include")
        .std("c++11")
        .warnings(false)
        .compile("woff2");
}

fn main() {
    build_woff2()
}