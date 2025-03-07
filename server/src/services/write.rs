use std::io::{self, BufWriter};
use std::io::Write;
use std::net::TcpStream;

use log::{debug, trace};

/// Prepare a message for sending.
///
/// # Arguments
///
/// * `message` - A string slice (`&str`) representing the message to be formatted.
///
/// # Return
///
/// * `Vec<u8>` - The formatted message as a vector of bytes (`Vec<u8>`).
///
/// This function takes a string slice,
/// and then converts it into a vector of bytes.
fn prepare_message(message: &str) -> Vec<u8> {
    format!("{}", message).as_bytes().to_vec()
}

/// Write a string to a stream
///
/// # Arguments
///
/// * `stream` - TcpStream
/// * `message` - String
///
/// # Return
///
/// * `Result<TcpStream, std::io::Error>` - TcpStream
///
/// # Example
///
/// * `Result<TcpStream, std::io::Error>` - This function returns a `Result` type. On success,
///   it returns the modified `TcpStream` wrapped in `Ok`. On failure, it returns an `std::io::Error`.
///
/// This function prepares a message by calling `prepare_message`, clones the provided TCP stream,
/// writes the prepared message to the cloned stream, and then returns the original stream.
///
/// ```no_run
/// use std::net::TcpStream;
/// use server::services::write::write;
///
/// let mut stream = TcpStream::connect("localhost:8787").unwrap();
/// let stream = write(&mut stream, "Hello world!").unwrap();
/// ```
///
pub fn write(stream: &mut TcpStream, message: &str) -> io::Result<()> {
    let mut stream_clone = stream.try_clone()?;

    let message_bytes = prepare_message(message);
    let json_size = (message.len() as u32).to_be_bytes();
    let total_size = (message.len() as u32).to_be_bytes();

    stream_clone.write_all(&total_size)?;
    stream_clone.write_all(&json_size)?;
    stream_clone.write_all(&message_bytes)?;

    Ok(())
}

/// Prepare a message for sending.
///
/// # Arguments
///
/// * `message` - A string slice (`&str`) representing the message to be formatted.
///
/// # Return
///
/// * `Vec<u8>` - The formatted message as a vector of bytes (`Vec<u8>`).
///
///
/// This function takes a string slice and converts it into a vector of bytes. It formats the message by calling `format!` macro with the provided message and converts the formatted message into bytes using `as_bytes()` method. The resulting bytes are then converted into a vector using `to_vec()` method and returned.
pub fn write_img(stream: &mut TcpStream, message: &str, img_data: Vec<u8>) -> io::Result<()> {
    let mut writer = BufWriter::new(stream);
    
    let json_bytes = prepare_message(message);
    let json_size = (message.len() as u32).to_be_bytes();
    let message_length = message.len() + img_data.len();
    let total_size = (message_length as u32).to_be_bytes();

    let mut buffer = Vec::new();
    buffer.extend_from_slice(&total_size);
    buffer.extend_from_slice(&json_size);
    buffer.extend_from_slice(&json_bytes);
    buffer.extend_from_slice(&img_data);

    for byte in buffer {
        writer.write_all(&[byte])?;
        trace!("Writing byte: {}", byte);
    }
    writer.flush()?;
    debug!("Données envoyées");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Result};
    use std::net::TcpListener;
    use std::thread;

    /// Test for `prepare_message` function.
    /// It verifies that the function correctly formats a given message.
    #[test]
    fn test_prepare_message() {
        let msg = "Hello, world!";
        let prepared_msg = prepare_message(msg);
        assert_eq!(prepared_msg, format!("{}", msg).as_bytes().to_vec());
    }

    /// Test for `write` function in a simulated server-client setup.
    /// This test sets up a TCP listener and tests if the `write` function
    /// can successfully write to a stream.
    #[test]
    fn test_write() -> Result<()> {
        thread::spawn(|| {
            let listener = TcpListener::bind("localhost:8787").unwrap();
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
            }
        });

        Ok(())
    }
}
