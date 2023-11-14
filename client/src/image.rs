use std::process::Command;

use shared::utils::filesystem::dir_exists;

/// Opens an image file with the default image viewer of the system.
///
/// This function checks if the provided path exists and then opens the image file
/// using the appropriate command based on the operating system.
///
/// # Arguments
/// * `path` - A string slice that holds the path to the image file.
///
/// # OS Specific Behavior
/// - Windows: Uses `cmd /c start` to open the image.
/// - Linux: Uses `xdg-open` to open the image.
/// - macOS: Uses `open` to open the image.
/// - Other OS: Prints "OS not supported" message.
///
/// # Panics
/// Panics if the command to open the image cannot be spawned.
pub fn open_image(path: &str) -> () {
    if !dir_exists(path) {
        return;
    }

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/c", "start", path])
            .spawn()
            .unwrap();
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn().unwrap();
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn().unwrap();
    } else {
        println!("OS not supported");
    }
}
