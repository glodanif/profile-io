mod cli;
mod manager;
mod profile;

use crate::cli::{Cli, Command};
use crate::manager::get_display_manager;
use crate::profile::ProfilesManager;
use clap::Parser;

fn main() {
    let display_manager = get_display_manager();
    let profiles_manager = ProfilesManager::new(&display_manager);

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Profiles) => {
            let profiles = profiles_manager.get_profiles_json();
            match profiles {
                Ok(profiles) => println!("{}", profiles),
                Err(err) => eprintln!("Failed to get profiles: {}", err),
            }
        }
        Some(Command::AddProfile { profile_json }) => {
            let id = profiles_manager.add_profile(profile_json);
            match id {
                Ok(id) => println!("Profile added successfully: {}", id),
                Err(err) => eprintln!("Failed to add profile: {}", err),
            }
        }
        Some(Command::RemoveProfile { profile_id }) => {
            eprintln!("Remove profile not implemented yet: {}", profile_id);
        }
        Some(Command::Current) => {
            let current_profile = profiles_manager.get_current_profile_json();
            match current_profile {
                Ok(profile) => println!("{}", profile),
                Err(err) => eprintln!("Failed to get current profile: {}", err),
            }
        }
        Some(Command::Restore) => {
            let current_profile = profiles_manager.get_current_profile();
            match current_profile {
                Ok(profile) => {
                    let result = display_manager.set_monitors_profile(&profile);
                    match result {
                        Ok(_) => {
                            println!("Profile restored successfully: {}", profile.name);
                            if let Some(profile_id) = &profile.id {
                                if let Err(err) =
                                    profiles_manager.set_current_profile_id(profile_id.clone())
                                {
                                    eprintln!(
                                        "Warning: Failed to update current profile ID: {}",
                                        err
                                    );
                                }
                            }
                        }
                        Err(err) => eprintln!("Failed to restore profile: {}", err),
                    }
                }
                Err(err) => eprintln!("Failed to get current profile: {}", err),
            }
        }
        Some(Command::Apply { profile_id }) => {
            let profile = profiles_manager.get_profile_by_id(profile_id.clone());
            match profile {
                Ok(profile) => {
                    let result = display_manager.set_monitors_profile(&profile);
                    match result {
                        Ok(_) => {
                            println!("Profile applied successfully: {}", profile.name);
                            if let Err(err) = profiles_manager.set_current_profile_id(profile_id) {
                                eprintln!("Warning: Failed to update current profile ID: {}", err);
                            }
                        }
                        Err(err) => eprintln!("Failed to apply profile: {}", err),
                    }
                }
                Err(err) => eprintln!("Failed to get profile by id: {}", err),
            }
        }
        Some(Command::ApplyNext) => {
            let profile = profiles_manager.get_next_profile();
            match profile {
                Ok(profile) => {
                    let result = display_manager.set_monitors_profile(&profile);
                    match result {
                        Ok(_) => {
                            println!("Profile applied successfully: {}", profile.name);
                            if let Some(profile_id) = &profile.id {
                                if let Err(err) =
                                    profiles_manager.set_current_profile_id(profile_id.clone())
                                {
                                    eprintln!(
                                        "Warning: Failed to update current profile ID: {}",
                                        err
                                    );
                                }
                            }
                        }
                        Err(err) => eprintln!("Failed to apply profile: {}", err),
                    }
                }
                Err(err) => eprintln!("Failed to get profile by id: {}", err),
            }
        }
        Some(Command::Monitors) => {
            let monitors = display_manager.get_monitors_json();
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
