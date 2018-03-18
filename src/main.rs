//! `cargo arch` is a Cargo plugin for making Arch Linux packages.
//! Packages' information is extract from `Cargo.toml`.
//! You can add additional information in `[package.metadata.arch]` section.
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use serde_json::Value;

mod config;
use config::core::{CargoManifest, GeneratePackageConfig, PopulateFromCargoManifest};
use config::arch::PKGBUILDConfig;

fn get_cargo_manifest_path() -> String {
    let output = Command::new("cargo").arg("locate-project").output().unwrap();
    let json: Value = serde_json::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap();
    json["root"].as_str().unwrap().to_owned()
}

fn read_cargo_manifest() -> Result<CargoManifest> {
    let path = get_cargo_manifest_path();
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(CargoManifest::from_str(&contents))
}

fn write_pkgbuild(pkgbuild: &PKGBUILDConfig) -> Result<()> {
    let mut pkgbuild_file = File::create("pkgbuild")?;
    pkgbuild_file.write_all(pkgbuild.generate_config().as_bytes());
    Ok(())
}

fn main() {
    let cargo_manifest = read_cargo_manifest().unwrap();
    let pkgbuild_config = PKGBUILDConfig::from_cargo_manifest(cargo_manifest);
    write_pkgbuild(&pkgbuild_config);
}
