#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AudioError {
    #[error("Failed to execute command: {0}")]
    CommandExecutionError(String),
    #[error("Failed to set audio sink: {0}")]
    FailedToSetAudioSink(String),
}
