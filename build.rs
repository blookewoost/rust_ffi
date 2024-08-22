fn main() {

    println!("cargo:rustc-link-search=/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=m");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/math.h")
        .allowlist_function("sqrt")
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = "src/bindings.rs";
    bindings.write_to_file(bindings_path).expect("Failed to write bindings!");

}