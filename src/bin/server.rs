use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

const K_MAX_MSG: usize = 4096;

fn read_full(stream: &mut TcpStream, buffer: &mut [u8], bytes: usize) -> std::io::Result<()> {

    let mut remaining_bytes = bytes;
    
    while remaining_bytes > 0 {
        let rv = stream.read(buffer)?;
        if rv <= 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, 
                    "Failed to read enough bytes"));
        }

        assert!(rv as usize <= remaining_bytes);

        remaining_bytes -= rv as usize;
    }

    Ok(())
}

fn write_all(stream: &mut TcpStream, buffer: &[u8], bytes: usize) -> std::io::Result<()> {

    let mut remaining_bytes = bytes;

    while remaining_bytes > 0 {
        let rv = stream.write(buffer)?;
        if rv <= 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, 
                    "Failed to read enough bytes"));
        }

        assert!(rv as usize <= remaining_bytes);
        remaining_bytes -= rv as usize;
    }

    Ok(())
}

fn one_request(stream: &mut TcpStream) -> std::io::Result<()> {

    // 4 bytes header
    let mut rbuf = [0; 4 + K_MAX_MSG + 1];
    
    // Request header
    read_full(stream, &mut rbuf, 4)?;
    let len_bytes = &rbuf[0..4];

    // Assume it's little endian.
    // use from_be_bytes if it's big endian
    let len = u32::from_le_bytes([
        len_bytes[0],
        len_bytes[1],
        len_bytes[2],
        len_bytes[3]
    ]);

    println!("Received length: {}", len);
    
    if len as usize > K_MAX_MSG {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Length is too long."));
    }

    // Request header
    read_full(stream, &mut rbuf[4..], len as usize)?;


    // do something
    rbuf[4 + len as usize] = 0;

    println!("Client says: {}", String::from_utf8_lossy(&rbuf[..]));

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    println!("Accepting a new connection");
    println!("{:?}", stream.peer_addr());

    let mut buffer = [0; 64];

    let bytes_read = stream.read(&mut buffer)?;
    println!("Received {} bytes", bytes_read);
    let s = String::from_utf8_lossy(&buffer[..]);
    println!("Received text: {}", s);


    let message = "hello world from server".as_bytes();
    stream.write_all(message)?;
    stream.flush()?;

    Ok(())

}

fn main() -> std::io::Result<()> {

    println!("Starting server..");

    let listener = TcpListener::bind("0.0.0.0:1234")?;

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                handle_client(&mut stream)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
