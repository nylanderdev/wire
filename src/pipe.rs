use std::io::{self, Read, Write};
use std::marker::Send;
use std::thread::{self, JoinHandle};

pub fn pipe(
    r: (impl Read + Send + 'static),
    w: (impl Write + Send + 'static),
    opt_buf_len: Option<usize>,
) -> JoinHandle<io::Result<()>> {
    // Typical for e.g. Linux systems
    const DEFAULT_BUF_LEN: usize = 65536;
    let buf_len = opt_buf_len.unwrap_or(DEFAULT_BUF_LEN);
    let buf_len = if buf_len == 0 {
        DEFAULT_BUF_LEN
    } else {
        buf_len
    };
    let mut buf = vec![0; buf_len];
    thread::spawn(move || pipe_loop(r, w, &mut buf))
}

fn pipe_loop(
    mut r: (impl Read + Send),
    mut w: (impl Write + Send),
    buf: &mut [u8],
) -> io::Result<()> {
    loop {
        let read_len = r.read(buf)?;
        w.write_all(&buf[..read_len])?;
    }
}
