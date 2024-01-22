use shellexpand;

use crate::settings::Settings;
use crate::commands;
use crate::cli::Cli;

const CONFIG_FILE_NAME: &str = "~/.dt.yml";

pub struct Application {
    cli: Cli,
    settings: Settings
}

impl Application {
    pub fn init() -> Self {
        let cli = Cli::parse(std::env::args());
        let settings = Settings::load(&shellexpand::tilde(CONFIG_FILE_NAME))
            .unwrap_or_else(|_| {
                eprintln!("Error: invalid config file");
                std::process::exit(1);
            });

        Self {
            cli,
            settings
        }
    }

    pub fn run(&self) {
        if self.cli.command.is_none() && !self.are_flags_present() {
            eprintln!("Error: no command specified");
            std::process::exit(1);
        }

        if self.cli.command.is_none() {
            self.handle_flags();
        } else {
            self.run_command(self.cli.command.as_ref().unwrap());
        }
    }

    fn are_flags_present(&self) -> bool {
        self.cli.sync || self.cli.reload || self.cli.open
    }

    fn handle_flags(&self) {
        if self.cli.sync {
            let _ = commands::sync(self.settings.root());
        } else if self.cli.reload {
            self.run_command("reload"); 
        } else if self.cli.open {
            self.run_command("open");
        } else {
            eprintln!("Error: no command specified");
            std::process::exit(1);
        }
    }

    fn run_command(&self, command: &str) {
        let command = self.settings.get_command(command).unwrap_or_else(|_| {
            eprintln!("Error: command {:?} was not found", command);
            std::process::exit(1);
        });

        commands::run(command).unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });
    }
}
