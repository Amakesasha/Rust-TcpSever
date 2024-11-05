use rust_tcp_sever::*;
use std::io::{Read, Write};

fn main() {
    // Running the server in 4 threads.
    Server::http_launch(TcpListener::bind("127.0.0.1:2").unwrap(), 4);
}

struct Server;

impl HttpControl for Server {
    // Function for reading a request and parsing it into Request.
    const FN_READ: HttpRead = my_read;
    // Function to write data to TcpStream.
    const FN_WRITE: HttpWrite = my_write;

    #[inline]
    // Your client ip check.
    fn check_stream(_stream: &TcpStream) -> bool { true }

    #[inline]
    // Your work with request and response.
    fn parser_request(_stream: &TcpStream, _request: &Request, _response: &mut Response) {}
}


#[inline]
fn my_read(mut stream: &TcpStream) -> Option<Request> {
    let mut buffer = [32; 1024];

    let str_request = match stream.read(&mut buffer).ok()? {
        0 => return None,
        _ => String::from_utf8_lossy(&buffer),
    };

    str_request.trim().parse().ok()
}

#[inline]
fn my_write(mut stream: &TcpStream, data: &[u8]) {
    stream.write_all(data).unwrap_or(());
}
