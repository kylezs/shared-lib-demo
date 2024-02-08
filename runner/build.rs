fn main() {
    // Add search path for the libraries
    // println!("cargo:rustc-link-search=native=/Users/kylezs/Documents/cf-repos/shared-lib-demo/target/debug");

    // Link the shared libraries
    println!("cargo:rustc-link-lib=dylib=first_shared_lib");
}
