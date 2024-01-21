use config::Config;
use std::collections::HashMap;
use shellexpand;

const CONFIG_FILE_NAME: &str = "~/.dt.yml";

#[derive(serde::Deserialize)]
pub struct Settings {
    root: String,
    editor: String,
    folders: HashMap<String, String>
}

#[derive(Debug)]
pub enum SettingsError {
    FolderNotFound
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SettingsError::FolderNotFound => write!(f, "Folder not found")
        }
    }
}

impl std::error::Error for SettingsError {}

impl Settings {
    pub fn load() -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name(&shellexpand::tilde(CONFIG_FILE_NAME)))
            .build()
            .unwrap();

        config
            .try_deserialize()
            .unwrap()
    }

    pub fn editor(&self) -> &str {
        &self.editor
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn get_folder_id(&self, folder_name: &str) -> Result<&str, SettingsError> {
        match self.folders.get(folder_name) {
            Some(folder) => Ok(folder),
            None => Err(SettingsError::FolderNotFound)
        }
    }
}
