use crate::manager::error::DataModuleError;
use crate::manager::monitor::Monitor;
use crate::profile::Profile;

pub trait DisplayManager {
    fn get_monitors(&self) -> Result<Vec<Monitor>, DataModuleError>;
    fn get_monitors_json(&self) -> Result<String, DataModuleError>;
    fn set_monitors_profile(&self, profile: &Profile) -> Result<(), DataModuleError>;
}
