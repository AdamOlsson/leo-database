use std::{net::TcpStream, io::{Read, Write}};

pub fn write(to: &mut TcpStream, buffer: &[u8]) -> Result<(), std::io::Error> {
    match to.write(buffer) {
        Ok(_) => {
            return Ok(());
        },
        Err(e) => {
            println!("Failed to write to stream.");
            return Err(e);
        },
    }
}

pub fn read_body(mut stream: &TcpStream, payload_size: usize) -> Result<Vec<u8>, std::io::Error> {
    /* Read the body from a stream. Returns a vector with the body. */
    let mut buffer = [0; 4096];
    let mut data: Vec<u8> = Vec::new();
    let mut bytes_read = 0;

    loop {
        match stream.read(&mut buffer) {
            Ok(i) => {
                data.extend_from_slice(&buffer);
                bytes_read += i;

                buffer.fill(0); // TODO remove this line
            }
            Err(e) => return Err(e),
        };

        if bytes_read >= payload_size {
            break;
        }
    }
    return Ok(data);
}