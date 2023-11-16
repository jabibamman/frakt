use std::{io::Read, net::TcpStream};

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
pub fn read_message(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let message = String::from_utf8_lossy(&buffer[..]);
    let message = message.trim_matches(char::from(0)).to_string();

    message
}
