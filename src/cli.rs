use std::path::{Path, PathBuf};

use path_absolutize::Absolutize;

use crate::{config, repo::Tracking, Command};

/// Handles the CLI commands
pub fn run(command: Command) {
    let mut tracking = Tracking::new();
    match command {
        Command::Add { path } => {
            let res = tracking.add(path);
            match res {
                Ok(r) => println!("Added {}", r.display()),
                Err(e) => println!("{}", e),
            }
        }
        Command::Delete { path } => {
            let res = tracking.delete(
                get_absolute_path(path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
            match res {
                Ok(r) => println!("Deleted {}", r),
                Err(e) => println!("{}", e),
            }
        }
        Command::List => {
            let res = tracking.list();
            match res {
                Ok(r) => {
                    for repo in r {
                        println!("{:#?}", repo);
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}

/// Given a path in any format (relative, absolute, etc..) give back the absolute path.
pub fn get_absolute_path(path: PathBuf) -> PathBuf {
    let p = Path::new(&path);
    p.absolutize().unwrap().to_path_buf()
}
