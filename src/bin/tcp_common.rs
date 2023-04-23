use std::net::{TcpStream};
use std::io::{Read, Write};

pub const K_MAX_MSG: usize = 4096;

pub fn read_full(stream: &mut TcpStream, buffer: &mut [u8]) -> std::io::Result<()> {

    let mut total_read = 0;
    
    while total_read < buffer.len() {
        let read = stream.read(&mut buffer[total_read..])?;
        println!("read bytes: {}", read);
        if read == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, 
                    "Unexpected EOF"));
        }
        total_read += read;
    }

    Ok(())
}

pub fn write_all(stream: &mut TcpStream, buffer: &[u8]) -> std::io::Result<()> {

    let mut bytes_written = 0;

    while bytes_written < buffer.len() {
        let written = stream.write(&buffer[bytes_written..buffer.len()])?;
        bytes_written += written;
    }

    Ok(())
}


