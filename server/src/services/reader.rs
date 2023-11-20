use std::{io::{Read, self}, net::TcpStream, time::Duration};

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
/// An `io::Result<String>` which is `Ok` containing the JSON string if the read is successful,
/// or an `io::Error` if an error occurs during the read operation.
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
///         Ok(json_msg) => println!("Received JSON: {}", json_msg),
///         Err(e) => eprintln!("Failed to read message: {}", e),
///     }
///
///     Ok(())
/// }
/// ```
pub fn read_message(stream: &mut TcpStream) -> io::Result<String> {
    let mut size_buffer = [0u8; 8];
    stream.read_exact(&mut size_buffer)?;

    let total_size = u32::from_be_bytes([size_buffer[0], size_buffer[1], size_buffer[2], size_buffer[3]]) as usize;
    let json_size = u32::from_be_bytes([size_buffer[4], size_buffer[5], size_buffer[6], size_buffer[7]]) as usize;

    let mut json_buffer = vec![0; json_size];
    stream.read_exact(&mut json_buffer)?;

    let json_str = match String::from_utf8(json_buffer) {
        Ok(str) => str,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    };

    println!("Réponse JSON du serveur: {}", json_str);

    // donnée supplémentaire en binaire
    let binary_data_size = total_size - json_size;
    if binary_data_size > 0 {
        let mut binary_buffer = vec![0; binary_data_size];
        stream.read_exact(&mut binary_buffer)?;
        println!("Données binaires reçues: {:?}", binary_buffer);
    }

    Ok(json_str)
}

pub fn get_response(stream: &mut TcpStream) -> io::Result<String> {
    stream.set_read_timeout(Some(Duration::new(5, 0)))?;
    let response = read_message(stream)?;
    Ok(response)
}

