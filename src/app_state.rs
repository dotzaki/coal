use std::{path::{PathBuf, Path}, fs::{self, File}};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppState {
    tracking_repo: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub fn list_tracking() {
    todo!()
}

pub fn delete_path(apath: &PathBuf) {
    // if path in tracking
    // delete path from tracking
    // else path is not in tracking
}

/// Add path to tracking
/// TODO: Repositories should be unique, handle by checking the commit hash of the first commit
/// then error out if the commit hash is the same
pub fn add_path(apath: &PathBuf) {
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
