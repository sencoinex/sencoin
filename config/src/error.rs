use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("The {target} could not be found. ensure the config file path: {path}")]
    ConfigFileNotFound { target: String, path: String },
    #[error("Error failed to open config file({path}): {reason}")]
    FailedToOpenConfigFile { path: String, reason: String },
    #[error("Error failed to read config file content({path}): {reason}")]
    FailedToReadConfigFileContent { path: String, reason: String },
    #[error("TomlDeserializationError: {cause:?}")]
    TomlDeserializationError {
        #[source]
        cause: toml::de::Error,
    },
}
