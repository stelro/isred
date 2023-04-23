use std::net::{TcpListener, TcpStream};

mod tcp_common;

fn one_request(stream: &mut TcpStream) -> std::io::Result<()> {

    // 4 bytes header
    let mut rbuf = [0u8; 4 + tcp_common::K_MAX_MSG + 1];
    // Request header
    tcp_common::read_full(stream, &mut rbuf[..4])?;
    
    let len = u32::from_be_bytes(rbuf[..4].try_into().unwrap()) as usize;
    
    println!("Received length: {}", len);

    if len > tcp_common::K_MAX_MSG {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Length is too long."));
    }

    if len == 0 {
        return Ok(());
    }

    // Request body
    tcp_common::read_full(stream, &mut rbuf[4..4 + len])?;

    // do something
    rbuf[4 + len] = b'\0';

    println!("Client says: {}", std::str::from_utf8(&rbuf[4..4 + len]).unwrap());

    // Reply using the same protocol
    const REPLY: &[u8] = b"ACK from server!";
    let len = REPLY.len();
    let mut wbuf = [0u8; 4 + REPLY.len()];
    
    wbuf[..4].copy_from_slice(&(len as u32).to_be_bytes());
    wbuf[4..][..len].copy_from_slice(REPLY);
    
    tcp_common::write_all(stream, &wbuf[..4 + len])?;

    Ok(())
}

fn main() -> std::io::Result<()> {

    println!("Starting server..");

    let listener = TcpListener::bind("0.0.0.0:1234")?;

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                one_request(&mut stream)?;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
