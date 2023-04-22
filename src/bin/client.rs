use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn write_to_stream(stream: &mut TcpStream) -> std::io::Result<()> {

    let message = "Hello world from client".as_bytes();
    stream.write_all(message)?;
    stream.flush()?;

    let mut buffer = [0; 64];
    let bytes_read = stream.read(&mut buffer)?;
    println!("Received {} bytes", bytes_read);
    let s = String::from_utf8_lossy(&buffer[..]);
    println!("Received text: {}", s);

    Ok(())
}
 
fn main() -> std::io::Result<()> {

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1234") {
        println!("Connected to the server!");
        write_to_stream(&mut stream)?;
    } else {
        println!("Couldn't connect to server...");
    }

    Ok(())
}
