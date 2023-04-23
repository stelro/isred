use std::net::{TcpListener, TcpStream};

mod tcp_common;
use tcp_common::{K_MAX_MSG, K_HEADER_SIZE, read_full, write_all};

fn handle_request(stream: &mut TcpStream) -> std::io::Result<()> {

    // 4 Bytes for the header
    let mut rbuf = [0u8; K_HEADER_SIZE + K_MAX_MSG + 1];
    // Request the header first of the message
    read_full(stream, &mut rbuf[..K_HEADER_SIZE])?;
    // Now parse the header and grab the size of the message
    let len = u32::from_be_bytes(rbuf[..K_HEADER_SIZE].try_into().unwrap()) as usize;
    println!("The message size according to header is {} bytes.", len);

    if len > K_MAX_MSG {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, 
                "Message length is too long."));
    }

    if len == 0 {
        return Ok(());
    }

    // Now request the body of the message
    read_full(stream, &mut rbuf[K_HEADER_SIZE..K_HEADER_SIZE + len])?;
    rbuf[K_HEADER_SIZE + len] = b'\0';

    println!("Message from the client: {}", 
        std::str::from_utf8(&rbuf[K_HEADER_SIZE..K_HEADER_SIZE + len]).unwrap());

    // Send something back to client
    const REPLY_TO_CLIENT: &[u8] = b"ACK from Server!";
    let len = REPLY_TO_CLIENT.len();
    let mut wbuf = [0u8;K_HEADER_SIZE + REPLY_TO_CLIENT.len()];

    wbuf[..K_HEADER_SIZE].copy_from_slice(&(len as u32).to_be_bytes());
    wbuf[K_HEADER_SIZE..][..len].copy_from_slice(REPLY_TO_CLIENT);
    write_all(stream, &wbuf[..K_HEADER_SIZE + len])?;   

    Ok(())

}

fn main() -> std::io::Result<()> {

    println!("Starting server..");

    let listener = TcpListener::bind("0.0.0.0:1234")?;

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) => {
                loop {
                    println!("Accepting new connection...");
                    handle_request(&mut stream)?;
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
