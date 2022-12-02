use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    if target.starts_with("thumbv") {
        if env::var_os("CARGO_FEATURE_INLINE_ASM").is_none() {
            fs::copy(
                format!("bin/{}.a", target),
                out_dir.join(format!("lib{}.a", name)),
            ).unwrap();

            println!("cargo:rustc-link-lib=static={}", name);
            println!("cargo:rustc-link-search={}", out_dir.display());
        }

        println!("cargo:rustc-cfg=thumb");
    }
}
