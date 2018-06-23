extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    // Find the library with pkg-config
    let lib = pkg_config::probe_library("powercap").unwrap();

    // Must get the include path(s) separately
    let mut args = Vec::with_capacity(lib.include_paths.len());
    for d in &lib.include_paths {
        args.push(format!("-I{}", d.to_string_lossy()));
    }
    let builder = bindgen::Builder::default().header("wrapper.h").clang_args(args);

    // Generate bindings
    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
