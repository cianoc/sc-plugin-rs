extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

   let bindings = bindgen::Builder::default()
        .header("plugin.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_type("Unit")
        .clang_arg("-I./include/plugin_interface/")
        .clang_arg("-I./include/common/")
        .clang_arg("-I./include/server")
         .clang_arg("-xc++")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");



}