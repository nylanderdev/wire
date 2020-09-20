pub const USAGE_ERROR: i32 = 1;
pub const UNRESOLVED_HOST_ERROR: i32 = 2;
pub const CONNECTION_ERROR: i32 = 3;

pub const USAGE: &str = "wire [host] [port]";

pub fn exit_with_closed_error() -> ! {
    eprintln!("Connection was closed");
    std::process::exit(CONNECTION_ERROR)
}

pub fn exit_with_connection_error() -> ! {
    eprintln!("Could not establish connection");
    std::process::exit(CONNECTION_ERROR)
}

pub fn exit_with_address_error(address_string: &String) -> ! {
    eprintln!("Could not resolve host {}", address_string);
    std::process::exit(UNRESOLVED_HOST_ERROR)
}

pub fn exit_with_usage_error() -> ! {
    eprintln!("Usage:\t{}", USAGE);
    std::process::exit(USAGE_ERROR)
}


pub fn unwrap_or_closed_error<T, _E>(result: Result<T, _E>) -> T {
    match result {
        Ok(t) => t,
        _ => exit_with_closed_error()
    }
}

pub fn unwrap_or_connection_error<T, _E>(result: Result<T, _E>) -> T {
    match result {
        Ok(t) => t,
        _ => exit_with_connection_error()
    }
}