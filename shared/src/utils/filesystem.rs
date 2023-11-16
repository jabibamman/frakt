/// Provides functions for file system operations, focusing on directories and file extensions.
/// Includes utilities for working with the current working directory, workspace directory,
/// and checking if a directory exists.
use std::{env, path::PathBuf, io};
use crate::types::filesystem::FileExtension;

use rand::random;

/// Returns the current working directory as a `PathBuf`.
/// Propagates any errors encountered.
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

/// Obtains the workspace directory. Typically the current working directory.
/// Errors are propagated from `get_current_working_dir`.
pub fn get_workspace_dir() -> std::io::Result<PathBuf> {
    Ok(get_current_working_dir()?)
}

/// Returns a `PathBuf` representing a directory path.
/// 
/// This function determines the appropriate directory path based on the current
/// mode (debug or release) and an optional workspace directory.
/// In release mode, it returns the current working directory.
/// In debug mode, it recursively determines the workspace directory,
/// appending "target" to it.
///
/// # Parameters
/// - `is_debug_mode`: A boolean indicating whether the function is running in debug mode.
/// - `workspace_dir`: An optional `PathBuf` representing the workspace directory.
///
/// # Returns
/// `std::result::Result<PathBuf, io::Error>`
/// - `Ok(PathBuf)`: The directory path as `PathBuf`.
/// - `Err(io::Error)`: An error occurred while determining the directory path.
///
/// # Examples
/// ```
/// use shared::utils::filesystem::get_dir_path_buf;
///
/// // Retrieve the directory path in the current mode
/// let path_buf = get_dir_path_buf().expect("Failed to get directory path");
/// println!("Directory Path: {:?}", path_buf);
/// ```
///
/// # Note
/// This function uses a recursive approach in debug mode to determine the workspace
/// directory. It may append "target" to the workspace directory in this case.
pub fn get_dir_path_buf() -> std::result::Result<PathBuf, io::Error> {
    fn recursive_get_dir_path_buf(is_debug_mode: bool, workspace_dir: Option<PathBuf>) -> std::result::Result<PathBuf, io::Error> {
        if !is_debug_mode {
            get_current_working_dir()
        } else {
            match workspace_dir {
                None => {
                    let dir = get_workspace_dir()?;
                    recursive_get_dir_path_buf(true, Some(dir))
                }
                Some(dir) => {
                    let new_dir = dir.join("target");
                    Ok(new_dir)
                }
            }
        }
    }

    recursive_get_dir_path_buf(cfg!(debug_assertions), None)
}

/// Maps a `FileExtension` enum variant to its corresponding file extension string.
/// Returns `png`, `jpg`, or `jpeg`.
pub fn get_extension_str(extension: FileExtension) -> &'static str {
    match extension {
        FileExtension::PNG => "png",
        FileExtension::JPG => "jpg",
        FileExtension::JPEG => "jpeg",
    }
}

/// Generates a file path string with a random component in the filename.
/// Constructs the path from the given filename, path, and extension.
/// Ensures unique filenames.
///
/// # Arguments
/// * `filename` - Base filename without extension.
/// * `path` - PathBuf where the file will be located.
/// * `extension` - File extension.
///
/// # Returns
/// Returns a `Result<String, String>` where `Ok` contains the file path, and `Err` contains an error message.
///
/// # Examples
/// ```
/// use std::path::PathBuf;
/// use shared::utils::filesystem::get_file_path;
///
/// let path = PathBuf::from("/some/directory");
/// match get_file_path("myfile", path, "txt") {
///     Ok(file_path) => println!("Generated file path: {}", file_path),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn get_file_path(filename: &str, path: PathBuf, extension: &str) -> Result<String, String> {
    let file_name_with_extension = format!("{}-{}.{}", filename, random::<u32>(), extension);
    let new_path = path.join(file_name_with_extension);
    new_path.to_str()
        .ok_or_else(|| "Failed to convert the path to a string".to_string())
        .map(|s| s.to_string())
}

/// Checks if a given path string represents an existing directory.
/// Returns `true` if the path exists and is a directory, `false` otherwise.
pub fn dir_exists(path: &str) -> bool {
    let path_buf = PathBuf::from(path);

    path_buf.exists()
}

/// The module includes tests for each utility function, ensuring their correct functionality.
/// These tests cover scenarios like checking if a directory exists, getting directory strings in different modes,
///  and validating file extension strings.
#[cfg(test)]
mod tests {
    use super::dir_exists;
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use tempfile::NamedTempFile;
    use std::io::{self, ErrorKind};


    #[test]
    fn test_dir_exists_with_tempfile() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();
        let temp_file = NamedTempFile::new_in(&temp_dir)?;
        let temp_file_path = temp_file.path();
        let path_str = temp_path.to_str().ok_or("Failed to convert path to string")?;

        assert!(dir_exists(path_str));
        assert!(temp_file_path.exists());
    
        Ok(())
    }
    

    #[test]
    fn test_dir_without_tempfile() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = PathBuf::from("test.txt");
        let file_path_str = file_path.to_str().ok_or("Failed to convert path to string")?;
        assert!(!dir_exists(file_path_str));
    
        Ok(())
    }

    #[test]
    fn test_get_dir_str_current() -> Result<(), Box<dyn std::error::Error>> {
        let current_dir_result: Result<String, io::Error> = get_dir_path_buf()
            .and_then(|path| {
                path.to_str()
                    .ok_or_else(|| io::Error::new(ErrorKind::Other, "Failed to convert path to string"))
                    .map(|s| s.to_owned())
            }
        );

        assert!(current_dir_result.is_ok());    
        let dir_str = current_dir_result?;
        assert_ne!(dir_str, "");
    
        Ok(())
    }
    
    #[test]
    fn test_get_dir_str_workspace() -> Result<(), Box<dyn std::error::Error>> {
        let workspace_dir_result = get_dir_path_buf()
            .and_then(|path| {
                path.to_str()
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to convert path to string"))
                    .map(|s| s.to_owned())
            });
    
        assert!(workspace_dir_result.is_ok());
        
        let dir_str = workspace_dir_result?;
    
        assert_ne!(dir_str, "");
    
        Ok(())
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
