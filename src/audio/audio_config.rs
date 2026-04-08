use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AudioConfig {
    pub sink_name: String,
    pub volume: u8,
}
