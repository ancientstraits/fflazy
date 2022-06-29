extern crate bindgen;

use std::env;
use std::path::PathBuf;

const FFLIBS: [&str; 6] = [
    "avutil",
    "avcodec",
    "avformat",
    "avfilter",
    "swscale",
    "swresample",
];

const CARGO_FLAGS: &str = "\
cargo:rustc-link-search=/usr/lib
cargo:rustc-link-search=/usr/local/lib
cargo:rustc-link-lib=avcodec
cargo:rerun-if-changed=wrapper.h
";

fn main() {
    println!("{}", CARGO_FLAGS);

    for lib in FFLIBS {
        let bindings = bindgen::Builder::default()
            .header_contents(&format!("bindgen_{}.h", lib)[..],
                &format!("#include <lib{0}/{0}.h>", lib)[..])
            .clang_arg("-I/usr/local/include")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings");

        let out = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out.join(&format!("bindgen_{}.rs", lib)[..]))
            .expect("Could not write bindings");
    }

/*
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/local/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Could not write bindings");
*/
}

