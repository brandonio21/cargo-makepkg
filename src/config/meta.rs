//! Metadata for different platform's package

use config::arch::CargoPKGBUILDManifest;

/// data in `[package.metadata]` section
#[derive(Deserialize, Default)]
pub struct CargoMetadata {
    pub archlinux_pkgbuild: Option<CargoPKGBUILDManifest>,
}
