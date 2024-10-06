extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    // Set Http Server.
    HttpServer::set_server(
        TcpListener::bind("127.0.0.1:443").unwrap()
    );
    // Launch Server in 4 Thread Mode.
    Server::launch(4);
}

struct Server;

impl HttpSever for Server {
    // Function for Read and Parse Request.
    const FN_READ: HttpRead = my_read;
    // Function for Write Response.
    const FN_WRITE: HttpWrite = my_write;

    #[inline]
    // Check Client.
    fn check_stream(_stream: &TcpStream) -> bool {
        // Your Checking.
        true
    }

    #[inline]
    // Your Parse Request and Make Response.
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) {}
}

use std::io::{BufReader, BufWriter, Read, Write};

#[inline]
fn my_read(mut stream: &TcpStream) -> Option<Request> {
    let mut buffer = [32; 1024];

    let str_request = match BufReader::new(&mut stream).read(&mut buffer).ok()? {
        0 => return None,
        _ => String::from_utf8_lossy(&buffer),
    };

    Request::parse_to_self(str_request.trim())
}

#[inline]
fn my_write(mut stream: &TcpStream, data: &[u8]) {
    BufWriter::new(&mut stream).write(data).unwrap_or(0);
}
