use crate::profile::monitor_config::MonitorConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: Option<String>,
    pub name: String,
    pub monitors: Vec<MonitorConfig>,
}
