mod run;
pub use run::NodeRunCommandArgs;

#[cfg(any(feature = "dev"))]
mod run_local_test_net;
#[cfg(any(feature = "dev"))]
pub use run_local_test_net::NodeRunLocalTestnetCommandArgs;

use crate::Result;
use clap::Subcommand;

/// Tool for operations related to nodes
///
/// This tool allows you to run a local test node for testing,
/// identify issues with nodes, and show related information.
#[derive(Subcommand)]
pub enum NodeSubCommand {
    Run(NodeRunCommandArgs),
    #[cfg(any(feature = "dev"))]
    RunLocalTestnet(NodeRunLocalTestnetCommandArgs),
}

impl NodeSubCommand {
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Run(args) => run::execute(args),
            #[cfg(any(feature = "dev"))]
            Self::RunLocalTestnet(args) => run_local_test_net::execute(args),
        }
    }
}
