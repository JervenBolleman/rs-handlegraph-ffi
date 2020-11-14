extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir1 = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_dir2 = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir1)
      .with_language(cbindgen::Language::C)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("libhandlegraph-ffi-c.h");
    
    cbindgen::Builder::new()
      .with_crate(crate_dir2)
      .with_language(cbindgen::Language::Cxx)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("libhandlegraph-ffi-cxx.h");
}
