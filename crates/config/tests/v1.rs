mod common;

use mcim_config::{ConfigV1, ConfigVersion, ModuleCategoryEnum, ModuleConfig, PackageConfig};
use std::path::PathBuf;

fn basic_package_config() -> PackageConfig {
    PackageConfig {
        id: "test-package".to_string(),
        name: "Test Package".to_string(),
        description: "A test package".to_string(),
        keywords: vec![],
        categories: vec![],
        repository: None,
        homepage: None,
        version: "0.1.0".to_string(),
        authors: vec![],
        license: Some("MIT".to_string()),
        license_file: None,
        readme: None,
        changelog: None,
        wasm: None,
        publish: vec![],
        build: None,
    }
}

fn basic_module_config(name: &str) -> ModuleConfig {
    ModuleConfig {
        name: name.to_string(),
        enabled: true,
    }
}

fn basic_config() -> ConfigV1 {
    ConfigV1 {
        manifest_version: 1,
        package: basic_package_config(),
        hook: vec![],
        server: vec![basic_module_config("my-server")],
        sandbox: vec![],
        interceptor: vec![],
    }
}

#[test]
fn test_validate_manifest_version() {
    let mut config = basic_config();

    config.manifest_version = 2;

    assert!(config.validate().is_err());

    config.manifest_version = 1;

    assert!(config.validate().is_ok());
}

#[test]
fn test_validate_has_modules() {
    let mut config = basic_config();

    config.server = vec![];

    assert!(config.validate().is_err());
}

#[test]
fn test_validate_unique_module_names() {
    let mut config = basic_config();

    config.sandbox = vec![basic_module_config("my-server")];

    assert!(config.validate().is_err());
}

#[test]
fn test_validate_license_specification() {
    let mut config = basic_config();

    config.package.license = None;

    assert!(config.validate().is_err());
}

macro_rules! test_validation {
    ($field:ident, $value:expr, $should_pass:expr) => {
        let mut config = basic_config();

        config.package.$field = $value;

        if $should_pass {
            assert!(config.validate().is_ok());
        } else {
            assert!(config.validate().is_err());
        }
    };
}

#[test]
fn test_package_config_validation() {
    test_validation!(id, "a".to_string(), false);
    test_validation!(id, "a".repeat(65), false);
    test_validation!(id, "Invalid-Name".to_string(), false);
    test_validation!(name, "a".to_string(), false);
    test_validation!(name, "a".repeat(65), false);
    test_validation!(description, "a".repeat(501), false);
    test_validation!(keywords, vec!["a".to_string(); 6], false);
    test_validation!(categories, vec![ModuleCategoryEnum::Server; 5], false);
    test_validation!(authors, vec!["<invalid>".to_string()], false);
    test_validation!(repository, Some("invalid-url".to_string()), false);
    test_validation!(homepage, Some("invalid-url".to_string()), false);
}

#[test]
fn test_path_exists_validation() {
    let mut config = basic_config();

    config.package.license_file = Some(PathBuf::from("non-existent-file"));

    assert!(config.validate().is_err());

    let dir = tempfile::tempdir().unwrap();

    config.package.license_file = Some(dir.path().to_path_buf());

    assert!(config.validate().is_err());
}

#[test]
fn test_defaults() {
    let config: ConfigV1 = toml::from_str(
        r#"
[package]
id = "test-package"
name = "Test Package"
license = "MIT"
[[server]]
name = "my-server"
"#,
    )
    .unwrap();
    assert_eq!(config.manifest_version, 1);
    assert_eq!(config.package.version, "0.0.0");
    assert!(config.server[0].enabled);
}
