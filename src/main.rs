use rust_tcp_sever::*;

fn main() {
    set_def_pages!(
        ("404 NOT FOUND", Response::from(("404.html", "text/html")))
    );

    Server::http_launch(TcpListener::bind("127.0.0.1:80").unwrap(), 4);
}

struct Server;

impl HttpControl for Server {
    #[inline]
    fn check_stream(_stream: &TcpStream) -> bool {
        true
    }

    #[inline]
    fn parser_request(_stream: &TcpStream, _request: &Request, _response: &mut Response) {}
}
