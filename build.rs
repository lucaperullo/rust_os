use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("linker.ld");
    fs::write(&dest_path, include_str!("linker.ld")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.to_string_lossy());
}