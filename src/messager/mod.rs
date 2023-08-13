use ::std::net::TcpStream;
use std::io::Result;
use std::io::{Read, Write};

pub struct Messager {
    stream: TcpStream,
    buffer: [u8; 1024],
}

impl Messager {
    pub fn new(stream: TcpStream, buffer: [u8; 1024]) -> Self {
        Self { stream, buffer }
    }

    fn read_bytes(&mut self) -> Result<usize> {
        self.stream.read(&mut self.buffer)
    }

    pub fn read(&mut self) -> Result<String> {
        match self.read_bytes() {
            Ok(size) => Ok(String::from_utf8_lossy(&self.buffer[..size])
                .trim()
                .to_string()),
            Err(error) => Err(error),
        }
    }

    pub fn send(&mut self, text: String) -> Result<()> {
        let line = [text, String::from("\n")].join("");
        let aux_buffer = line.as_bytes();
        self.stream.write_all(&aux_buffer[..aux_buffer.len()])
    }
}
