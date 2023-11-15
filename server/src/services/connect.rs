use std::io::Result;
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
pub fn connect(address: &str) -> Result<TcpStream> {
    let stream = TcpStream::connect(address)?;

    Ok(stream)
}
