use std::io::Result;
use std::io::Write;
use std::net::TcpStream;

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
/// ```no_run
/// use std::net::TcpStream;
/// use server::services::write::write;
/// 
/// let stream = TcpStream::connect("localhost:8787").unwrap();
/// let stream = write(stream, "Hello world!").unwrap();
/// ```
/// 
pub fn write(mut stream: TcpStream, message: &str) -> Result<TcpStream> {
    stream.write(format!("${}", message).as_bytes())?;

    Ok(stream)
}
