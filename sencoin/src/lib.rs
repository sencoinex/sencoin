pub mod node;

use clap::Parser;

pub type Result<T> = anyhow::Result<T>;

/// Command Line Interface (CLI) for developing and interacting with the SEN blockchain
#[derive(Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
pub enum Cli {
    #[command(subcommand)]
    Node(node::NodeSubCommand),
}

impl Cli {
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Node(subcommand) => subcommand.execute(),
        }
    }
}
