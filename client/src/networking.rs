use std::{
    io,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use cli::parser::CliClientArgs;
use image::{ImageBuffer, Rgb};
use log::{debug, error, info};
use server::services::{reader::get_response, write::write};
use shared::{
    types::{error::FractalError, filesystem::FileExtension, messages::FragmentTask},
    utils::{
        filesystem::{get_dir_path_buf, get_extension_str, get_file_path},
        fragment_task_impl::FragmentTaskOperation,
    },
};

use crate::{fractal_generation::generate_fractal_set, image::open_image};

/// Connect to a server at the specified address.
///
/// # Arguments
///
/// * `address` - Address
///
/// # Return
///
/// * `Result<TcpStream, FractalError>` - A `TcpStream` if the connection was successful, or a `FractalError` if the connection failed.
///
pub fn connect_to_server(cli_args: &CliClientArgs) -> Result<TcpStream, FractalError> {
    let address = format!("{}:{}", cli_args.hostname, cli_args.port);
    info!("Connecting to server at {}...", address);
    let addrs = address
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| FractalError::ConnectionError("Unable to resolve address".to_string()))?;

    let timeout_duration = Duration::from_secs(1);
    TcpStream::connect_timeout(&addrs, timeout_duration).map_err(FractalError::Io)
}

/// Send a request to the server.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the `TcpStream` to send the request to.
/// * `serialized_request` - A reference to the serialized request.
///
/// # Return
///
/// * `Result<(), io::Error>` - An `io::Error` if the request could not be sent.
///
pub fn send_request(stream: &mut TcpStream, serialized_request: &str) -> io::Result<()> {
    info!("Sending request to server...");
    write(stream, serialized_request)
}

/// Receive a `FragmentTask` from the server.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the `TcpStream` to receive the response from.
///
/// # Return
///
/// * `Result<Option<FragmentTask>, FractalError>` - A `Result` containing an `Option<FragmentTask>` if the response was received successfully, or a `FractalError` if the response could not be received.
///
pub fn receive_fragment_task(stream: &mut TcpStream) -> Result<Option<FragmentTask>, FractalError> {
    let response = get_response(stream)?;
    debug!("Received response: {}", response);
    match FragmentTask::deserialize(&response) {
        Ok(fract) => Ok(Some(fract)),
        Err(e) => {
            error!("Deserialization error: {}", e);
            Err(FractalError::TaskNotSet(format!(
                "Deserialization error: {}",
                e
            )))
        }
    }
}

/// Save a fractal image to the filesystem.
///
/// # Arguments
///
/// * `img` - An `ImageBuffer` containing the fractal image.
/// * `img_path` - A reference to the path where the image will be saved.
///
/// # Return
///
/// * `Result<(), FractalError>` - An `io::Error` if the image could not be saved.
///
pub fn save_fractal_image(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    img_path: &str,
) -> Result<(), FractalError> {
    img.save(img_path).map_err(FractalError::Image)?;
    info!("Image saved successfully");
    debug!("Image path {}", img_path);
    Ok(())
}

/// Process a `FragmentTask` by generating a fractal image and saving it to the filesystem.
///
/// # Arguments
///
/// * `task` - A `FragmentTask` containing details such as the fractal type, resolution, and range.
/// * `open_after_save` - A boolean indicating whether the image should be opened after it is saved.
///
/// # Return
///
/// * `Result<(), FractalError>` - An `io::Error` if the image could not be saved.
///
/// # Details
///
/// This function generates a file path for the image, generates the fractal image, saves it to the filesystem, and opens it if `open_after_save` is true.
///
pub fn process_fragment_task(
    task: FragmentTask,
    open_after_save: bool,
) -> Result<(), FractalError> {
    let dir_path_buf = get_dir_path_buf()?;

    let img_path = get_file_path("julia", dir_path_buf, get_extension_str(FileExtension::PNG))?;
    let img = generate_fractal_set(task)?;
    save_fractal_image(img, &img_path)?;

    if open_after_save {
        open_image(&img_path)?;
    }

    Ok(())
}
