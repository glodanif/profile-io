use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
