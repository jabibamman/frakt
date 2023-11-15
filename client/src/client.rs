use server::services::{connect::connect, write::write};

/// Connect to the server and send a message
///
/// # Arguments
///
/// * `host` - Hostname
/// * `port` - Port
///
/// # Return
///
/// * `Result<TcpStream, std::io::Error>` - TcpStream
///
/// # Example
///
/// ```
/// use client::client::connect_server;
///
/// let stream = connect_server("localhost", "8080");
/// ```
///
/// # Panic
///
/// * `std::io::Error` - If the connection failed
///
pub fn connect_server(host: &str, port: &str) -> std::io::Result<std::net::TcpStream> {
    let stream = connect(format!("{}:{}", host, port))?;

    write(stream, "Hello World !")?;

    Ok(())
}
