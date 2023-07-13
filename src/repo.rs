use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

const TEST_PATH: &str = "/tmp/coal/file";

/// Object to serialize/deserialize into the filesystem
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Tracking {
    pub active: Vec<Repo>,
}

impl Tracking {
    pub fn new() -> Tracking {
        let data = fs::read_to_string(TEST_PATH).expect("Tried to read TEST_PATH into string");
        if data.is_empty() {
            Tracking { active: Vec::new() }
        } else {
            // HACK: This might not return `Tracking` object
            serde_yaml::from_str(&data).expect("Tried to deserialize the TEST_PATH into AppState")
        }
    }

    /// This serializes the current state into TEST_PATH
    pub fn write_changes(self) {
        let se = serde_yaml::to_string(&self).expect("Tried to serialize AppState into string");
        fs::write(TEST_PATH, &se).expect("Tried to write serialized AppState into TEST_PATH");
    }

    pub fn list(self) -> Vec<Repo> {
        if self.active.is_empty() {
            Vec::new()
        } else {
            self.active
        }
    }

    /// Loop over the actively tracked repo and use `write_changes` then send whether or not
    /// success
    /// Returns whether or not the path was successfully deleted
    pub fn delete(self, path_name: String) -> bool {
        todo!()
    }

    /// Returns whether or not the path was successfully added
    pub fn add(self) -> bool {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Repo {
    name: String,
    path: PathBuf,
}

pub fn delete_repo(path_name: String) {
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

pub fn add_repo(apath: &PathBuf) {
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
pub fn setup_tracking_file() {
    if !Path::new(TEST_PATH).exists() {
        File::create(TEST_PATH).expect("Tried to create TEST_PATH");
    }
}
