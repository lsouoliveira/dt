use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// File to open
    pub folder: Option<String>,

    /// Fetch and pull changes from remote
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub sync: bool
}

impl Cli {
    pub fn parse() -> Self {
        Self::parse_from(std::env::args())
    }
}
