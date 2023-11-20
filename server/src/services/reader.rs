use std::{io::{Read, self}, net::TcpStream, time::Duration};

/// Read a message from a TCP stream.
///
/// This function reads data from the given TCP stream up to 1024 bytes.
/// It assumes that the message is UTF-8 encoded. If the message contains
/// non-UTF-8 bytes, they will be replaced by U+FFFD REPLACEMENT CHARACTER.
/// The function also trims null characters from the message.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the TCP stream from which to read the message.
///
/// # Returns
///
/// Returns a `String` containing the message read from the stream.
///
/// # Examples
///
/// ```no_run
/// use std::net::{TcpListener, TcpStream};
/// use server::services::reader::read_message;
///
/// let listener = TcpListener::bind("127.0.0.1:0").unwrap();
/// let address = listener.local_addr().unwrap();
/// let mut stream = TcpStream::connect(address).unwrap();
///
/// let message = read_message(stream);
/// println!("Received message: {}", message);
/// ```
///
/// # Panics
///
/// Panics if the reading from the stream fails or if the buffer cannot be
/// converted to a UTF-8 string.
///
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

