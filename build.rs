extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("include/wrapper.hpp")
        .clang_arg("--include-directory=include")
        .clang_arg("--include-directory=include/third_party/abseil-cpp")
        .clang_arg("-DWEBRTC_WIN")
        .clang_arg("-DNOMINMAX")
        .clang_arg("-DWEBRTC_USE_BUILTIN_ISAC_FLOAT")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}