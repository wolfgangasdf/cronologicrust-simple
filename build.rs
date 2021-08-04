
// https://rust-lang.github.io/rust-bindgen/tutorial-3.html

extern crate bindgen;

use std::path::PathBuf;

fn handlecrono(folder: &str, libname: &str, genbindings: bool, iscpp: bool) {
    // link to cronologic tt4 or hptdc library
    println!("cargo:rustc-link-search=./{}", folder);
    println!("cargo:rustc-link-lib={}", libname); // can't use paths here

    if genbindings { // create wrapper files
        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed=wrapper.h");

        let mut bgb = bindgen::Builder::default()
        .header(format!("{}/wrapper.h", folder))
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false);
        if iscpp { // bindgen for cpp is a mess, selective enable it for needed types.
            bgb = bgb.clang_arg("-x").clang_arg("c++")
            .allowlist_recursively(false)
            .allowlist_type("TDC.*").allowlist_type("DWORD").allowlist_type("USHORT").allowlist_type("Config")
            .allowlist_type("Frame").allowlist_type("HIT")
            .blocklist_function("*PrettyPrint*") // bindgen error
        }
        let bindings = bgb.generate().expect("Unable to generate bindings");

        // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(format!("{}.rs", folder));
        let out_path = PathBuf::from(format!("src/crono-bindgen/{}.rs", folder));
        println!("cargo:warning=saving to {:?}", out_path); // the only way to debug output in bindgen
        bindings.write_to_file(out_path).expect("Couldn't write bindings!");
    }
}


fn main() {
    let genbindings = false; // set to true to re-generate the bindings!
    handlecrono("crono_tt4", "xtdc4_driver_64", genbindings, false);
    handlecrono("crono_hptdc", "hptdc_driver_3.9.3_x64", genbindings, true);
}



