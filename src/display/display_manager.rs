use crate::display::display_error::DisplayError;
use crate::display::monitor::Monitor;
use crate::profile::Profile;

pub trait DisplayManager {
    fn get_monitors(&self) -> Result<Vec<Monitor>, DisplayError>;
    fn get_monitors_json(&self) -> Result<String, DisplayError>;
    fn set_monitors_profile(&self, profile: &Profile) -> Result<(), DisplayError>;
}
