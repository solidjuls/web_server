use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;
use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    // // Wrap the stream in a BufReader, so we can use the BufRead methods
    // let mut reader: BufReader<TcpStream> = BufReader::new(stream);
    // // Read current current data in the TcpStream
    // let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    // // Mark the bytes read as consumed so the buffer will not return them in a subsequent read
    // reader.consume(received.len());
    // println!("Result {:?}", String::from_utf8(received).unwrap());

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn listen_connections() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Couldn't connect");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("connection accepted");
                let handler: JoinHandle<()> = thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => println!("connection failed: {:?}", e),
        }
    }
}
fn main() {
    listen_connections();
    println!("Hello, world!");
}
