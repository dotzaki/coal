use std::{path::{PathBuf, Path}, fs::{self, File}};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    tracking_repo: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Repo {
    name: String,
    path: PathBuf,
}

const TEST_PATH: &str = "/tmp/coal/file";

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

/// List all of the Repo objects in AppState::tracking_repo
pub fn list_tracking() {
    if get_state().tracking_repo.is_empty() {
        println!("No paths are being tracked");
    } else {
        for repo in get_state().tracking_repo.iter() {
            println!("{:#?}", repo);
        }
    }
}

/// Delete a path from tracking via. name as the names should be unique
/// TODO: Make this less shit
pub fn delete_path(path_name: String) {
    let mut app_state = get_state();
    let mut index = 0;
    for repo in app_state.tracking_repo.iter() {
        if repo.name == path_name {
            app_state.tracking_repo.remove(index);
            write_state(app_state);
            println!("Path {:?} has been removed", path_name);
            return;
        }
        index += 1;
    }
    println!("Path is not being tracked.");
}

/// Add path to tracking
/// Assume the names of the directory are unique.
/// TODO: Have some unique identifier that isn't the name of the directory.
pub fn add_path(apath: &PathBuf) {

    for repo in get_state().tracking_repo.iter() {
        if repo.path == *apath {
            println!("Path {:?} is already being tracked", apath);
            return;
        }
    }
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

/// Check if the state file exists if not create an empty file.
pub fn init_state() {
    if !Path::new(TEST_PATH).exists() {
        File::create(TEST_PATH).expect("Tried to create TEST_PATH");
    }
}
