use crate::audio::audio_manager::AudioManager;
use crate::display::display_error::DisplayError;
use crate::display::display_manager::DisplayManager;
use crate::display::monitor::Monitor;
use crate::display::transformation::Transformation;
use crate::profile::config::Config;
use crate::profile::profile::Profile;
use crate::profile::validation_error::ValidationError;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const CONFIG_FILE_NAME: &str = "config.toml";

pub struct ProfilesManager<'a> {
    display_manager: &'a Box<dyn DisplayManager>,
    config_dir: PathBuf,
    config_file: PathBuf,
}

impl<'a> ProfilesManager<'a> {
    pub fn new(
        display_manager: &'a Box<dyn DisplayManager>,
        audio_manager: &'a Box<dyn AudioManager>,
    ) -> Self {
        let config_dir = dirs::config_dir()
            .expect("Could not find config directory")
            .join(env!("CARGO_PKG_NAME"));
        let config_file = config_dir.join(CONFIG_FILE_NAME);
        ProfilesManager {
            display_manager,
            config_dir,
            config_file,
        }
    }

    fn get_profiles(&self) -> Result<Config, DisplayError> {
        if self.config_file.exists() {
            let config_file_content = fs::read_to_string(&self.config_file)
                .map_err(|_| DisplayError::FailedToGetConfig)?;
            let config: Config = toml::from_str(&config_file_content)
                .map_err(|_| DisplayError::FailedToGetConfig)?;
            Ok(config)
        } else {
            fs::create_dir_all(&self.config_dir).map_err(|_| DisplayError::FailedToCreateConfig)?;
            Ok(Config {
                profiles: Vec::new(),
                current_profile_id: None,
            })
        }
    }

    pub fn get_profiles_json(&self) -> Result<String, DisplayError> {
        let profiles = self.get_profiles()?;
        serde_json::to_string_pretty(&profiles)
            .map_err(|_| DisplayError::EncodingError("get_profiles_json"))
    }

    pub fn add_profile(&self, profile_json: String) -> Result<String, DisplayError> {
        let mut profile: Profile = serde_json::from_str(&profile_json).map_err(|err| {
            println!("Error: {}", err);
            DisplayError::EncodingError("add_profile")
        })?;

        let mut profiles = self.get_profiles()?;

        let id = if let Some(user_id) = &profile.id {
            if profiles
                .profiles
                .iter()
                .any(|p| p.id.as_ref() == Some(user_id))
            {
                return Err(DisplayError::ConfigIsNotSupported(
                    ValidationError::DuplicateProfileId(user_id.clone()),
                ));
            }
            user_id.clone()
        } else {
            Uuid::new_v4().to_string()
        };

        let available_displays = self.display_manager.get_monitors()?;
        let validation_error = self.validate_profile(&profile, available_displays);
        if let Some(validation_error) = validation_error {
            return Err(DisplayError::ConfigIsNotSupported(validation_error));
        }

        profile.id = Some(id.clone());
        profiles.profiles.push(profile);
        fs::write(&self.config_file, toml::to_string(&profiles).unwrap())
            .map_err(|_| DisplayError::FailedToSetConfig)?;
        Ok(id)
    }

    pub fn get_current_profile(&self) -> Result<Profile, DisplayError> {
        let profiles = self.get_profiles()?;
        let current_profile_id = profiles.current_profile_id;
        if let Some(id) = current_profile_id {
            return Ok(profiles
                .profiles
                .into_iter()
                .find(|p| p.id.as_ref() == Some(&id))
                .unwrap());
        }
        Err(DisplayError::CurrentProfileNotSet)
    }

    pub fn get_current_profile_json(&self) -> Result<String, DisplayError> {
        let profile = self.get_current_profile()?;
        serde_json::to_string_pretty(&profile)
            .map_err(|_| DisplayError::EncodingError("get_current_profile_json"))
    }

    pub fn get_profile_by_id(&self, id: String) -> Result<Profile, DisplayError> {
        let profiles = self.get_profiles()?;
        profiles
            .profiles
            .into_iter()
            .find(|p| p.id.as_ref() == Some(&id))
            .ok_or(DisplayError::ProfileNotFound)
    }

    pub fn get_next_profile(&self) -> Result<Profile, DisplayError> {
        let mut profiles = self.get_profiles()?;
        if profiles.profiles.len() < 2 {
            return Err(DisplayError::NotEnoughProfiles);
        }

        let current_profile_id = profiles
            .current_profile_id
            .ok_or(DisplayError::CurrentProfileNotSet)?;

        let current_index = profiles
            .profiles
            .iter()
            .position(|p| p.id.as_ref() == Some(&current_profile_id))
            .ok_or(DisplayError::ProfileNotFound)?;
        let next_index = (current_index + 1) % profiles.profiles.len();

        Ok(profiles.profiles.remove(next_index))
    }

    pub fn set_current_profile_id(&self, profile_id: String) -> Result<(), DisplayError> {
        let mut config = self.get_profiles()?;
        if !config
            .profiles
            .iter()
            .any(|p| p.id.as_ref() == Some(&profile_id))
        {
            return Err(DisplayError::ProfileNotFound);
        }
        config.current_profile_id = Some(profile_id);

        fs::write(&self.config_file, toml::to_string(&config).unwrap())
            .map_err(|_| DisplayError::FailedToSetConfig)?;

        Ok(())
    }

    fn validate_profile(
        &self,
        profile: &Profile,
        available_displays: Vec<Monitor>,
    ) -> Option<ValidationError> {
        for monitor_config in &profile.monitors {
            let matching_display = available_displays
                .iter()
                .find(|d| d.name == monitor_config.name);

            if matching_display.is_none() {
                return Some(ValidationError::MonitorNotFound(
                    monitor_config.name.clone(),
                ));
            }

            let display = matching_display.unwrap();

            if monitor_config.scale < 0.1 || monitor_config.scale > 20.0 {
                return Some(ValidationError::InvalidScaleValue(
                    monitor_config.name.clone(),
                    monitor_config.scale.to_string(),
                    "Scale must be between 0.1 and 20.0".to_string(),
                ));
            }
            if Transformation::from_code(monitor_config.transformation).is_none() {
                return Some(ValidationError::InvalidTransformationValue(
                    monitor_config.name.clone(),
                    monitor_config.transformation.to_string(),
                    "Transformation must be between 0 and 7".to_string(),
                ));
            }

            let resolution_supported = display.modes.iter().any(|mode| {
                mode.resolution.width == monitor_config.resolution.width
                    && mode.resolution.height == monitor_config.resolution.height
            });

            if !resolution_supported {
                return Some(ValidationError::ResolutionNotSupported(
                    monitor_config.name.clone(),
                    format!(
                        "{}x{}",
                        monitor_config.resolution.width, monitor_config.resolution.height
                    ),
                    "Resolution not available for this monitor".to_string(),
                ));
            }

            let refresh_rate_supported = display.modes.iter().any(|mode| {
                mode.resolution.width == monitor_config.resolution.width
                    && mode.resolution.height == monitor_config.resolution.height
                    && mode
                        .refresh_rate
                        .iter()
                        .any(|&rate| (rate as f64 - monitor_config.refresh_rate).abs() < 0.01)
            });

            if !refresh_rate_supported {
                return Some(ValidationError::RefreshRateNotSupported(
                    monitor_config.name.clone(),
                    monitor_config.refresh_rate.to_string(),
                    "Refresh rate not available for this resolution".to_string(),
                ));
            }

            if let Some(mirror_source) = &monitor_config.mirror_of_name {
                if mirror_source == &monitor_config.name {
                    return Some(ValidationError::InvalidMirrorSourceName(
                        monitor_config.name.clone(),
                        mirror_source.clone(),
                        "Monitor cannot mirror itself".to_string(),
                    ));
                }

                if !profile.monitors.iter().any(|m| &m.name == mirror_source) {
                    return Some(ValidationError::InvalidMirrorSourceName(
                        monitor_config.name.clone(),
                        mirror_source.clone(),
                        "Mirror source monitor not found in profile".to_string(),
                    ));
                }
            }
        }

        None
    }
}
