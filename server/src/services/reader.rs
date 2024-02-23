use std::{
    io::{self, Read},
    net::TcpStream,
};

use log::debug;

/// Reads a message from a TCP stream, parsing and returning the JSON component.
///
/// This function first reads the message size and JSON size from the stream.
/// It then reads the JSON data based on the specified size and converts it into a `String`.
/// If there's any additional binary data following the JSON data, it also reads that.
/// The function handles any errors that occur during reading and parsing.
///
/// # Parameters
/// * `stream`: A mutable reference to a `TcpStream` from which the message will be read.
///
/// # Returns
/// An `io::Result` containing a tuple of the JSON data as a `String` and the binary data as a `Vec<u8>`.
/// The binary data is empty if there's no additional data.
///
/// # Errors
/// Returns an `io::Error` if there's an issue with stream reading or if the JSON data
/// cannot be converted into a UTF-8 string.
///
/// # Examples
/// ```no_run
/// use std::io;
/// use std::net::TcpStream;
/// use server::services::reader::read_message;
///
/// fn main() -> io::Result<()> {
///     let mut stream = TcpStream::connect("localhost:8787").unwrap();
///     match read_message(&mut stream) {
///         Ok((json_str, data)) => println!("Message: {}, Data: {:?}", json_str, data),
///         Err(e) => eprintln!("Failed to read message: {}", e),
///     }
///
///     Ok(())
/// }
/// ```
pub fn read_message(stream: &mut TcpStream) -> io::Result<(String, Vec<u8>)> {
    let mut total_message = [0; 4];
    let mut json_message = [0; 4];

    stream.read_exact(&mut total_message)?;
    stream.read_exact(&mut json_message)?;

    let json_size = u32::from_be_bytes(json_message) as usize;
    let total_size = u32::from_be_bytes(total_message) as usize;

    let mut json_buffer = vec![0; json_size];
    stream.read_exact(&mut json_buffer)?;

    let json_str = match String::from_utf8(json_buffer) {
        Ok(str) => str,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    };

    debug!("Réponse String message du serveur: {}", json_str);

    // donnée supplémentaire en binaire
    let binary_data_size = total_size - json_size;
    let mut data = Vec::new();
    if binary_data_size > 0 {
        data = vec![0; binary_data_size];
        stream.read_exact(&mut data)?;
    }

    Ok((json_str, data))
}

pub fn get_response(stream: &mut TcpStream) -> io::Result<(String, Vec<u8>)> {
    Ok(read_message(stream)?)
}
