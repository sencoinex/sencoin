use crate::Result;
use clap::Args;
use sencoin_config::{NodeConfig, NodeConfigArgs};
use std::path::PathBuf;

/// Run validator or full node
#[derive(Args, Debug)]
pub struct NodeRunCommandArgs {
    /// Path to node configuration file.
    #[arg(short = 'f', long)]
    config: Option<PathBuf>,

    /// Data directory to store chain data.
    #[clap(long)]
    data_dir: Option<PathBuf>,
}

impl Into<NodeConfigArgs> for &NodeRunCommandArgs {
    fn into(self) -> NodeConfigArgs {
        todo!()
    }
}

pub fn execute(args: NodeRunCommandArgs) -> Result<()> {
    // load config
    let config_file_path = if let Some(path) = args.config.as_deref() {
        path
    } else {
        // define default path
        todo!()
    };
    let config_args: NodeConfigArgs = (&args).into();
    let config = NodeConfig::load(config_file_path, config_args)?;
    start(config)
}

fn start(_config: NodeConfig) -> Result<()> {
    // TODO set up panic handler
    // TODO set up logger
    // TODO start inspection service
    // TODO start storage service
    // TODO start telemetry service
    // TODO start network service
    // TODO start peer monitoring service
    // TODO start state sync service
    // TODO start mempool service
    // TODO start consensus service
    todo!()
}
