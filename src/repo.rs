use std::{
    fs::{self},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{cli, config};

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
        let config_path = config::get_path();

        let data = match fs::read_to_string(&config_path) {
            Ok(t) => t,
            Err(_) => String::new(),
        };

        match serde_yaml::from_str(&data) {
            Ok(t) => t,
            // Make backup of current config.yaml to config.yaml.bak then restart with a fresh
            // config.yaml
            // TODO: Display warning! to user that the current file is erronous and it was backed
            // and replaced with a fresh one.
            Err(_) => {
                let backup = config_path.with_extension("bak");
                fs::copy(&config_path, &backup).expect("Failed to create backup of config.yaml");
                Tracking { active: Vec::new() }
            }
        }
    }

    /// This serializes the current state into the `config.yaml`path.
    fn write(&self) {
        let se = serde_yaml::to_string(self).expect("Tried to read tracking object into string");

        fs::write(config::get_path(), se)
            .expect("Tried to write serialized tracking object into config path");
    }

    pub fn list(&self) -> Result<Vec<Repo>, &str> {
        if self.active.is_empty() {
            Err("Currently not tracking any repositories.")
        } else {
            Ok(self.active.clone())
        }
    }

    /// Returns the name of the repo that was deleted
    pub fn delete(&mut self, dir_name: String) -> Result<String, &str> {
        let mut index = 0;
        for i in 0..self.active.len() {
            if self.active[i].name == dir_name {
                self.active.remove(i);
                self.write();
                return Ok(dir_name);
            }
        }
        Err("Repository passed in is not currently tracked.")
    }

    /// Returns the path if successful
    pub fn add(&mut self, dir_path: PathBuf) -> Result<PathBuf, &str> {
        let dir_path = cli::get_absolute_path(dir_path);
        // If repo is already in tracking
        for repo in self.active.iter() {
            if repo.path == *dir_path {
                return Err("Repository already tracked, Won't be adding {dir_path} to tracking.");
            }
        }

        if has_repo(&dir_path) {
            let name = dir_path.file_name();
            let repo = Repo {
                // TODO: get rid of unwrap
                name: name.unwrap().to_str().unwrap().to_string(),
                path: dir_path.to_owned(),
            };
            self.active.push(repo);
            self.write();
            Ok(dir_path)
        } else {
            Err("Path passed in is not a repo, via repo::has_repo()")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Repo {
    pub name: String,
    pub path: PathBuf,
}

/// Returns true if the path passed in is a git repo.
/// TODO: Change to handle all types of dirs, not just git repo
fn has_repo(apath: &PathBuf) -> bool {
    let mut path = apath.clone();
    path.push(".git");
    if path.exists() {
        true
    } else {
        false
    }
}
