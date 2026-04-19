use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, help = "Print commands without executing them")]
    pub dry_run: bool,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Monitors,
    AudioSinks,
    Profiles,
    AddProfile { profile_json: String },
    RemoveProfile { profile_id: String },
    Current,
    Restore { delay_ms: Option<u64> },
    Apply { profile_id: String },
    ApplyNext,
}
