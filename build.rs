fn main() {
    // Skip compilation on docs.rs to avoid build failures.
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:rustc-cfg=docsrs");
        println!("cargo:rerun-if-env-changed=DOCS_RS");
        return;
    }
    // Initialize C++ build configuration.
    let mut build = cxx_build::bridge("src/lib.rs");

    // Add local C++ headers for CXX declarations.
    build.include("cxx");

    // Locate vf3lib headers with the following priority:
    // 1. VF3LIB_DIR environment variable (for custom locations)
    // 2. Vendored copy at vendor/vf3lib (default)
    let vf3_root = if let Ok(custom_dir) = std::env::var("VF3LIB_DIR") {
        if !std::path::Path::new(&custom_dir)
            .join("include/VFLib.h")
            .exists()
        {
            panic!(
                "VF3LIB_DIR set but {}/include/VFLib.h not found",
                custom_dir
            );
        }
        custom_dir
    } else if std::path::Path::new("vendor/vf3lib/include/VFLib.h").exists() {
        "vendor/vf3lib".to_string()
    } else {
        panic!(
            "Could not find vf3lib headers. Set VF3LIB_DIR or ensure vendor/vf3lib/include exists."
        );
    };
    let include_dir = std::path::Path::new(&vf3_root).join("include");
    println!("cargo:rerun-if-env-changed=VF3LIB_DIR");
    println!("cargo:rerun-if-changed={}", include_dir.display());
    build.include(include_dir);

    // Single translation unit prevents duplicate symbols from header-only templates.
    build
        .file("cxx/vf3_bridge.cc")
        .flag_if_supported("-std=c++14");

    // Suppress warnings from vendored C++ headers.
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    if env != "msvc" {
        for flag in [
            "-Wno-deprecated",
            "-Wno-deprecated-declarations",
            "-Wno-cpp",
            "-Wno-unused-parameter",
            "-Wno-reorder",
            "-Wno-sign-compare",
            "-Wno-unused-variable",
            "-Wno-unused-but-set-variable",
            "-Wno-delete-non-virtual-dtor",
        ] {
            build.flag_if_supported(flag);
        }
    }

    // Platform-specific linking requirements.
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target == "linux" {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=atomic");
        println!("cargo:rustc-link-lib=pthread");
    } else if target == "macos" {
        println!("cargo:rustc-link-lib=c++");
    }

    build.compile("vf3bridge");
}
