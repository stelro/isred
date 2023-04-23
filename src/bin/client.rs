use std::net::{TcpStream};

mod tcp_common;

fn query(stream: &mut TcpStream, text: &str) -> std::io::Result<()> {

    let len = text.len();
    if len > tcp_common::K_MAX_MSG {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Text too long"));
    }

    let mut wbuf = [0u8; 4 + tcp_common::K_MAX_MSG];
    wbuf[..4].copy_from_slice(&(len as u32).to_be_bytes());
    wbuf[4..][..len].copy_from_slice(text.as_bytes());

    tcp_common::write_all(stream, &wbuf[..4 + len])?;   

    // 4 bytes header
    let mut rbuf = [0u8; 4 + tcp_common::K_MAX_MSG + 1];
    // Request header
    tcp_common::read_full(stream, &mut rbuf[..4])?;
    
    let len = u32::from_be_bytes(rbuf[..4].try_into().unwrap()) as usize;
    
    println!("Received length: {}", len);

    if len  > tcp_common::K_MAX_MSG {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Reply too long"));
    }

    // Request body
    tcp_common::read_full(stream, &mut rbuf[4..4 + len])?;

    // do something
    rbuf[4 + len] = b'\0';

    println!("Server says: {}", std::str::from_utf8(&rbuf[4..4 + len]).unwrap());
    
    Ok(())
}
 
fn main() -> std::io::Result<()> {
    
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1234") {
        println!("Connected to the server!");
        query(&mut stream, "ACK from client 1")?;
        query(&mut stream, "ACK from client 2")?;
        query(&mut stream, "ACK from client 3")?;
        drop(stream);
    } else {
        println!("Couldn't connect to server...");
    }
    

    Ok(())
}
