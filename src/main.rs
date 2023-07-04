use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use path_absolutize::Absolutize;

mod app_state;

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

/// Check if the command is being ran by itself ... if so, run the tui.
/// If it is being ran with commands then handle.
fn main() {
    let cli = Cli::parse();
    app_state::init_state();

    match cli.command {
        Some(command) => {
            command_handler(&command);
        }
        None => {
            println!("Starting TUI");
        }
    }
}

/// When application is ran with commands, then it is handled here.
/// We might want to use some form of strategy pattern to handle the commands?
/// To be honest though, just get it done quick and dirty.
fn command_handler(command: &Commands) {
    let apath;
    match command {
        Commands::Add { path } => {
            apath = get_absolute_path(path);
            app_state::add_path(&apath);
        }
        Commands::Delete { path } => {
            apath = get_absolute_path(path);
            app_state::delete_path(apath.file_name().unwrap().to_str().unwrap().to_string());
        }
        Commands::List => {
            app_state::list_tracking();
        }
    }
}

/// Given a path in any format (relative, absolute, etc..) give back the absolute path.
fn get_absolute_path(path: &PathBuf) -> PathBuf {
    let p = Path::new(&path);
    p.absolutize().unwrap().to_path_buf()
}
