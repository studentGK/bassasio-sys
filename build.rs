fn main() {
    // Tell cargo to link against bassasio.lib.
    // The .lib file sits two levels up from this crate (next to the C project files).
    // Adjust the path if your directory layout differs.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = std::path::Path::new(&manifest_dir).parent().unwrap().to_path_buf();

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=bassasio");

    // Re-run this script only if build.rs itself changes.
    println!("cargo:rerun-if-changed=build.rs");
}
