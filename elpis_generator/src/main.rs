
use std::env;
use std::path::{Path, PathBuf};

use elpis_generator::CodeGenerator;

fn main() {
    let defs_path = match env::var("FFXIV_DEFINITIONS") {
        Ok(s) => PathBuf::new().join(s),
        _ => env::current_dir().unwrap().join("definitions"),
    };

    let generator = CodeGenerator::new(&defs_path).unwrap();
    let version = generator.fetch_version().unwrap();
    println!("{version}");

    let structs = generator.fetch_structs();
    for s in &structs {
        println!("{}", s);
    }
}