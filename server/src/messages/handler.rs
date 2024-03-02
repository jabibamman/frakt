#[cfg(target_os = "windows")]
use gui::create_window_and_display_image;

use log::{debug, error, info};
use shared::{
    types::{
        error::FractalError, filesystem::FileExtension, messages::Message,
        pixel_intensity::PixelIntensity,
    },
    utils::{
        filesystem::{get_dir_path_buf, get_extension_str, get_file_path}, fragment_task_impl::FragmentTaskOperation, image::image_from_pixel_intensity
    },
};
use std::{
    io::{self, Read},
    net::TcpStream,
};

use tokio::task;
use super::serialization::deserialize_message;
use crate::{messages::fragment_maker::create_tasks, services};

/// Handles a client TCP stream.
///
/// This function is responsible for managing a single client TCP stream. It performs the following actions:
/// - Retrieves and prints the local address of the stream.
/// - Reads data from the stream into a buffer and prints the received message.
/// - Handles potential errors in both steps and prints relevant error messages.
///
/// ## Parameters
/// - `stream`: A mutable reference to a `TcpStream`. This stream represents the connection with the client.
///
/// ## Panics
/// This function doesn't explicitly panic, but operations on `stream` (like `read`) might panic if they encounter irrecoverable errors.
///
/// ## Example Usage
/// Note: This example is a conceptual representation and may not work as-is.
/// ```no_run
/// use std::net::{TcpListener, TcpStream};
/// use server::messages::handler::handle_client;
///
/// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
/// for stream in listener.incoming() {
///     if let Ok(stream) = stream {
///         handle_client(stream);
///     }
/// }
/// ```
pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    info!("Handling client");
    match stream.local_addr() {
        Ok(addr) => info!("Connection established {}", addr),
        Err(e) => error!("Failed to get local address: {}", e),
    }

    let mut total_size_buffer = [0; 4];
    stream.read_exact(&mut total_size_buffer)?;
    let total_size = u32::from_be_bytes(total_size_buffer) as usize;
    debug!("Total size: {}", total_size);

    let mut json_size_buffer = [0; 4];
    stream.read_exact(&mut json_size_buffer)?;
    let json_size = u32::from_be_bytes(json_size_buffer) as usize;
    debug!("JSON size: {}", json_size);

    let mut buffer = vec![0; json_size];
    stream.read_exact(&mut buffer)?;

    let json_str = std::str::from_utf8(&buffer[0..json_size]).map_err(|e| {
        error!("Invalid UTF-8 sequence: {}", e);
        io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")
    })?;

    debug!("Received JSON: {}", json_str);
    let data_str = match std::str::from_utf8(&buffer) {
        Ok(str) => str,
        Err(e) => {
            error!("Invalid UTF-8 sequence: {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
        }
    };

    let mut pixel_intensity = vec![];
    if total_size - json_size > 0 {
        let mut buffer = vec![0; total_size - json_size];
        stream.read_exact(&mut buffer)?;
        debug!("Received data: {:?}", buffer);
        pixel_intensity = PixelIntensity::vec_data_to_pixel_intensity_matrix(buffer);
    }

    debug!("Received data: {}", data_str);

    let message_result = deserialize_message(data_str);
    debug!("Deserialized data {:?}", message_result);

    match message_result {
        Ok(Message::FragmentRequest(_request)) => {
            let task = match create_tasks() {
                Ok(task) => task[0].clone(),
                Err(e) => {
                    error!("Error creating task: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
                }
            };

            debug!("Task created: {:?}", task.clone());

            let serialized_task = match task.serialize() {
                Ok(serialized_task) => serialized_task,
                Err(e) => {
                    error!("Error serializing task: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
                }
            };
            info!(
                "Sending serialized task to client at {}",
                stream.peer_addr()?
            );

            services::write::write(&mut stream, &serialized_task)?;

            let response = services::reader::get_response(&mut stream)?;

            debug!("Received response: {:?}", response);
        }
        Ok(Message::FragmentTask(_task)) => {
            error!("Received unexpected message type: FragmentTask");
        }
        Ok(Message::FragmentResult(_result)) => {
            let img = match image_from_pixel_intensity(pixel_intensity) {
                Ok(img) => img,
                Err(e) => {
                    error!("Error creating image from pixel intensity: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
                }
            };

            let img_clone = img.clone();

            #[cfg(target_os = "windows")]
            task::spawn_blocking(move || {
                if let Err(e) = create_window_and_display_image(&img) {
                    error!("Error creating window and displaying image: {:?}", e);
                }
            });

            let dir_path_buf = match get_dir_path_buf() {
                Ok(dir_path_buf) => dir_path_buf,
                Err(e) => {
                    error!("Error getting directory path: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
                }
            };

            let img_path: String = match get_file_path(
                "test-23_02_23",
                dir_path_buf,
                get_extension_str(FileExtension::PNG),
            ) {
                Ok(img_path) => img_path,
                Err(e) => {
                    error!("Error getting file path: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
                }
            };

            match img_clone.save(img_path.clone()).map_err(FractalError::Image) {
                Ok(_) => {
                    info!("Image saved successfully");
                    debug!("Image path {}", img_path);
                }
                Err(e) => {
                    error!("Error saving image: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
                }
            }
        }
        Err(e) => {
            error!("Error deserializing request: {:?}", e);
        }
    }

    Ok(())
}
