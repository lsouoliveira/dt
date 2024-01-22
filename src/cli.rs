use clap::Parser;
use std::ffi::OsString;

#[derive(Parser)]
pub struct Cli {
    /// Custom command to run
    pub command: Option<String>,

    /// Fetch and pull changes from remote
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub sync: bool,

    /// Reload dotfiles (needs to be implemented as a custom command)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub reload: bool,

    /// Open the dotfiles (needs to be implemented as a custom command)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub open: bool
}

impl Cli {
    pub fn parse<I, T>(itr: I) -> Self
    where
      I: IntoIterator<Item = T>,
      T: Into<OsString> + Clone,
    {
        Cli::parse_from(itr)
    }
}
