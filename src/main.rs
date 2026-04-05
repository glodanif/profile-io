mod cli;
mod manager;
mod profile;

use crate::cli::{Cli, Command};
use clap::Parser;

fn main() {
    let display_manager = manager::get_display_manager();

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Profiles) => {
            println!("Profiles command");
        }
        Some(Command::Current) => {}
        Some(Command::Restore) => {}
        Some(Command::Apply { profile_id }) => {}
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
