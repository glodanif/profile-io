#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ValidationError {
    #[error("Monitor not found")]
    MonitorNotFound(String),
    #[error("Resolution not supported")]
    ResolutionNotSupported(String, String, String),
    #[error("Refresh rate not supported")]
    RefreshRateNotSupported(String, String, String),
    #[error("Invalid transformation value")]
    InvalidTransformationValue(String, String, String),
    #[error("Invalid scale value")]
    InvalidScaleValue(String, String, String),
    #[error("Invalid mirror source name")]
    InvalidMirrorSourceName(String, String, String),
    #[error("Duplicate profile ID")]
    DuplicateProfileId(String),
}
