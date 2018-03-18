//! Basic Rust package's config, modified from Cargo.
use config::meta::CargoMetadata;
use toml;


/// data in Cargo.toml
#[derive(Deserialize)]
pub struct CargoManifest {
    pub package: CargoPackage,
}

impl CargoManifest {
    pub fn from_str(contents: &str) -> CargoManifest {
        toml::from_str(&contents)
            .expect("Could not decode CargoManifest text")
    }
}

/// data in `[package]` section
#[derive(Deserialize)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub license: String,    // Multiple licenses are separated by `/`
    pub readme: String,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub metadata: Option<CargoMetadata>,
}

pub trait PopulateFromCargoManifest<T> {
    fn from_cargo_manifest(cargo_manifest: CargoManifest) -> T;
}

/// A trait for generate specific platform package's config
pub trait GeneratePackageConfig {
    fn generate_config(&self) -> String;
}
