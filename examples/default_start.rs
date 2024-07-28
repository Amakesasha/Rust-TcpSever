extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    TcpServer::set_def_page(Response::new_from_file(
        "examples/defpage.html",
        "text/html",
    ));

    let server = TcpServer::new(Server::get_server("127.0.0.1:8077"), ThreadPool::new(4));

    Server::launch_range_port(server, 8075..8080);
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

    #[inline]
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener {
        TcpListener::bind(ip_addr).unwrap()
    }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {
        match request.metod_url_http[1].as_str() {
            "/response" => response.set_file("examples/webpage.html", "text/html"),

            "/image.png" => response.set_file("examples/image.png", "image/png"),
            "/video.mp4" => response.set_file("examples/video.mp4", "video/mp4"),
            "/audio.mp3" => response.set_file("examples/audio.mp3", "audio/mp3"),

            "/wer" => response.set_redirect("/response"),

            "/sleep" => std::thread::sleep(std::time::Duration::from_secs(30)),
            _ => {}
        }
    }

    #[inline]
    fn match_post(_request: &Request, _response: &mut Response) {}
    #[inline]
    fn match_put(_request: &Request, _response: &mut Response) {}
}
