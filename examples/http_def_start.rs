use rust_tcp_sever::*;

fn main() {
    // Sets HTTP communication map default code and file status.
    HttpServer::set_map_code_page(vec![(
        String::from("404 NOT FOUND"),
        Response::new_from_file("examples_rs/defpage.html", "text/html"),
    )]);

    // Running the server in 4 threads.
    Server::http_launch(TcpListener::bind("127.0.0.1:1").unwrap(), 4);
}

struct Server;

impl HttpControl for Server {
    // Function for reading a request and parsing it into Request.
    const FN_READ: HttpRead = HttpServer::read;
    // Function to write data to TcpStream.
    const FN_WRITE: HttpWrite = HttpServer::write;

    #[inline]
    // Your client ip check.
    fn check_stream(_stream: &TcpStream) -> bool {
        true
    }

    #[inline]
    // Your work with request and response;
    fn parser_request(_stream: &TcpStream, _request: &Request, _response: &mut Response) {}
}
