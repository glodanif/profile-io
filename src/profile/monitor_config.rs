use serde::{Deserialize, Serialize};
use crate::manager::size::Size;

#[derive(Serialize, Deserialize)]
pub struct MonitorConfig {
    pub name: String,
    pub scale: f64,
    pub transformation: u8,
    pub resolution: Size,
    pub refresh_rate: f64,
    pub is_enabled: bool,
    pub mirror_of_name: Option<String>,
    pub current_position: Size,
}
