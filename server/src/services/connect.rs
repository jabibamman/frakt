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
/// 
/// # Example
/// 
/// ```rust
/// use server::services::connect::connect;
/// 
/// let stream = connect("localhost:8080");
/// ```
pub fn connect(address: &str) -> Result<TcpStream> {
    let stream = TcpStream::connect(address)?;

    Ok(stream)
}