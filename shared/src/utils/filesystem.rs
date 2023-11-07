use std::{path::PathBuf, env};

use crate::types::filesystem::{DirType, FileExtension};

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn get_workspace_dir() -> std::io::Result<PathBuf> {
    let mut path_buf = get_current_working_dir()?;
    path_buf.pop();
    Ok(path_buf)
}

pub fn get_dir_str(dir_type: DirType) -> String {
    match dir_type {
        DirType::Current => get_current_working_dir().expect("Failed to get the current directory").display().to_string(),
        DirType::Workspace => get_workspace_dir().expect("Failed to get the workspace directory").display().to_string(),
    }
}

pub fn get_extension_str(extension: FileExtension) -> &'static str {
    match extension {
        FileExtension::PNG => "png",
        FileExtension::JPG => "jpg",
        FileExtension::JPEG => "jpeg",
    }
}

pub fn dir_exists(path: &str) -> bool {
    let path_buf = PathBuf::from(path);
    path_buf.exists()
}

