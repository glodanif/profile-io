use crate::display::size::Size;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mode {
    pub resolution: Size,
    pub refresh_rate: Vec<f32>,
}
