use crate::{manager::error::DataModuleError};
use crate::profile::Profile;

pub trait DisplayManager {
    fn get_monitors(&self) -> Result<String, DataModuleError>;
    fn set_monitors_profile(&self, profile: &Profile) -> Result<(), DataModuleError>;
}
