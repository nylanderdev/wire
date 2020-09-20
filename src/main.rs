use std::collections::VecDeque;
use std::io::{Read, stdin, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel, SyncSender};
use std::sync::mpsc::TryRecvError::{Disconnected, Empty};
use std::sync::mpsc::TrySendError::Full;
use std::thread;
use std::thread::JoinHandle;

use connection::Connection;
use errors::*;

mod errors;
mod connection;

const BUFFER_SIZE: usize = 2 * 47438;//24 * 1024;

type Comm = u8;

fn main() {
    let address = get_address();
    let connection = unwrap_or_connection_error(Connection::new(address));
    let (stdin_send, from_stdin) = channel();
    spawn_stdin_thread(stdin_send);
    let (to_netio, netio_recv) = channel::<Comm>();
    let (netio_send, from_netio) = sync_channel(BUFFER_SIZE);
    spawn_netio_thread(connection, netio_recv, netio_send);

    loop {
        match from_stdin.try_recv() {
            Ok(input) => {
                to_netio.send(input).unwrap();
            }
            Err(e) => match e {
                Empty => (),
                Disconnected => {
                    return;
                }
            }
        }
        match from_netio.try_recv() {
            Ok(input) => {
                print!("{}", input as char)
            }
            Err(e) => match e {
                Empty => (),
                Disconnected => {
                    return;
                }
            }
        }
    }
}

fn spawn_netio_thread(connection: Connection, thread_in: Receiver<Comm>, thread_out: SyncSender<Comm>) -> JoinHandle<()> {
    thread::spawn(move || {
        let (t_in, t_out) = (thread_in, thread_out);
        let mut queue = VecDeque::<u8>::new();
        let mut conn = connection;
        loop {
            match t_in.try_recv() {
                Ok(data) => { conn.write([data; 1].as_ref()).unwrap(); }
                Err(e) => match e {
                    Empty => (),
                    Disconnected => {
                        return;
                    }
                }
            }
            if conn.poll().unwrap() {
                let bytes = conn.read_available().unwrap();
                for byte in bytes {
                    queue.push_front(byte);
                }
            }
            while !queue.is_empty() {
                let tail = queue.pop_back().unwrap();
                match t_out.try_send(tail) {
                    Ok(_) => (),
                    Err(e) => match e {
                        Full(_) => {
                            queue.push_back(tail);
                            break;
                        }
                        _ => {
                            return;
                        }
                    }
                }
            }
        }
    })
}

fn spawn_stdin_thread(thread_out: Sender<u8>) -> JoinHandle<()> {
    thread::spawn(move || {
        let t_out = thread_out;
        loop {
            let mut buffer = [0; BUFFER_SIZE];
            match stdin().read(&mut buffer) {
                Ok(read_size) => {
                    if read_size > 0 {
                        for i in 0..read_size {
                            match t_out.send(buffer[i]) {
                                Ok(_) => {}
                                // possible disconnect, return. fixme
                                _ => return
                            }
                        }
                    } else {
                        // EOF
                        return;
                    }
                }
                _ => {}
            }
        }
    })
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
        _ => exit_with_usage_error()
    }
}

fn address_from_string_and_port(address_string: &String, port: usize) -> SocketAddr {
    let address_with_port = format!("{}:{}", address_string, port);
    let mut addresses = match address_with_port.to_socket_addrs() {
        Ok(iter) => iter,
        _ => exit_with_address_error(address_string)
    };
    loop {
        match addresses.next() {
            Some(address) => return address,
            _ => exit_with_address_error(address_string)
        }
    }
}

