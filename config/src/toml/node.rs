use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

/// Node configurations defined in `node.toml`
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NodeConfigToml {}

impl NodeConfigToml {
    pub fn load<P: AsRef<Path>>(config_file_path: P) -> Result<Self> {
        let file_path_str = config_file_path.as_ref().to_str().unwrap();
        if !config_file_path.as_ref().exists() {
            return Err(Error::ConfigFileNotFound {
                target: "node config file".to_owned(),
                path: file_path_str.to_owned(),
            });
        }
        let mut file =
            File::open(&config_file_path).map_err(|err| Error::FailedToOpenConfigFile {
                path: file_path_str.to_owned(),
                reason: format!("{:?}", err),
            })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| Error::FailedToReadConfigFileContent {
                path: file_path_str.to_owned(),
                reason: format!("{:?}", err),
            })?;
        toml::from_str(&contents).map_err(|cause| Error::TomlDeserializationError { cause })
    }
}
