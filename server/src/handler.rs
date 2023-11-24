use std::{net::TcpStream, io::Read};


pub fn handle_client(mut stream: TcpStream) {
    match stream.local_addr() {
        Ok(addr) => println!("[SERVER] Connection established {}", addr),
        Err(e) => println!("[SERVER] Failed to get local address: {}", e),
    }

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => println!("[SERVER] Message received: {}", String::from_utf8_lossy(&buffer[..])),
        Err(e) => println!("[SERVER] Error reading from stream: {}", e),
    }
}