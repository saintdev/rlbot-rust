extern crate bindgen;

use std::{
    env,
    path::{Path, PathBuf},
};

fn generate_ffi() {
    let rlbot_include_dirs = vec![
        "src/main/cpp/RLBotInterface/src/RLBotInterface",
        "src/main/cpp/RLBotInterface/src/RLBotMessages",
        "src/generated/cpp/flatbuffers",
    ];

    let rlbot_root_dir = PathBuf::from(env::var("RLBOT_DIR").unwrap_or("../RLBot".into()).as_str());

    let mut clang_args: Vec<String> = "-fdeclspec -x c++ -std=c++17"
        .split(" ")
        .into_iter()
        .map(String::from)
        .collect();

    let mut rlbot_includes: Vec<String> = rlbot_include_dirs
        .into_iter()
        .map(|s| format!("-I{}", rlbot_root_dir.join(s).to_str().unwrap()))
        .collect();

    let header = Path::new("cpp/rlbot.hpp").to_str().unwrap();

    clang_args.append(rlbot_includes.as_mut());

    let bindings = bindgen::Builder::default()
        .header(header)
        .disable_name_namespacing()
        .raw_line("#![allow(non_camel_case_types, non_snake_case, missing_docs)]")
        .whitelist_type("ByteBuffer")
        .whitelist_type("RLBotCoreStatus")
        .whitelist_type("BallPredictionPacket")
        .whitelist_type("MatchSettings")
        .whitelist_type("FieldInfo")
        .whitelist_type("LiveDataPacket")
        .whitelist_type("RigidBodyTick")
        .whitelist_type("QuickChatPreset")
        .derive_default(true)
        .layout_tests(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .clang_args(clang_args.into_iter())
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(Path::new("src/ffi.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    if let Some(_) = env::var_os("CARGO_FEATURE_GENERATE_FFI") {
        generate_ffi();
    }
}
