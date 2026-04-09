mod audio;
mod cli;
mod dispatcher;
mod display;
mod notifications;
mod profile;

use crate::audio::get_audio_manager;
use crate::cli::Cli;
use crate::dispatcher::Dispatcher;
use crate::display::get_display_manager;
use crate::notifications::notifications_manager::NotificationsManager;
use crate::profile::ProfilesManager;
use clap::Parser;

fn main() {
    let display_manager = get_display_manager();
    let audio_manager = get_audio_manager();
    let notifications_manager = NotificationsManager::new();
    let profiles_manager = ProfilesManager::new(&display_manager);
    let dispatcher = Dispatcher::new(
        &display_manager,
        &audio_manager,
        &profiles_manager,
        &notifications_manager,
    );
    let cli = Cli::parse();
    dispatcher.handle_command(cli.command);
}
