mod common;
use crate::common::create_temp_config_file;
use mcim_config::load_config;
use std::path::Path;

#[test]
fn test_load_config_success() {
    let content = r#"
manifest_version = 1
[package]
id = "test-package"
name = "Test Package"
version = "0.1.0"
license = "MIT"
[[server]]
name = "my-server"
"#;
    let temp_file = create_temp_config_file(content);
    let schema_path = temp_file.path().parent().unwrap().join("schema.json");

    std::fs::write(&schema_path, "{}").unwrap();

    let config = load_config(temp_file.path()).unwrap();

    assert_eq!(config.package.id, "test-package");
    assert_eq!(config.package.name, "Test Package");
}

#[test]
fn test_load_config_file_not_found() {
    let result = load_config(Path::new("non_existent_file.toml"));

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to read configuration file")
    );
}

#[test]
fn test_load_config_invalid_toml() {
    let content = "this is not toml";
    let temp_file = create_temp_config_file(content);
    let result = load_config(temp_file.path());

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Failed to parse TOML configuration")
    );
}

#[test]
fn test_load_config_validation_error() {
    let content = r#"
manifest_version = 1
[package]
id = "test-package"
name = "Test Package"
version = "0.1.0"
license = "MIT"
[[server]]
name = "my-server"
[[server]]
name = "my-server" # duplicate name
"#;
    let temp_file = create_temp_config_file(content);
    let schema_path = temp_file.path().parent().unwrap().join("schema.json");

    std::fs::write(&schema_path, "{}").unwrap();

    let result = load_config(temp_file.path());

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Validation error"));
}
