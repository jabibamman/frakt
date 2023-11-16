use std::io;
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
/// - Other OS: Returns an `Err` indicating that the OS is not supported.
///
/// # Errors
/// Returns an `Err` if the image file does not exist or if the command to open the image fails to spawn.
///
/// # Examples
/// ```
/// use my_crate::open_image; // Replace with the actual path to your function
///
/// match open_image("/path/to/image.png") {
///     Ok(_) => println!("Image opened successfully"),
///     Err(e) => println!("Failed to open image: {}", e),
/// }
/// ```
pub fn open_image(path: &str) -> Result<(), io::Error> {
    if !dir_exists(path) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Image file not found",
        ));
    }

    let result = if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/c", "start", path]).spawn()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).spawn()
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "OS not supported"));
    };

    result.map(|_| ())
}
