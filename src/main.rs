use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct RustCrate {
    pub name: String,
    pub main_rs_path: PathBuf,
    pub cargo_toml_path: PathBuf,
}

pub fn find_rust_crates<P: AsRef<Path>>(path: P) -> Vec<RustCrate> {
    let mut rust_crates = vec![];

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().ends_with("Cargo.toml"))
    {
        if let Ok(cargo_toml) = fs::read_to_string(entry.path()) {
            if let Ok(value) = cargo_toml.parse::<Value>() {
                if let Some(crate_name) = value.get("package").and_then(|p| p.get("name")) {
                    let cargo_toml_path = entry.into_path();
                    let src_dir = cargo_toml_path.parent().unwrap().join("src");
                    let main_rs_path = src_dir.join("main.rs");

                    if main_rs_path.exists() {
                        rust_crates.push(RustCrate {
                            name: crate_name.as_str().unwrap().to_string(),
                            main_rs_path,
                            cargo_toml_path,
                        });
                    }
                }
            }
        }
    }

    rust_crates
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        return;
    }

    let path = &args[1];
    let crates = find_rust_crates(path);
    for krate in &crates {
        println!("{:?}", krate);
    }
}
