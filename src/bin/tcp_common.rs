use std::net::{TcpStream};
use std::io::{Read, Write};

pub const K_MAX_MSG: usize = 4096;
pub const K_HEADER_SIZE: usize = 4;

pub fn read_full(stream: &mut TcpStream, buffer: &mut [u8]) -> std::io::Result<()> {

    let mut bytes_read = 0;
    
    while bytes_read < buffer.len() {
        let read = stream.read(&mut buffer[bytes_read..])?;
        if read == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, 
                    "Unexpected EOF"));
        }
        bytes_read += read;
    }

    Ok(())
}

pub fn write_all(stream: &mut TcpStream, buffer: &[u8]) -> std::io::Result<()> {

    let mut bytes_wrote = 0;
    while bytes_wrote < buffer.len() {
        let wrote = stream.write(&buffer[bytes_wrote..])?;
        bytes_wrote += wrote;
    }

    Ok(())
}


