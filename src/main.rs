use cargo_metadata::CargoOpt;
use cargo_metadata::Metadata;
use cargo_metadata::MetadataCommand;
use cargo_metadata::Package;
use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;
use walkdir::WalkDir;

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

        // println!("{}", fs::read_to_string(&krate.cargo_toml_path).unwrap());

        let metadata = MetadataCommand::new()
            .manifest_path(&krate.cargo_toml_path)
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

        let dependencies = vec![
            "lambda_http",
            "lambda_runtime",
            "tokio",
            env!("CARGO_PKG_NAME"),
        ]; // List of dependencies to check
        let has_required_info =
            has_binary_target_and_dependencies(&krate.name, &metadata, &dependencies);

        println!(
            "Has binary target and required dependencies: {}",
            has_required_info
        );
        println!("");

        // TODO
        // 1. Parse main.rs
        // 2. Parse the route attribute from main.rs
        // 3. Create a Vec<RouteMapping> with parsed routes
        // 4. Build each crate that has a valid mapping using `cargo lambda build --release --target TARGET_ARCHITECTURE`
        // 5. Deploy each lambda
        // 6. Configure AWS API Gateway mapping for each RouteMapping
    }
}

#[derive(Debug)]
pub struct RustCrate {
    pub name: String,
    pub main_rs_path: PathBuf,
    pub cargo_toml_path: PathBuf,
}

fn find_rust_crates<P: AsRef<Path>>(path: P) -> Vec<RustCrate> {
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

fn has_binary_target_and_dependencies(
    package_name: &str,
    metadata: &Metadata,
    dependencies: &[&str],
) -> bool {
    let package = metadata
        .packages
        .iter()
        .find_map(|package| {
            if package.name.eq(package_name) {
                Some(package)
            } else {
                None
            }
        })
        .unwrap();

    has_binary_target(package) && has_all_dependencies(package, dependencies)
}

fn has_binary_target(package: &Package) -> bool {
    package
        .targets
        .iter()
        .any(|target| target.kind.contains(&"bin".to_string()))
}

fn has_all_dependencies(package: &Package, dependencies: &[&str]) -> bool {
    let dep_names: HashSet<_> = package
        .dependencies
        .iter()
        .map(|dependency| dependency.name.as_str())
        .collect();

    println!(
        "PD:{}  vs D: {}",
        dep_names.iter().join(","),
        dependencies.iter().join(",")
    );
    dependencies.iter().all(|&dep| dep_names.contains(dep))
}
