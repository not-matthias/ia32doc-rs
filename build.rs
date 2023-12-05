use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=ia32.hpp");

    let bindings = bindgen::Builder::default()
        .header("ia32.hpp")
        .clang_arg("-xc++")
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
