use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use elpis_generator::CodeGenerator;

fn main() {
    let defs_path = match ::std::env::var("FFXIV_DEFINITIONS") {
        Ok(s) => PathBuf::new().join(s),
        _ => ::std::env::current_dir().unwrap().join("definitions"),
    };

    if !defs_path.exists() || defs_path.is_file() {
        panic!("Failed to find xiv type definitions");
    }

    let generator = CodeGenerator::new(&defs_path).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let hello_path = Path::new(&out_dir).join("hello.rs");
    generate_file(&hello_path, b"
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ");

    let sheets_path = Path::new(&out_dir).join("sheets.rs");
    generate_file(&sheets_path, b"include!(concat!(env!(\"OUT_DIR\"), \"/hello.rs\"));");

    let gen_dir = Path::new(&out_dir).join("gen");
    if !gen_dir.exists() {
        fs::create_dir(&gen_dir).unwrap();
    }
    let hello2_path = gen_dir.join("hello2.rs");
    generate_file(&hello2_path, b"
        pub fn message2() -> &'static str {
            \"Hello, World!\"
        }
    ");
    println!("cargo:rustc-env=GENERATED_ENV={}", gen_dir.display());

    println!("cargo:rustc-cfg=has_generated_feature")
}

fn generate_file<P: AsRef<Path>>(path: P, text: &[u8]) {
    let mut f = File::create(path).unwrap();
    f.write_all(text).unwrap()
}