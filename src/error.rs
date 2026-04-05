#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DataModuleError {
    #[error("Failed to get monitors")]
    FailedToGetMonitors,
}
