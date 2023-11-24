use std::{io::{Read, self, BufReader}, net::TcpStream};

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
/// use server::handler::handle_client;
///
/// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
/// for stream in listener.incoming() {
///     if let Ok(stream) = stream {
///         handle_client(stream);
///     }
/// }
/// ```
pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    match stream.local_addr() {
        Ok(addr) => println!("[SERVER] Connection established {}", addr),
        Err(e) => println!("[SERVER] Failed to get local address: {}", e),
    }

    let mut stream_reader = BufReader::new(&mut stream);
    let mut data = String::new();
    stream_reader.read_to_string(&mut data)?;

    let trimmed_data = data.trim();
    let message_result = deserialize_message(trimmed_data);

    match message_result {
        Ok(Message::FragmentRequest(request)) => {
            let task = create_task_for_request(request);
            let serialized_task: String = serialize_task(&task)?;
            services::write::write(&mut stream, &serialized_task)?;
        }
        Ok(Message::FragmentTask(_task)) => {
            todo!()
        }
        Ok(Message::FragmentResult(result)) => {
            process_result(result);
        }
        Err(e) => {
            println!("[SERVER] Error deserializing request: {:?}", e);
        }
    }

    Ok(())
}