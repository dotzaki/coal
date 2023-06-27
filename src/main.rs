#![allow(dead_code, unused)]

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Stdout},
    path::{Path, PathBuf},
    time::Duration,
};

use clap::{Args, Parser, Subcommand};

use path_absolutize::Absolutize;

const TEST_PATH: &str = "/tmp/coal/file";

/// State of the application, used to write with serde?
struct State {
    tracking_repo: Vec<PathBuf>,
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

// #[derive(Args, Debug)]
// struct RepoPath {
//     /// Directory path
//     path: PathBuf,
// }

/// Check if the command is being ran by itself ... if so, run the tui.
/// If it is being ran with commands then handle.
fn main() {
    let cli = Cli::parse();
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
    match command {
        Commands::Add { path } => {
            let apath = get_absolute_path(path);
            println!("{:?}", apath);
        }
        Commands::Delete { path } => println!("Del {:#?}", path),
        Commands::List => println!("List"),
    }
}

/// Just to get the current directory that the application is ran from.
/// I want this so when the user rungs `coal add` and some relative path, then we can get the
/// absolute path via. prepending this functions output
fn get_current_dir() -> Result<PathBuf, std::io::Error> {
    std::env::current_dir()
}

fn get_absolute_path(path: &PathBuf) -> PathBuf {
    let p = Path::new(&path);
    p.absolutize().unwrap().to_path_buf()
}
