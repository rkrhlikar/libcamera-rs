use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .header("src/wrapper.h")
        .newtype_enum(".*")
        .allowlist_type("libcamera.*StreamRole")
        .allowlist_type("libcamera.*Request.*Status")
        .allowlist_type("libcamera.*Request.*ReuseFlag")
        .allowlist_type("libcamera.*FrameMetadata.*Status")
        .allowlist_type("libcamera.*FrameMetadata.*Plane")
        .allowlist_type("libcamera.*CameraConfiguration.*Status")
        .allowlist_type("libcamera.*PixelFormat")
        .allowlist_type("libcamera.*SizeRange")
        .allowlist_type("libcamera.*Size")
        .blocklist_function(".*")
        .clang_args(["-x", "c++", "-std=c++2a"])
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cxx_build::bridge("src/ffi.rs") // returns a cc::Build
        .file("src/ffi.cc")
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
