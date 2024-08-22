fn main() {

    /*
    
    Communicate with cargo via stdout using println!
        - tell cargo where to look for the library we want to link
        - tell cargo which library we want to link
        - generate bindings to a file

    */
    
    println!("cargo:rustc-link-search=/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=m"); // m is libm.so, the 'lib' prefix and the extension are assumed.

    let bindings = bindgen::Builder::default()
        .header("/usr/include/math.h")
        .allowlist_function("sqrt") // let's not generate everything. Whitelist the sqrt function
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = "src/bindings.rs";
    bindings.write_to_file(bindings_path).expect("Failed to write bindings!");

}