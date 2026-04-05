#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DataModuleError {
    #[error("Failed to get monitors")]
    CommandExecutionError,
    #[error("Failed to parse command output")]
    CommandOutputParseError,
    #[error("Failed to encode/decode data")]
    EncodingError,
    #[error("Current profile not set")]
    CurrentProfileNotSet,
    #[error("Profile not found")]
    ProfileNotFound,
}
