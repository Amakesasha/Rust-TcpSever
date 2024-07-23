extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    { // Not Necessary!
        let mut response = Response::const_new();
        response.set_file("src/main.rs");

        TcpServer::set_def_page(response);
    }

    Server::launch(TcpServer::new(
        TcpListener::bind("127.0.0.1:8080").unwrap(),
        // number = Minimum Number of Workers (Request Processing Threads)
        ThreadPool::new(4),
    ));
}

struct Server;

impl SeverControl for Server {
    const TYPE_HTTP: Option<&'static str> = Some("HTTP/2.0");

    #[inline]
    fn match_methods(request: &Request, response: &mut Response) {
        match request.metod_url_http[0].as_str() {
            "GET" => Self::match_get(request, response),
            "POST" => Self::match_post(request, response),
            "PUT" => Self::match_put(request, response),
            _ => {}
        }
    }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {}

    #[inline]
    fn match_post(_request: &Request, response: &mut Response) {}

    #[inline]
    fn match_put(_request: &Request, _response: &mut Response) {}
}
