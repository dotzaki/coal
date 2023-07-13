use std::path::{Path, PathBuf};

use path_absolutize::Absolutize;

use crate::{repo::Tracking, Command};

/// Handles the CLI commands
/// TODO: Make the output more pretty
pub fn run(command: Command, mut tracking: Tracking) {
    match command {
        Command::Add { path } => {
            let res = tracking.add(&path);
            println!("{:?}", res);
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
            println!("{:?}", res);
        }
        Command::List => {
            let list = tracking.list();
            println!("{:?}", list);
        }
    }
}

/// Given a path in any format (relative, absolute, etc..) give back the absolute path.
fn get_absolute_path(path: PathBuf) -> PathBuf {
    let p = Path::new(&path);
    p.absolutize().unwrap().to_path_buf()
}
