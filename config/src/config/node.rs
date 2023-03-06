use crate::{NodeConfigArgs, NodeConfigToml, Result};
use std::path::Path;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NodeConfig {}

impl NodeConfig {
    pub fn load<P: AsRef<Path>, A: Into<NodeConfigArgs>>(
        config_file_path: P,
        args: A,
    ) -> Result<Self> {
        let toml = NodeConfigToml::load(config_file_path)?;
        let args: NodeConfigArgs = args.into();
        Ok((toml, args).into())
    }
}

impl From<(NodeConfigToml, NodeConfigArgs)> for NodeConfig {
    fn from((_toml, _args): (NodeConfigToml, NodeConfigArgs)) -> Self {
        // TODO copy from toml
        // TODO override values by args
        todo!()
    }
}
