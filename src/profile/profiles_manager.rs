use crate::manager::error::DataModuleError;
use crate::profile::profile::Profile;
use crate::profile::profiles::Profiles;

pub struct ProfilesManager {
    app_data_dir: &'static str,
}

impl ProfilesManager {
    pub fn new() -> Self {
        Self {
            app_data_dir: "profile-io",
        }
    }

    fn get_profiles(&self) -> Result<Profiles, DataModuleError> {
        unimplemented!()
    }

    pub fn get_profiles_json(&self) -> Result<String, DataModuleError> {
        let profiles = self.get_profiles()?;
        serde_json::to_string_pretty(&profiles).map_err(|_| DataModuleError::EncodingError)
    }

    pub fn get_current_profile(&self) -> Result<Profile, DataModuleError> {
        let profiles = self.get_profiles()?;
        let current_profile_id = profiles.current_profile_id;
        if let Some(id) = current_profile_id {
            return Ok(profiles.profiles.into_iter().find(|p| p.id == id).unwrap());
        }
        Err(DataModuleError::CurrentProfileNotSet)
    }

    pub fn get_current_profile_json(&self) -> Result<String, DataModuleError> {
        let profile = self.get_current_profile()?;
        serde_json::to_string_pretty(&profile).map_err(|_| DataModuleError::EncodingError)
    }

    pub fn get_profile_by_id(&self, id: String) -> Result<Profile, DataModuleError> {
        let profiles = self.get_profiles()?;
        profiles
            .profiles
            .into_iter()
            .find(|p| p.id == id)
            .ok_or(DataModuleError::ProfileNotFound)
    }
}
