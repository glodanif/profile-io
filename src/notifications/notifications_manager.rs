use crate::notifications::notification_error::NotificationError;
use std::process::Command;

const NOTIFY_CMD: &str = "notify-send";

pub struct NotificationsManager;

impl NotificationsManager {
    pub fn new() -> Self {
        NotificationsManager {}
    }

    pub fn notify(&self, title: &str, message: &str) -> Result<(), NotificationError> {
        let output = Command::new(NOTIFY_CMD)
            .args(&[title, message])
            .output()
            .map_err(|_| {
                NotificationError::CommandExecutionError(format!(
                    "Failed to execute command {} {} {}",
                    NOTIFY_CMD, title, message
                ))
            })?;
        if output.status.success() {
            Ok(())
        } else {
            Err(NotificationError::CommandExecutionError(format!(
                "Failed to execute command {} {} {}",
                NOTIFY_CMD, title, message
            )))
        }
    }
}
