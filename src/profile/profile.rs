use crate::audio::audio_config::AudioConfig;
use crate::profile::monitor_config::MonitorConfig;
use crate::profile::workspace_config::WorkspaceConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: Option<String>,
    pub name: String,
    pub monitors: Vec<MonitorConfig>,
    pub workspaces: Vec<WorkspaceConfig>,
    pub move_all_windows_to_monitor: Option<String>,
    pub focus_monitor_name: Option<String>,
    pub focus_workspace_id: Option<u8>,
    pub audio_sink: Option<AudioConfig>,
}
