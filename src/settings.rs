use config::Config;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct Settings {
    root: String,
    commands: HashMap<String, String>
}

#[derive(Debug)]
pub enum SettingsError {
    CommandNotFound,
    ParseError
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SettingsError::CommandNotFound => write!(f, "Command not found"),
            SettingsError::ParseError => write!(f, "Parse error")
        }
    }
}

impl std::error::Error for SettingsError {}

impl Settings {
    pub fn load(path: &str) -> Result<Self, SettingsError> {
        let config = Config::builder()
            .add_source(config::File::with_name(path))
            .build()
            .map_err(|_| SettingsError::ParseError)?;

        config
            .try_deserialize()
            .map_err(|_| SettingsError::ParseError)
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn get_command(&self, command: &str) -> Result<&str, SettingsError> {
        match self.commands.get(command) {
            Some(command) => Ok(command),
            None => Err(SettingsError::CommandNotFound)
        }
    }
}
