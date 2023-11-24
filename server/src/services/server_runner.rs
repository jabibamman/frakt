use std::net::TcpListener;
use crate::handler::handle_client;

/// Starts a TCP server on the specified address.
///
/// This function initializes a TCP server, binds it to the provided address, and starts listening for incoming connections.
/// Each incoming connection is handled by the `handle_client` function. In case of an error while binding to the address,
/// it logs the error and returns without starting the server.
///
/// ## Parameters
/// - `address`: A string slice (`&str`) representing the address on which the server will listen. 
///   The address should be in the format "{hostname}:{port}".
///
/// ## Returns
/// A `std::io::Result<()>`. If successful, it returns `Ok(())`. In case of an error during binding, it returns `Ok(())` after logging the error.
///
/// ## Errors
/// This function will return an `Err` variant of `std::io::Result` if there is an error in the TCP listener's incoming connection stream.
///
/// ## Example Usage
/// ```no_run
/// use server::services::server_runner::run_server;
/// 
/// let address = "127.0.0.1:8080";
/// match run_server(address) {
///     Ok(()) => println!("Server running on {}", address),
///     Err(e) => println!("Failed to start server: {}", e),
/// }
/// ```
pub fn run_server(address: &str) -> std::io::Result<()> {
    println!("[SERVER] Starting server on {}", address);
    let listener = match TcpListener::bind(address) {
        Ok(listener) => listener,
        Err(e) => {
            println!("[SERVER] Failed to bind to address: {}", e);
            return Ok(());
        }
    };

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

#[cfg(test)]
mod server_runner_tests {
    use super::*;
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_run_server() {
        let address = "127.0.0.1:8787";
        thread::spawn(move || {
            run_server(address).unwrap();
        });

        thread::sleep(Duration::from_millis(100));

        match TcpStream::connect(address) {
            Ok(_) => println!("Successfully connected to server"),
            Err(e) => panic!("Failed to connect to server: {}", e),
        }
    }
}