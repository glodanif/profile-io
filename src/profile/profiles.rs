use serde::{Deserialize, Serialize};
use crate::profile::Profile;

#[derive(Serialize, Deserialize)]
pub struct Profiles {
    pub current_profile_id: Option<String>,
    pub profiles: Vec<Profile>,
}
