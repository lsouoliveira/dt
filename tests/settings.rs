use dt::settings::Settings;
use assert_fs::prelude::*;

#[test]
fn test_load() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  test: echo")?;

    let result = Settings::load(file.path().to_str().unwrap());

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_load_invalid_file() -> Result<(), Box<dyn std::error::Error>> {
    let result = Settings::load("/tmp/invalid.yml");

    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_root() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  test: echo")?;

    let settings = Settings::load(file.path().to_str().unwrap())?;

    assert_eq!(settings.root(), "/tmp");

    Ok(())
}

#[test]
fn test_get_command() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  test: echo\n  test2: echo2")?;

    let settings = Settings::load(file.path().to_str().unwrap())?;

    assert_eq!(settings.get_command("test")?, "echo");
    assert_eq!(settings.get_command("test2")?, "echo2");

    Ok(())
}

#[test]
fn test_get_command_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("dt.yml")?;
    file.write_str("root: /tmp\ncommands:\n  test: echo")?;

    let settings = Settings::load(file.path().to_str().unwrap())?;

    let result = settings.get_command("test2");

    assert!(result.is_err());

    Ok(())
}
