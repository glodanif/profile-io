use crate::profile::Profile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub current_profile_id: Option<String>,
    pub profiles: Vec<Profile>,
}
