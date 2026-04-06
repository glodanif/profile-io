#[derive(Debug, thiserror::Error, PartialEq)]
pub enum NotificationError {
    #[error("Failed to execute command: {0}")]
    CommandExecutionError(String),
}
