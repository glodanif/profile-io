use crate::manager::{mode::Mode, size::Size, transformation::Transformation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub model: String,
    pub description: String,
    pub scale: f64,
    pub transformation: Transformation,
    pub resolution: Size,
    pub refresh_rate: f64,
    pub is_enabled: bool,
    pub mirror_of_name: Option<String>,
    pub current_position: Size,
    pub modes: Vec<Mode>,
}
