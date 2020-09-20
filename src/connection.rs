use std::io::{ErrorKind, Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::fmt::{Result as FmtResult, Debug, Formatter};

pub struct Connection {
    socket: TcpStream,
}

impl Connection {
    pub fn from_tcp_stream(stream: TcpStream) -> Result<Self> {
        stream.set_nonblocking(true)?;
        Ok(Connection { socket: stream })
    }

    pub fn new(address: SocketAddr) -> Result<Self> {
        let stream_result = TcpStream::connect(address)?;
        Ok(Self::from_tcp_stream(stream_result)?)
    }

    pub fn write_str(&mut self, str: &str) -> Result<()> {
        self.write(str.as_bytes())?;
        Ok(())
    }

    pub fn write_string(&mut self, string: &String) -> Result<()> {
        self.write(string.as_bytes())?;
        Ok(())
    }

    pub fn read_available(&mut self) -> Result<Vec<u8>> {
        const BUFFER_SIZE: usize = 1;
        let mut received_bytes: Vec<u8> = Vec::new();
        let mut buffer = [0 as u8; BUFFER_SIZE];
        loop {
            let read_count_result = self.socket.read(&mut buffer);
            match read_count_result {
                Ok(read_count) => for index in 0..read_count {
                    received_bytes.push(buffer[index]);
                }
                Err(error) if error.kind() == ErrorKind::WouldBlock => break,
                Err(error) => return Err(error)
            }
        }
        return Ok(received_bytes);
    }

    pub fn wait_and_read(&mut self) -> Result<Vec<u8>> {
        self.wait()?;
        self.read_available()
    }

    pub fn wait(&mut self) -> Result<()> {
        let mut garbage_buffer = vec![0 as u8; 0];
        self.socket.set_nonblocking(false)?;
        self.socket.peek(garbage_buffer.as_mut_slice())?;
        self.socket.set_nonblocking(true)?;
        Ok(())
    }

    pub fn poll(&mut self) -> Result<bool> {
        let mut garbage_buffer = vec![0 as u8; 0];
        match self.socket.peek(garbage_buffer.as_mut_slice()) {
            Err(error) if error.kind() == ErrorKind::WouldBlock => Ok(false),
            Ok(_) => Ok(true),
            Err(error) => Err(error)
        }
    }
}

impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.socket.read(buf)
    }
}

impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.socket.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.socket.flush()
    }
}

impl Debug for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.socket.fmt(f)
    }
}