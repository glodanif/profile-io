use crate::profile::validation_error::ValidationError;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DisplayError {
    #[error("Failed to execute command {0}")]
    CommandExecutionError(String),
    #[error("Failed to parse command output")]
    CommandOutputParseError,
    #[error("Failed to encode/decode data {0}")]
    EncodingError(&'static str),
    #[error("Current profile not set")]
    CurrentProfileNotSet,
    #[error("Profile not found")]
    ProfileNotFound,
    #[error("Not enough profiles to switch (need at least 2)")]
    NotEnoughProfiles,
    #[error("Failed to get config")]
    FailedToGetConfig,
    #[error("Failed to create config")]
    FailedToCreateConfig,
    #[error("Failed to set config")]
    FailedToSetConfig,
    #[error("Config is not supported {0}")]
    ConfigIsNotSupported(ValidationError),
}
