#![allow(unused, dead_code)]
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use path_absolutize::Absolutize;
use repo::Tracking;

mod app;
mod cli;
mod repo;
mod tui;

/// Check if the command is being ran by itself ... if so, run the tui.
/// If it is being ran with commands then handle.
fn main() {
    let cli = Cli::parse();

    repo::setup_tracking_file();
    let mut tracking = Tracking::new();

    match cli.command {
        Some(command) => {
            let _ = cli::run(command, tracking);
        }
        None => {
            // Handle error here from tui initialize
            let _ = tui::run();
        }
    }
}

/// Command line input, used to parse whether or not the application is ran with or without
/// arguments
#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

/// Wanted enums, so we have a "definitive" set of commands that exist?
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add repo to tracking
    Add { path: PathBuf },
    /// Delete repo from tracking
    Delete { path: PathBuf },
    /// List tracking repos
    List,
}
