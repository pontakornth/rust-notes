use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use http_server_final_project::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // Single-thread
        // handle_connnection(stream);
        //
        // Multi-thread (Unlimited)
        // Vulnerable to DDos Devil.
        // It creates a thread for a request. Imagine 1000000 requests.
        // The server will become hotter than the sun.
        // thread::spawn(|| handle_connnection(stream));

        // Thread pools - Finite number of thread.
        pool.execute(|| {
            handle_connnection(stream);
        })
    }
}

fn handle_connnection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Array of size 1024 filled with 0.

    // Actually, reading needs the origin to be mutable.
    stream.read(&mut buffer).unwrap();

    // GET /
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // lossy = invalid characters are replaced with replace characters.
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Let's response something.
    // let response  = "HTTP/1.1 200 OK \r\n\r\n";
    // stream.write(response.as_bytes()).unwrap();
    // stream.flush().unwrap();
    //
    //
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // simulate slow request
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
