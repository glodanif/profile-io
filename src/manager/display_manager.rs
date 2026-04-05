use crate::error::DataModuleError;

pub trait DisplayManager {
    fn get_monitors(&self) -> Result<String, DataModuleError>;
}
