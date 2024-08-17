extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    // Set Type Http, not necessary.
    TcpServer::set_http("HTTP/2.0");
    // Set Page Code Map, preferably)
    TcpServer::set_map_code_page(vec![(
        String::from("404 NOT FOUND"),
        Response::new_from_file("examples_rs/defpage.html", "text/html"),
    )]);

    // Creating a Server.
    let server = TcpServer::new(Server::get_server("127.0.0.1:80"), ThreadPool::new(4));
    // Running a server on multiple ports (in this case 443).
    Server::launch_range_port(server, 443..444);
}

struct Server;

impl SeverControl for Server {
    #[inline]
    // Your Parsed Request.
    fn match_methods(request: &Request, response: &mut Response) {}

    #[inline]
    // Create Server, you can leave it like that.
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener {
        TcpListener::bind(ip_addr).unwrap()
    }
}
