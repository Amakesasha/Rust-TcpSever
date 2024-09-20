extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    // Set Tcp Server.
    TcpServer::set_server(
        TcpListener::bind("127.0.0.1:443").unwrap()
    );
    // Launch Server in 4 Thread Mode.
    Server::launch(4);
}

struct Server;

impl SeverControl for Server {
    // Function for Read and Parse Request.
    const FN_READ: FnRead = my_read_stream;
    // Function for Write Response.
    const FN_WRITE: FnWrite = my_write_stream;

    #[inline]
    // Check Client.
    fn check_stream(_stream: &TcpStream) -> bool {
        // Your Checking.
        true 
    }

    #[inline]
    // Your Parse Request and Make Response.
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) { }
}

use std::io::{BufReader, Read, BufWriter, Write};

#[inline]
fn my_read_stream(mut stream: &TcpStream) -> Option<Request> {
    let mut buffer = [32; 1024];

    let str_request = match BufReader::new(&mut stream).read(&mut buffer).ok()? {
        0 => return None,
        _ => String::from_utf8_lossy(&buffer),
    };

    Request::parse_to_self(str_request.trim())
}

#[inline]
fn my_write_stream(mut stream: &TcpStream, data: &[u8]) {
    BufWriter::new(&mut stream).write(data).unwrap_or(0);
}