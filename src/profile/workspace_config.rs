use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub id: u32,
    pub monitor_name: String,
}
