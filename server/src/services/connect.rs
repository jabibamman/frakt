use std::io::Result;
use std::net::TcpStream;

pub fn connect(address: &str) -> Result<TcpStream> {
    let stream = TcpStream::connect(address)?;

    Ok(stream)
}