use crate::Result;
use clap::Args;

/// Run local testnet
///
/// This local testnet will run it's own Genesis and run as a single node
/// network locally.  Optionally, a faucet can be added for minting APT coins.
#[derive(Args, Debug)]
pub struct NodeRunLocalTestnetCommandArgs {
    /// Clean the state and start with a new chain at genesis
    #[arg(long)]
    force_restart: bool,
}

pub fn execute(args: NodeRunLocalTestnetCommandArgs) -> Result<()> {
    println!("{:?}", args);
    Ok(())
}
