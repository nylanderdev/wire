use std::io::{stdin, stdout};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

use errors::*;
use pipe::pipe;

mod errors;
mod pipe;

fn main() {
    let address = get_address();
    let socket = unwrap_or_connection_error(TcpStream::connect(address));
    let socket_read = socket.try_clone().unwrap();
    let socket_write = socket;
    let stdin = stdin();
    let stdout = stdout();
    let p1 = pipe(stdin, socket_write, None);
    let p2 = pipe(socket_read, stdout, None);
    p1.join().unwrap();
    p2.join().unwrap();
}

fn get_address() -> SocketAddr {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        let port = port_from_string(&args[2]);
        address_from_string_and_port(&args[1], port)
    } else {
        exit_with_usage_error()
    }
}

fn port_from_string(port_string: &String) -> usize {
    match port_string.parse::<usize>() {
        Ok(port) => port,
        _ => exit_with_usage_error(),
    }
}

fn address_from_string_and_port(address_string: &String, port: usize) -> SocketAddr {
    let address_with_port = format!("{}:{}", address_string, port);
    let mut addresses = match address_with_port.to_socket_addrs() {
        Ok(iter) => iter,
        _ => exit_with_address_error(address_string),
    };
    loop {
        match addresses.next() {
            Some(address) => return address,
            _ => exit_with_address_error(address_string),
        }
    }
}
