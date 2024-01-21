use crate::settings::Settings;
use crate::commands;
use crate::cli::Cli;

pub struct Application {
    cli: Cli,
    settings: Settings
}

impl Application {
    pub fn init() -> Self {
        let cli = Cli::parse();
        let settings = Settings::load();

        Self {
            cli,
            settings
        }
    }

    pub fn run(&self) {
        if self.cli.folder.is_none() && !self.are_flags_present() {
            eprintln!("Error: no folder specified");
            std::process::exit(1);
        }

        if self.cli.folder.is_none() {
            self.handle_flags();
        } else {
            self.open_folder();
        }
    }

    fn are_flags_present(&self) -> bool {
        self.cli.sync
    }

    fn handle_flags(&self) {
        if self.cli.sync {
            let _ = commands::sync(self.settings.root());
        } else {
            eprintln!("Error: no folder specified");
            std::process::exit(1);
        }
    }

    fn open_folder(&self) {
        let editor_command = self.settings.editor();
        let folder_id = self.cli.folder.as_ref().unwrap();

        let folder_path = self.settings.get_folder_id(folder_id).unwrap_or_else(|_| {
            eprintln!("Error: folder with id {} not found", folder_id);
            std::process::exit(1);
        });

        commands::open_editor(editor_command, folder_path).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
    }
}
