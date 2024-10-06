extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    // Set Type HTTP (For example: HTTP/1.1, HTTP/2.0).
    HttpServer::set_http("HTTP/2.0"); // No Need.

    // Set Http Server.
    HttpServer::set_server(TcpListener::bind("127.0.0.1:443").unwrap()); // Need to.

    // Set CodeMapPage (When response.status_code == {CODE}, Will Load {DATA} Attached to {CODE}).
    HttpServer::set_map_code_page(vec![(
        // {CODE}
        String::from("404 NOT FOUND"),
        // {DATA}
        Response::new_from_file("examples_rs/defpage.html", "text/html"),
    )]); // No Need

    // Launch Server in 4 Thread Mode.
    Server::launch(4); // Need to.
}

struct Server;

impl HttpSever for Server {
    // Function for Read and Parse Request.
    const FN_READ: HttpRead = HttpServer::read;
    // Function for Write Response.
    const FN_WRITE: HttpWrite = HttpServer::write;

    #[inline]
    // Check Client.
    fn check_stream(_stream: &TcpStream) -> bool {
        true
    }

    #[inline]
    // Your Parse Request and Make Response.
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) {}
}
