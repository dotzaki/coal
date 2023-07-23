use std::path::PathBuf;

use directories::ProjectDirs;

const CONFIG_FILE_NAME: &str = "config.yaml";
const QUALIFIER: &str = "";
const ORGANIZATION_NAME: &str = "dotzaki";
const APP_NAME: &str = "coal";

/// Check if the state file exists if not create an empty file.
pub fn get_path() -> PathBuf {
    if let Some(proj_dir) = ProjectDirs::from(QUALIFIER, ORGANIZATION_NAME, APP_NAME) {
        let config_dir = proj_dir.config_dir();
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).unwrap();
        }
        let config = config_dir.join(CONFIG_FILE_NAME);
        if !config.exists() {
            std::fs::File::create(&config).unwrap();
        }
        config
    } else {
        //FIX: Maybe don't panic here
        panic!("Could not find project directory");
    }
}
