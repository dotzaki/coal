use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use path_absolutize::Absolutize;

mod app;
mod cli;
mod repo;
mod tui;

/// Check if the command is being ran by itself ... if so, run the tui.
/// If it is being ran with commands then handle.
fn main() {
    let cli = Cli::parse();

    repo::setup_tracking_file();

    match cli.command {
        Some(command) => {
            let _ = cli::run();
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
    command: Option<Commands>,
}

/// Wanted enums, so we have a "definitive" set of commands that exist?
#[derive(Subcommand, Debug)]
enum Commands {
    /// Add repo to tracking
    Add { path: PathBuf },
    /// Delete repo from tracking
    Delete { path: PathBuf },
    /// List tracking repos
    List,
}

/// Given a path in any format (relative, absolute, etc..) give back the absolute path.
fn get_absolute_path(path: PathBuf) -> PathBuf {
    let p = Path::new(&path);
    p.absolutize().unwrap().to_path_buf()
}
