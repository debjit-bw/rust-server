use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::time::Duration;
use std::{fs, thread};
use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream: TcpStream = stream.unwrap();
        pool.execute( || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];
    stream.read(&mut buf).unwrap();
    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buf[..])
    // )

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = 
        if buf.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buf.starts_with(sleep) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        &contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
