//! `cargo arch` is a Cargo plugin for making Arch Linux packages.
//! Packages' information is extract from `Cargo.toml`.
//! You can add additional information in `[package.metadata.arch]` section.
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

mod config;
use config::core::{CargoManifest, GeneratePackageConfig, PopulateFromCargoManifest};
use config::arch::PKGBUILDConfig;

fn get_cargo_manifest_path() -> String {
    let output = Command::new("cargo").arg("locate-project").output()
        .expect("Failed to call `cargo locate-project`");

    let unicode_output = String::from_utf8(output.stdout)
        .expect("Failed to convert output of `cargo locate-project` to unicode");

    let json: serde_json::Value = serde_json::from_str(&unicode_output)
        .expect("Failed to parse `cargo locate-project` output to JSON");

    return json["root"].as_str()
        .expect("Failed to convert `cargo locate-project` root path to str")
        .to_owned();
}

fn read_cargo_manifest(manifest_path: &str) -> CargoManifest {
    let mut file = File::open(manifest_path)
        .expect(format!("Failed to open Cargo.toml at path {}", manifest_path).as_str());

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read Cargo.toml");
    CargoManifest::from_str(&contents)
}

fn get_pkgbuild_directory(
    cargo_manifest_path: &str,
    pkgbuild: &PKGBUILDConfig,
) -> PathBuf {
    let cargo_path = Path::new(cargo_manifest_path);
    let root_dir = cargo_path.parent()
        .expect(format!("Could not get parent of manifest path {}", cargo_manifest_path).as_str());
    let version_str = format!("{}-{}", &pkgbuild.pkgver, &pkgbuild.pkgrel);
    let pkgbuild_dir = root_dir.join("target").join(version_str);

    // Finally, create the dir
    if !pkgbuild_dir.exists() {
        create_dir_all(&pkgbuild_dir).expect("Could not create pkgbuild dir");
    }
    pkgbuild_dir
}

fn write_pkgbuild(pkgbuild_dir: &Path, pkgbuild: &PKGBUILDConfig) {
    let pkgbuild_file_path = pkgbuild_dir.join("PKGBUILD");
    let mut pkgbuild_file = File::create(&pkgbuild_file_path)
        .expect("Could not create PKGBUILD file");
    pkgbuild_file.write_all(pkgbuild.generate_config().as_bytes())
        .expect("Could not write PKGBUILD file");
}

fn run_makepkg(pkgbuild_dir: &Path) {
    let mut makepkg_proc = Command::new("makepkg").current_dir(&pkgbuild_dir).spawn()
        .expect("Failed to execute `makepkg`");
    let exitcode = makepkg_proc.wait().expect("Failed to wait on makepkg process");
    if !exitcode.success() {
        match exitcode.code() {
            Some(code) => panic!("makepkg failed with exit code {}", code),
            None => panic!("makepkg failed."),
        }
    }
}

fn main() {
    let cargo_manifest_path = get_cargo_manifest_path();
    let cargo_manifest = read_cargo_manifest(&cargo_manifest_path);
    let pkgbuild_config = PKGBUILDConfig::from_cargo_manifest(cargo_manifest);
    let pkgbuild_dir = get_pkgbuild_directory(&cargo_manifest_path, &pkgbuild_config);
    write_pkgbuild(&pkgbuild_dir.as_path(), &pkgbuild_config);
    run_makepkg(&pkgbuild_dir.as_path());
}
