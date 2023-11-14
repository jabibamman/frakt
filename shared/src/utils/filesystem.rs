use std::{env, path::PathBuf};

use crate::types::filesystem::FileExtension;

use rand::random;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn get_workspace_dir() -> std::io::Result<PathBuf> {
    Ok(get_current_working_dir()?)
}

pub fn get_dir_path_buf() -> PathBuf {
    if cfg!(not(debug_assertions)) {
        get_current_working_dir().expect("Failed to get the current directory")
    } else {
        let mut workspace_dir = get_workspace_dir().expect("Failed to get the workspace directory");
        workspace_dir.push("target");
        workspace_dir
    }
}

pub fn get_extension_str(extension: FileExtension) -> &'static str {
    match extension {
        FileExtension::PNG => "png",
        FileExtension::JPG => "jpg",
        FileExtension::JPEG => "jpeg",
    }
}

pub fn get_file_path(filename: &str, path: PathBuf, extension: &str) -> String {
    let mut path_buf = path;
    let file_name_with_extension = format!("{}-{}.{}", filename, random::<u32>(), extension);
    path_buf.push(file_name_with_extension);
    path_buf.to_str().unwrap_or_default().to_string()
}

pub fn dir_exists(path: &str) -> bool {
    let path_buf = PathBuf::from(path);
    path_buf.exists() && path_buf.is_dir()
}

#[cfg(test)]
mod tests {
    use super::dir_exists;
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use tempfile::NamedTempFile;

    #[test]
    fn test_dir_exists_with_tempfile() {
        let temp_dir = tempdir().expect("Failed to create a temporary directory");
        let temp_path = temp_dir.path();

        let temp_file = NamedTempFile::new_in(&temp_dir).expect("Failed to create a NamedTempFile");
        let temp_file_path = temp_file.path();

        assert!(dir_exists(
            temp_path
                .to_str()
                .expect("Failed to convert path to string")
        ));
        assert!(temp_file_path.exists());
    }

    #[test]
    fn test_dir_without_tempfile() {
        let file_path = PathBuf::from("test.txt");
        assert!(!dir_exists(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_get_dir_str_current() {
        let current_dir = get_dir_path_buf().to_str().unwrap_or_default().to_string();
        assert_ne!(current_dir, "");
    }

    #[test]
    fn test_get_dir_str_workspace() {
        let workspace_dir = get_dir_path_buf().to_str().unwrap_or_default().to_string();
        assert_ne!(workspace_dir, "");
    }

    #[test]
    fn test_get_extension_str_png() {
        let extension = get_extension_str(FileExtension::PNG);
        assert_eq!(extension, "png");
    }

    #[test]
    fn test_get_extension_str_jpg() {
        let extension = get_extension_str(FileExtension::JPG);
        assert_eq!(extension, "jpg");
    }

    #[test]
    fn test_get_extension_str_jpeg() {
        let extension = get_extension_str(FileExtension::JPEG);
        assert_eq!(extension, "jpeg");
    }
}
