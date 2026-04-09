use crate::audio::audio_manager::AudioManager;
use crate::cli::Command;
use crate::display::display_manager::DisplayManager;
use crate::notifications::notifications_manager::NotificationsManager;
use crate::profile::{Profile, ProfilesManager};

pub struct Dispatcher<'a> {
    display_manager: &'a Box<dyn DisplayManager>,
    audio_manager: &'a Box<dyn AudioManager>,
    profiles_manager: &'a ProfilesManager<'a>,
    notifications_manager: &'a NotificationsManager,
}

impl<'a> Dispatcher<'a> {
    pub fn new(
        display_manager: &'a Box<dyn DisplayManager>,
        audio_manager: &'a Box<dyn AudioManager>,
        profiles_manager: &'a ProfilesManager<'a>,
        notifications_manager: &'a NotificationsManager,
    ) -> Self {
        Self {
            display_manager,
            audio_manager,
            profiles_manager,
            notifications_manager,
        }
    }

    pub fn handle_command(&self, command: Option<Command>) {
        match command {
            Some(Command::Profiles) => {
                let profiles = &self.profiles_manager.get_profiles_json();
                match profiles {
                    Ok(profiles) => println!("{}", profiles),
                    Err(err) => eprintln!("Failed to get profiles: {}", err),
                }
            }
            Some(Command::AddProfile { profile_json }) => {
                let id = &self.profiles_manager.add_profile(profile_json);
                match id {
                    Ok(id) => println!("Profile added successfully: {}", id),
                    Err(err) => eprintln!("Failed to add profile: {}", err),
                }
            }
            Some(Command::RemoveProfile { profile_id }) => {
                eprintln!("Remove profile not implemented yet: {}", profile_id);
            }
            Some(Command::Current) => {
                let current_profile = &self.profiles_manager.get_current_profile_json();
                match current_profile {
                    Ok(profile) => println!("{}", profile),
                    Err(err) => eprintln!("Failed to get current profile: {}", err),
                }
            }
            Some(Command::Restore) => match &self.profiles_manager.get_current_profile() {
                Ok(profile) => self.apply_profile(profile),
                Err(err) => eprintln!("Failed to get current profile: {}", err),
            },
            Some(Command::Apply { profile_id }) => {
                match &self.profiles_manager.get_profile_by_id(profile_id) {
                    Ok(profile) => self.apply_profile(profile),
                    Err(err) => eprintln!("Failed to get profile by id: {}", err),
                }
            }
            Some(Command::ApplyNext) => match &self.profiles_manager.get_next_profile() {
                Ok(profile) => self.apply_profile(profile),
                Err(err) => eprintln!("Failed to get next profile: {}", err),
            },
            Some(Command::Monitors) => {
                let monitors = &self.display_manager.get_monitors_json();
                match monitors {
                    Ok(monitors) => println!("{}", monitors),
                    Err(err) => eprintln!("Failed to get monitors: {}", err),
                }
            }
            None => {
                eprintln!("No command provided, use --help for usage");
                std::process::exit(1);
            }
        }
    }

    fn apply_profile(&self, profile: &Profile) {
        let result = self.display_manager.set_monitors_profile(&profile);
        match result {
            Ok(_) => {
                println!("Monitor config applied successfully: {}", profile.name);
                let _ = self
                    .notifications_manager
                    .notify("Profile applied", &profile.name);
                if let Some(profile_id) = &profile.id {
                    if let Err(err) = self
                        .profiles_manager
                        .set_current_profile_id(profile_id.clone())
                    {
                        eprintln!("Warning: Failed to update current profile ID: {}", err);
                    }
                }
            }
            Err(err) => eprintln!("Failed to apply profile: {}", err),
        }

        if let Some(audio_sink) = profile.audio_sink.as_ref() {
            let result = self.audio_manager.set_audio_sink(audio_sink);
            match result {
                Ok(_) => {
                    println!("Audio config applied successfully: {}", profile.name);
                }
                Err(err) => {
                    eprintln!("Warning: Failed to update set audio sink {}", err); 
                }
            }
        }
    }
}
