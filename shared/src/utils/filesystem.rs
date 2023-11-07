use std::{path::PathBuf, env};

use crate::types::filesystem::DirType;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn get_workspace_dir() -> std::io::Result<PathBuf> {
    let mut path_buf = get_current_working_dir()?;
    path_buf.pop();
    Ok(path_buf)
}

pub fn get_dir_str(dir_type: DirType) -> String {
    let path_buf = match dir_type {
        DirType::Current => get_current_working_dir(),
        DirType::Workspace => get_workspace_dir(),
    }.expect("Failed to get the directory");

    path_buf.display().to_string()
}


pub fn dir_exists(path: &str) -> bool {
    let path_buf = PathBuf::from(path);
    path_buf.exists()
}

