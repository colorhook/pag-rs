use std::env;
use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let pag_dir =
        PathBuf::from(env::var("PAG_DIR").unwrap_or(format!("{}/../third_party/libpag", root)));
    let pag_headers = pag_dir.join("include");
    println!(
        "cargo:rustc-link-search=all={}",
        pag_dir.join("build").to_string_lossy()
    );
    println!("cargo:rustc-link-lib={}={}", "static", "pag-static");

    let path = std::path::PathBuf::from("src"); // include path
    let mut project = autocxx_build::Builder::new("src/lib.rs", &[&path, &pag_headers]).build()?;

    project
        .flag_if_supported("-std=c++14")
        .file("src/binding.cc")
        .compile("libpag-rs");

    if cfg!(target_os = "macos") {
        let frameworks = vec![
            "AppKit",
            "ApplicationServices",
            "Cocoa",
            "AudioToolbox",
            "AVFoundation",
            "CoreFoundation",
            "CoreGraphics",
            "CoreMedia",
            "CoreServices",
            "CoreVideo",
            "CoreText",
            "Foundation",
            "OpenCL",
            "OpenGL",
            "QTKit",
            "QuartzCore",
            "Security",
            "VideoDecodeAcceleration",
            "VideoToolbox",
        ];
        for f in frameworks {
            println!("cargo:rustc-link-lib=framework={}", f);
        }
        println!("cargo:rustc-link-lib=iconv");
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/binding.h");
    println!("cargo:rerun-if-changed=src/binding.cc");

    Ok(())
}
