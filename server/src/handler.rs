use std::{io::Read, net::TcpStream};

/// Handles a client TCP stream.
///
/// This function is responsible for managing a single client TCP stream. It performs the following actions:
/// - Retrieves and prints the local address of the stream.
/// - Reads data from the stream into a buffer and prints the received message.
/// - Handles potential errors in both steps and prints relevant error messages.
///
/// ## Parameters
/// - `stream`: A mutable reference to a `TcpStream`. This stream represents the connection with the client.
///
/// ## Panics
/// This function doesn't explicitly panic, but operations on `stream` (like `read`) might panic if they encounter irrecoverable errors.
///
/// ## Example Usage
/// Note: This example is a conceptual representation and may not work as-is.
/// ```no_run
/// use std::net::{TcpListener, TcpStream};
/// use server::handler::handle_client;
///
/// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
/// for stream in listener.incoming() {
///     if let Ok(stream) = stream {
///         handle_client(stream);
///     }
/// }
/// ```
pub fn handle_client(mut stream: TcpStream) {
    match stream.local_addr() {
        Ok(addr) => println!("[SERVER] Connection established {}", addr),
        Err(e) => println!("[SERVER] Failed to get local address: {}", e),
    }

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => println!(
            "[SERVER] Message received: {}",
            String::from_utf8_lossy(&buffer[..])
        ),
        Err(e) => println!("[SERVER] Error reading from stream: {}", e),
    }
}
