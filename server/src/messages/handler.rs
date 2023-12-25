use std::{io::{Read, self, BufReader}, net::TcpStream};

use log::{info, error, debug};
use shared::types::messages::Message;

use crate::services;

use super::{serialization::{deserialize_message, serialize_task}, fragment_maker::{create_task_for_request, process_result}};

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
/// use server::messages::handler::handle_client;
///
/// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
/// for stream in listener.incoming() {
///     if let Ok(stream) = stream {
///         handle_client(stream);
///     }
/// }
/// ```
pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    info!("Handling client");
    match stream.local_addr() {
        Ok(addr) => info!("Connection established {}", addr),
        Err(e) => error!("Failed to get local address: {}", e),
    }

    let mut reader = BufReader::new(&stream);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    debug!("Read {} bytes", buffer.len());

    let data_str = match std::str::from_utf8(&buffer) {
        Ok(str) => str,
        Err(e) => {
            error!("Invalid UTF-8 sequence: {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"));
        }
    };

    let json_start = data_str.find('{').unwrap_or(0);
    let json_str = &data_str[json_start..];
    
    debug!("Received data: {}", json_str);

    let message_result = deserialize_message(json_str);
    debug!("Deserialized data {:?}", message_result);
    
    match message_result {
        Ok(Message::FragmentRequest(request)) => {
            let task = create_task_for_request(request);
            let serialized_task: String = serialize_task(&task)?;

            info!("Sending serialized task to client at {}", stream.peer_addr()?); 
            
            services::write::write(&mut stream, &serialized_task)?;

            let response = services::reader::get_response(&mut stream)?;

            debug!("Received response: {:?}", response);
        }
        Ok(Message::FragmentTask(_task)) => {
            todo!()
        }
        Ok(Message::FragmentResult(result)) => {
            process_result(result);
        }
        Err(e) => {
            error!("Error deserializing request: {:?}", e);
        }
    }

    Ok(())
}