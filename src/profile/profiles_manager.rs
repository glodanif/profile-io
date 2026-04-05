use crate::profile::profile::Profile;

pub struct ProfilesManager {}

impl ProfilesManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_profiles(&self) -> Vec<Profile> {
        vec![]
    }

    pub fn get_current_profile(&self) -> Option<Profile> {
        None
    }

    pub fn get_profile_by_id(&self, id: &str) -> Option<Profile> {
        None
    }
}
