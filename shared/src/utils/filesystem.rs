use std::{path::PathBuf, env};

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn get_current_working_dir_str() -> String {
    let path_buf = get_current_working_dir().expect("Failed to get current directory");
    path_buf.display().to_string()
}

