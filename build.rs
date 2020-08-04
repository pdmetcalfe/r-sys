use bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system R
    // shared library.
    println!("cargo:rustc-link-lib=R");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let r_include = env::var("R_INCLUDE_DIR")
	.expect("unable to find R_INCLUDE_DIR");
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
	.clang_arg("-I")
	.clang_arg(r_include)
	.whitelist_function("R[f]?_.*")
	.whitelist_var(".*SXP")
	.blacklist_function("R_.*PStream")
	.rustified_enum("Rboolean")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
