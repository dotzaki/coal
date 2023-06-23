#![allow(dead_code, unused)]

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Stdout},
    path::{Path, PathBuf},
    time::Duration,
};

use clap::{Args, Parser, Subcommand};

const TEST_PATH: &str = "/tmp/coal/file";

/// State of the application, used to write with serde?
struct State {
    tracking_repo: Vec<PathBuf>,
}

/// Coalesce your repositories 🧐
#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add repo to tracking
    Add(RepoPath),
    /// Delete repo from tracking
    Delete(RepoPath),
    /// List tracking repos
    List,
}

#[derive(Args, Debug)]
struct RepoPath {
    /// Directory path
    path: PathBuf,
}

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

fn command_handler(command: &Commands) {
    match command {
        Commands::Add(path) => {
            // Write into TEST_PATH for now.
            //
        }
        Commands::Delete(path) => println!("Del {:#?}", path.path),
        Commands::List => println!("List"),
    }
}

fn get_current_dir() -> Result<PathBuf, std::io::Error> {
    std::env::current_dir()
}

/// This just writes some text into a file that we need to test that the list can show
fn setup_state() {
    let data = "some text\nwritten to file\nhere ";
    match fs::create_dir("/tmp/coal") {
        Ok(_) => println!("Directory created at /tmp/coal"),
        Err(error) => {
            if error.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Trying to create directory with error: {:?}", error)
            } else {
                println!("Directory already exists at /tmp/coal")
            }
        }
    };
    fs::write(TEST_PATH, data).expect("Should be able to write to `/tmp/coal/file.txt`");
}
