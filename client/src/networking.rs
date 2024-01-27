use std::{
    io,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use cli::parser::CliClientArgs;
use image::{ImageBuffer, Rgb};
use log::{debug, error, info};
use server::services::{
    reader::get_response,
    write::{write, write_img},
};
use shared::{
    types::{
        error::FractalError,
        filesystem::FileExtension,
        messages::{FragmentResult, FragmentTask},
        pixel_data::PixelData,
    },
    utils::{
        filesystem::{get_dir_path_buf, get_extension_str, get_file_path},
        fragment_result_impl::FragmentResultOperation,
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
/// * `Result<Option<FragmentTask, Vec<u8>>>, FractalError>` - A `FragmentTask` if the response was successful, or a `FractalError` if the response failed.
///
pub fn receive_fragment_task(
    stream: &mut TcpStream,
) -> Result<Option<(FragmentTask, Vec<u8>)>, FractalError> {
    let (fragment_task, data) = get_response(stream)?;
    debug!("Received response: {}", fragment_task);
    debug!("Received data in receive fragment task: {:?}", data);
    match FragmentTask::deserialize(&fragment_task) {
        Ok(task) => Ok(Some((task, data))),
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
/// * `data` - A `Vec<u8>` containing the image data.
/// * `open_after_save` - A boolean indicating whether the image should be opened after it is saved.
///
/// # Return
///
/// * `Result<(TcpStream), FractalError>` - A `TcpStream` if the connection was successful, or a `FractalError` if the connection failed.
///
pub fn process_fragment_task(
    task: FragmentTask,
    data: Vec<u8>,
    cli_args: &CliClientArgs,
) -> Result<TcpStream, FractalError> {
    let dir_path_buf = get_dir_path_buf()?;

    let img_path = get_file_path("julia", dir_path_buf, get_extension_str(FileExtension::PNG))?;
    let (img, pixel_data_bytes, pixel_intensity_matrice) = generate_fractal_set(task.clone())?;

    debug!("Pixel data bytes: {:?}", pixel_data_bytes);

    let pixel_data = convert_to_pixel_data(pixel_data_bytes.clone(), task.clone());

    let mut vec_data = data.clone();

    for i in 0..pixel_intensity_matrice.len() {
        vec_data.extend(pixel_intensity_matrice[i].zn.to_be_bytes());
        vec_data.extend(pixel_intensity_matrice[i].count.to_be_bytes());
    }

    debug!("Vec data: {:?}", vec_data);

    if cli_args.save {
        save_fractal_image(img.clone(), &img_path)?;
    }

    let stream = send_fragment_result(&task, cli_args, pixel_data, vec_data)?;

    if cli_args.open {
        open_image(&img_path)?;
    }

    Ok(stream)
}

/// Send a `FragmentResult` to the server after generating a fractal image.
///
/// # Arguments
///
/// * `img` - An `ImageBuffer` containing the fractal image.
/// * `fragment_task` - A `FragmentTask` containing details such as the fractal type, resolution, and range.
/// * `cli_args` - A `CliClientArgs` containing the command line arguments.
/// * `pixel_data` - A `PixelData` struct containing the image data.
/// * `data` - A `Vec<u8>` containing the image data.
///
/// # Return
///
/// * `Result<(), FractalError>` - An `io::Error` if the image could not be saved.
///
/// # Details
///
/// This function converts the `ImageBuffer` to a `Vec<u8>` and then to a `PixelData` struct.
///
fn send_fragment_result(
    fragment_task: &FragmentTask,
    cli_args: &CliClientArgs,
    pixel_data: PixelData,
    data: Vec<u8>,
) -> Result<TcpStream, FractalError> {
    let fragment_result = FragmentResult::new(
        fragment_task.id,
        fragment_task.resolution,
        fragment_task.range,
        pixel_data,
    );
    let serialized = FragmentResult::serialize(&fragment_result)?;

    let mut new_stream = connect_to_server(cli_args)?;
    debug!("Sending fragment result: {}", serialized);
    write_img(&mut new_stream, &serialized, data)?;

    Ok(new_stream)
}

/// Convert a `Vec<u8>` to a `PixelData` struct.
///
/// # Arguments
///
/// * `data` - A `Vec<u8>` containing the image data.
///
/// # Return
///
/// * `PixelData` - A `PixelData` struct containing the image data.
///
/// # Details
///
/// This function converts the `Vec<u8>` to a `PixelData` struct.
fn convert_to_pixel_data(data: Vec<u8>, task: FragmentTask) -> PixelData {
    let pixel_size = 3; // Pour RGB, 4 pour RGBA, etc.
    let total_pixels = data.len() / pixel_size;

    PixelData {
        offset: task.id.count,
        count: total_pixels as u32,
    }
}
