use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct ControlIds {
    controls: Vec<BTreeMap<String, Control>>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Control {
    /// The C++ type identifier for the value stored in this control.
    #[serde(alias = "type")]
    typ: String,

    description: String,

    #[serde(alias = "enum")]
    enum_values: Option<Vec<EnumValue>>,

    size: Option<Vec<serde_yaml::Value>>,

    /// TODO: Handle this appropriately
    #[serde(default)]
    draft: bool,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct EnumValue {
    name: String,
    value: u32,
    description: String,
}

fn compile_controls(out_dir: &Path) {
    let control_ids: ControlIds = {
        let yaml = std::fs::read_to_string("data/control_ids.yaml").unwrap();
        serde_yaml::from_str(&yaml).unwrap()
    };

    let mut out = String::new();

    for control in control_ids.controls {
        assert_eq!(control.len(), 1);
        let (control_name, control) = control.first_key_value().unwrap();

        let (primitive_type, enum_allowed) = match control.typ.as_str() {
            "int32_t" => ("i32", true),
            "int64_t" => ("i64", true),
            "bool" => ("bool", true),
            "float" => ("f32", false),
            "Rectangle" => ("Rectangle", false),
            "Size" => ("Size", false),
            _ => panic!("Unsupported control type: {}", control.typ),
        };

        let mut control_type = primitive_type.to_string();

        // TODO: We should also implement a fancy printer for enum values by name.
        if let Some(enum_values) = &control.enum_values {
            assert!(
                enum_allowed,
                "Control type {} not allowed to be an enum",
                control.typ
            );

            control_type = format!("{}Enum", control_name);

            out.push_str(&format!(
                "control_enum!({}Enum {} {{\n",
                control_name, primitive_type
            ));

            for value in enum_values {
                out.push_str(&format!("    {} = {},\n", value.name, value.value));
            }

            out.push_str("});\n\n");
        }

        if let Some(dims) = &control.size {
            let mut is_static_size = true;
            let mut size = 1;
            for dim in dims {
                match dim {
                    serde_yaml::Value::Number(n) => {
                        size *= n.as_u64().unwrap();
                    }
                    serde_yaml::Value::String(_) => {
                        is_static_size = false;
                        break;
                    }
                    _ => panic!("Unexpected dimension type in control size: {:?}", dim),
                }
            }

            if is_static_size {
                control_type = format!("[{}]", control_type);
            } else {
                control_type = format!("[{}; {}]", control_type, size);
            }
        }

        let namespace = if control.draft { "draft" } else { "stable" };

        out.push_str(&format!(
            "control!({}, {}, {});\n\n",
            control_name, control_type, namespace
        ));

        // Handle array

        // out.push_str(&format!(
        //     "pub const {name}: Control<T> = Control::new({id}, \"{name}\"",
        //     name = control_name,
        //     id = control
        // ));
    }

    std::fs::write(out_dir.join("controls.rs"), out).unwrap();
}

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    compile_controls(&out_path);

    // todo!();

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
        .allowlist_type("libcamera.*ControlType")
        .allowlist_type("libcamera.*Rectangle")
        .allowlist_type("libcamera.*ControlValuePrimitive")
        .allowlist_var("libcamera.*formats.*")
        .allowlist_var("libcamera.*controls.*")
        .opaque_type("libcamera.*Control.*")
        .no_debug("libcamera.*PixelFormat")
        .enable_cxx_namespaces()
        .blocklist_function(".*")
        .clang_args([
            "-x",
            "c++",
            "-std=c++2a",
            "-I/usr/aarch64-linux-gnu/include/c++/9",
            "-I/usr/aarch64-linux-gnu/include/c++/9/aarch64-linux-gnu",
            "-I/usr/include/libcamera",
            "--target=aarch64-linux-gnu",
        ])
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // println!("{:?}", out_path.join("bindings.rs"));
    // todo!();

    cxx_build::bridge("src/ffi.rs") // returns a cc::Build
        .file("src/ffi.cc")
        .include("/usr/include/libcamera/")
        .flag("-std=c++2a")
        .flag("-Wl,--unresolved-symbols=ignore-in-shared-libs")
        .compile("libcamera-cxx");

    println!("cargo:rustc-link-arg=-Wl,--unresolved-symbols=ignore-in-shared-libs");

    println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu/");

    println!("cargo:rustc-link-lib=dylib=camera");
    println!("cargo:rustc-link-lib=dylib=camera-base");

    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=src/wrappers.cc");
    println!("cargo:rerun-if-changed=src/wrappers.h");
}
