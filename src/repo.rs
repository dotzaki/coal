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

/// Need to enforce there only being one "instance" at a time to avoid race conditions? possibly.
/// Problem two separate pieces of code see the thing, both want to write assuming the original
/// state one writes before the other, so the second has old information.
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
    pub fn write(&self) {
        let se = serde_yaml::to_string(self).expect("Tried to serialize AppState into string");
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
    /// TODO: Might want to use anyhow errors instead of bool to updat caller on what happened.
    pub fn delete(&mut self, path_name: String) -> bool {
        let mut index = 0;
        for repo in self.active.iter() {
            if repo.name == path_name {
                self.active.remove(index);
                self.write();
                return true;
            }
        }
        false
    }

    /// Returns whether or not the path was successfully added
    pub fn add(&mut self, path: &PathBuf) -> bool {
        for repo in self.active.iter() {
            // If the repo already exists
            if repo.path == *path {
                return false;
            }
        }

        if has_repo(path) {
            let name = path.file_name();
            let repo = Repo {
                name: name.unwrap().to_str().unwrap().to_string(),
                path: path.clone(),
            };
            self.active.push(repo);
            self.write();
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Repo {
    pub name: String,
    pub path: PathBuf,
}

/// Checks if a path has a git repo.
fn has_repo(apath: &PathBuf) -> bool {
    let mut path = apath.clone();
    path.push(".git");
    if path.exists() {
        true
    } else {
        false
    }
}

/// Check if the state file exists if not create an empty file.
pub fn setup_tracking_file() {
    if !Path::new(TEST_PATH).exists() {
        File::create(TEST_PATH).expect("Tried to create TEST_PATH");
    }
}
