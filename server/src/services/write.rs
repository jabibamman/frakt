use std::io::Result;
use std::net::TcpStream;
use std::io::Write;

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
pub fn write(mut stream: TcpStream, message: &str) -> Result<TcpStream> {

    stream.write(format!("${}", message).as_bytes())?;

    Ok(stream)
}