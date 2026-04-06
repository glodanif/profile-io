use crate::display::display_error::DisplayError;
use std::process::Command;

const NOTIFY_CMD: &str = "notify-send";

pub struct NotificationsManager;

impl NotificationsManager {
    pub fn new() -> Self {
        NotificationsManager {}
    }

    pub fn notify(&self, title: &str, message: &str) -> Result<(), DisplayError> {
        let output = Command::new(NOTIFY_CMD)
            .args(&[title, message])
            .output()
            .map_err(|_| DisplayError::CommandExecutionError)?;
        if output.status.success() {
            Ok(())
        } else {
            Err(DisplayError::CommandExecutionError)
        }
    }
}
