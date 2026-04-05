mod cli;
mod manager;
mod profile;

use crate::cli::{Cli, Command};
use crate::manager::get_display_manager;
use crate::profile::ProfilesManager;
use clap::Parser;

fn main() {
    let display_manager = get_display_manager();
    let profiles_manager = ProfilesManager::new();

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Profiles) => {
            let profiles = profiles_manager.get_profiles_json();
            match profiles {
                Ok(profiles) => println!("{}", profiles),
                Err(err) => eprintln!("Failed to get profiles: {}", err),
            }
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
                        Ok(_) => println!("Profile restored successfully: {}", profile.name),
                        Err(err) => eprintln!("Failed to restore profile: {}", err),
                    }
                }
                Err(err) => eprintln!("Failed to get current profile: {}", err),
            }
        }
        Some(Command::Apply { profile_id }) => {
            let profile = profiles_manager.get_profile_by_id(profile_id);
            match profile {
                Ok(profile) => {
                    let result = display_manager.set_monitors_profile(&profile);
                    match result {
                        Ok(_) => println!("Profile applies successfully: {}", profile.name),
                        Err(err) => eprintln!("Failed to apply profile: {}", err),
                    }
                }
                Err(err) => eprintln!("Failed to get profile by id: {}", err),
            }
        }
        Some(Command::Monitors) => {
            let monitors = display_manager.get_monitors();
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
