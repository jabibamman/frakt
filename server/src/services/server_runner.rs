use std::net::TcpListener;
use crate::handler::handle_client;

pub fn run_server(address: &str) -> std::io::Result<()> {
    println!("[SERVER] Starting server...");
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