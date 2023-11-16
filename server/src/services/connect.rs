use std::io;
use std::net::TcpStream;

/// Connect to a server
///
/// # Arguments
///
/// * `address` - Address
///
/// # Return
///
/// * `Result<TcpStream, std::io::Error>` - TcpStream
pub fn connect(address: &str) -> io::Result<TcpStream> {
    let stream = TcpStream::connect(address)?;

    Ok(stream)
}
