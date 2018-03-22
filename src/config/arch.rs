//! Arch Linux's package config
use config::core::{CargoManifest, PopulateFromCargoManifest, GeneratePackageConfig};
use config::meta::CargoMetadata;

#[cfg(test)]
use config::core::CargoPackage;


/// data in `[package.metadata.archlinux_pkgbuild]` section, which the user can
/// provide for PKGBUILD specific overrides
#[derive(Deserialize, Default)]
pub struct CargoPKGBUILDManifest {
    /// The maintainers of the package
    pub maintainers: Option<Vec<String>>,
    /// The name of the package.
    pub pkgname: Option<String>,
    /// The version of the software as released from the author.
    pub pkgver: Option<String>,
    /// This is the release number specific to the Arch Linux release.
    pub pkgrel: Option<u32>,
    /// This should be a brief description of the package and its functionality.
    pub pkgdesc: Option<String>,
    /// This field contains a URL that is associated with the software being packaged.
    /// This is typically the project’s web site.
    pub url: Option<String>,
}

/// see `man PKGBUILD`
/// and https://wiki.archlinux.org/index.php/PKGBUILD
pub struct PKGBUILDConfig {
    /// The maintainers of the package
    pub maintainers: Vec<String>,
    /// The name of the package.
    pub pkgname: String,
    /// The version of the software as released from the author.
    pub pkgver: String,
    /// This is the release number specific to the Arch Linux release.
    pub pkgrel: u32,
    /// This should be a brief description of the package and its functionality.
    pub pkgdesc: String,
    /// This field contains a URL that is associated with the software being packaged.
    /// This is typically the project’s web site.
    pub url: String,
}


impl PopulateFromCargoManifest<PKGBUILDConfig> for PKGBUILDConfig {
    /// Generate a PKGBUILDConfig object from a Cargo manifest. This involves taking
    /// information directly from the Cargo manifest (where applicable) and translating
    /// to PKGBUILD. The user may have also provided additional metadata in Cargo.toml
    fn from_cargo_manifest(cargo_manifest: CargoManifest) -> PKGBUILDConfig {
        // Since the additional metadata in Cargo.toml is all optional,
        // we use it as a starting point.
        let override_pkgbuild_config = cargo_manifest.package.metadata
            .unwrap_or(CargoMetadata::default())
            .archlinux_pkgbuild
            .unwrap_or(CargoPKGBUILDManifest::default());

        let maintainers = override_pkgbuild_config.maintainers.unwrap_or(
            cargo_manifest.package.authors
        );
        let pkgname = override_pkgbuild_config.pkgname.unwrap_or(
            cargo_manifest.package.name
        );
        let pkgver = override_pkgbuild_config.pkgver.unwrap_or(
            cargo_manifest.package.version
        );
        let pkgrel = override_pkgbuild_config.pkgrel.unwrap_or(1);
        let pkgdesc = override_pkgbuild_config.pkgdesc.unwrap_or(
            cargo_manifest.package.description
        );
        let url = override_pkgbuild_config.url.unwrap_or(
            cargo_manifest.package.homepage.unwrap_or(
                cargo_manifest.package.repository.unwrap_or("".to_string())
            )
        );

        PKGBUILDConfig {
            maintainers: maintainers,
            pkgname: pkgname,
            pkgver: pkgver,
            pkgrel: pkgrel,
            pkgdesc: pkgdesc,
            url: url,
        }
    }
}

#[test]
fn test_from_cargo_manifest() {
    let manifest = CargoManifest {
        package: CargoPackage {
            name: "Test".to_string(),
            version: "test".to_string(),
            description: "test description".to_string(),
            authors: vec!["foobar".to_string(), "foobaz".to_string()],
            license: "BSD".to_string(),
            readme: "README.md".to_string(),
            homepage: None,
            documentation: None,
            repository: None,
            keywords: None,
            metadata: None,
        }
    };
    let pkgbuild = PKGBUILDConfig::from_cargo_manifest(manifest);
    assert_eq!(pkgbuild.maintainers,
               vec!["foobar".to_string(), "foobaz".to_string()]);
    assert_eq!(pkgbuild.pkgname, "Test".to_string());
    assert_eq!(pkgbuild.pkgver, "test".to_string());
    assert_eq!(pkgbuild.pkgrel, 1);
    assert_eq!(pkgbuild.pkgdesc, "test description".to_string());
    assert_eq!(pkgbuild.url, "".to_string());
}

impl GeneratePackageConfig for PKGBUILDConfig {
    fn generate_config(&self) -> String {
        let mut buffer = String::new();

        for maintainer in &self.maintainers {
            buffer.push_str(format!("# Maintainer: {}\n", maintainer).as_str());
        }
        let contents = format!(r#"pkgname={}
pkgver={}
pkgrel={}
pkgdesc="{}"
url="{}""#,
        &self.pkgname, &self.pkgver, &self.pkgrel, &self.pkgdesc, &self.url);
        buffer.push_str(contents.as_str());
        buffer
   }
}

#[test]
fn test_generate_config() {
    let pkgbuild_config = PKGBUILDConfig {
        maintainers: vec!["foobar".to_string(), "foobaz".to_string()],
        pkgname: "Test".to_string(),
        pkgver: "1.0".to_string(),
        pkgrel: 5,
        pkgdesc: "Test package".to_string(),
        url: "test.com".to_string(),
    };
    let expected_result = r#"# Maintainer: foobar
# Maintainer: foobaz
pkgname=Test
pkgver=1.0
pkgrel=5
pkgdesc="Test package"
url="test.com""#.to_string();
    assert_eq!(pkgbuild_config.generate_config(), expected_result);

}
