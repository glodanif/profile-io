mod cli;
pub mod dispatcher;
mod display;
mod notifications;
mod profile;
pub mod audio;

use crate::cli::Cli;
use crate::dispatcher::Dispatcher;
use crate::display::get_display_manager;
use crate::notifications::notifications_manager::NotificationsManager;
use crate::profile::ProfilesManager;
use clap::Parser;

fn main() {
    let notifications_manager = NotificationsManager::new();
    let display_manager = get_display_manager();
    let profiles_manager = ProfilesManager::new(&display_manager, &notifications_manager);
    let dispatcher = Dispatcher::new(&display_manager, &profiles_manager, &notifications_manager);
    let cli = Cli::parse();
    dispatcher.handle_command(cli.command);
}
