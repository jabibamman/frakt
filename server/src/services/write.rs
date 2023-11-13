use std::io::Result;
use std::net::TcpStream;
use std::io::Write;

pub fn write(mut stream: TcpStream, string: &str) -> Result<TcpStream> {

    stream.write(format!("${}", string).as_bytes())?;

    Ok(stream)
}