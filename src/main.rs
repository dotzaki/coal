use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};

use serde::{Deserialize, Serialize};

use serde_yaml;

use path_absolutize::Absolutize;

const TEST_PATH: &str = "/tmp/coal/file";

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
    // Check if the state file exists if not create an empty file.

    if !Path::new(TEST_PATH).exists() {
        File::create(TEST_PATH).expect("Tried to create TEST_PATH");
    }
    
    match cli.command {
        Some(command) => {
            command_handler(&command);
        }
        None => {
            println!("Starting TUI");
        }
    }
}

/// This deserializes the `.yaml` file into the AppState object that you can then manipulate
/// NOTE: Make sure to write your changes using `write_state()` if changing the state file.
fn get_state() -> AppState {
    let data = fs::read_to_string(TEST_PATH).expect("Tried to read TEST_PATH into string");
    if data.is_empty() {
        AppState {
            tracking_repo: Vec::new(),
        }
    } else {
        serde_yaml::from_str(&data).expect("Tried to deserialize the TEST_PATH into AppState")
    }
}

/// Serialize the current AppState and write it into the TEST_PATH
fn write_state(state: AppState) {
    let se = serde_yaml::to_string(&state).expect("Tried to serialize AppState into string");
    fs::write(TEST_PATH, &se).expect("Tried to write serialized AppState into TEST_PATH");
}

/// When application is ran with commands, then it is handled here.
/// We might want to use some form of strategy pattern to handle the commands?
/// To be honest though, just get it done quick and dirty.
fn command_handler(command: &Commands) {
    let apath;
    match command {
        Commands::Add { path } => {
            apath = get_absolute_path(path);
            add_path(&apath);
        }
        Commands::Delete { path } => {
            apath = get_absolute_path(path);
            delete_path(&apath);
        }
        Commands::List => {
            list_tracking();
        }
    }
}

fn list_tracking() {
    todo!()
}

fn delete_path(apath: &PathBuf) {
    // if path in tracking
    // delete path from tracking
    // else path is not in tracking
}

/// Add path to tracking
/// TODO: Repositories should be unique, handle by checking the commit hash of the first commit
/// then error out if the commit hash is the same
fn add_path(apath: &PathBuf) {
    if has_repo(apath) {
        println!("Adding path: {:?}", apath);
        let name = apath.file_name();
        let repo = Repo {
            name: name.unwrap().to_str().unwrap().to_string(),
            path: apath.clone(),
        };
        let mut app_state = get_state();
        app_state.tracking_repo.push(repo);
        write_state(app_state);
    } else {
        println!("Path {:?} does not have a git repo", apath);
    }
}

/// Checks if a path has a git repo.
fn has_repo(apath: &PathBuf) -> bool {
    let mut path = apath.clone();
    path.push(".git");
    if path.exists() {
        return true;
    } else {
        return false;
    }
}

/// Given a path in any format (relative, absolute, etc..) give back the absolute path.
fn get_absolute_path(path: &PathBuf) -> PathBuf {
    let p = Path::new(&path);
    p.absolutize().unwrap().to_path_buf()
}

#[derive(Serialize, Deserialize, Debug)]
struct AppState {
    tracking_repo: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    name: String,
    path: PathBuf,
}
