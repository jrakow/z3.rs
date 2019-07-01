use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=z3");
    println!("cargo:rustc-link-search=native=/home/jrakow/.local/lib");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .prepend_enum_name(false)
        .default_enum_style(bindgen::EnumVariation::Rust)
        .blacklist_item("Z3_TRUE")
        .blacklist_item("Z3_FALSE")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
