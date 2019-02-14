extern crate simple_http_server;
use simple_http_server::ThreadPool;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io;
use std::thread;
use std::time::Duration;
use std::fs::File;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 OK\r\n\r\n", "404.html")
    };

    let mut html = File::open(filename).unwrap();
    let mut contents = String::new();

    html.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
