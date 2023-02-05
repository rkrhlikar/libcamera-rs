fn main() {
    cxx_build::bridge("src/ffi.rs") // returns a cc::Build
        .file("src/wrappers.cc")
        .include("/usr/local/include/libcamera/")
        .flag("-std=c++2a")
        .compile("libcamera-cxx");

    println!("cargo:rustc-link-search=native=/usr/local/lib/x86_64-linux-gnu/");

    println!("cargo:rustc-link-lib=dylib=camera");
    println!("cargo:rustc-link-lib=dylib=camera-base");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/wrappers.cc");
    println!("cargo:rerun-if-changed=src/wrappers.h");
}

