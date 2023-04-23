use std::net::{TcpStream};

mod tcp_common;
use tcp_common::{K_MAX_MSG, K_HEADER_SIZE, read_full, write_all};

fn query(stream: &mut TcpStream, text: &str) -> std::io::Result<()> {

    let len = text.len();
    if len > K_MAX_MSG {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, 
                "Message text is too long"));
    }

    let mut wbuf = [0u8;K_HEADER_SIZE + K_MAX_MSG];
    wbuf[..K_HEADER_SIZE].copy_from_slice(&(len as u32).to_be_bytes());
    wbuf[K_HEADER_SIZE..][..len].copy_from_slice(text.as_bytes());

    write_all(stream, &wbuf[..K_HEADER_SIZE + len])?;   

    // Read servers response
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

    println!("Message from the Server: {}", 
        std::str::from_utf8(&rbuf[K_HEADER_SIZE..K_HEADER_SIZE + len]).unwrap());

    Ok(())
}
 
fn main() -> std::io::Result<()> {
    
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1234") {
        println!("Connected to the server!");

        query(&mut stream, "ACK from client 1")?;
        query(&mut stream, "ACK from client 2")?;
        query(&mut stream, "ACK from client 3")?;
        query(&mut stream, "ACK from client 4")?;
        query(&mut stream, "Some other random bigger message")?;

        stream.shutdown(std::net::Shutdown::Both)?;

    } else {
        println!("Couldn't connect to server...");
    }
    

    Ok(())
}
