extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    Server::http_launch(TcpListener::bind("127.0.0.1:80").unwrap(), 4);
}

struct Server;

impl HttpControl for Server {
    const FN_READ: HttpRead = HttpServer::read;
    const FN_WRITE: HttpWrite = HttpServer::write;

    #[inline]
    fn check_stream(_stream: &TcpStream) -> bool { true }

    #[inline]
    fn parser_request(_stream: &TcpStream, _request: &Request, _response: &mut Response) { }
}