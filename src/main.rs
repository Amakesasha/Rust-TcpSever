extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    TcpServer::set_http("HTTP/2.0");
    TcpServer::set_add_job(true);
    TcpServer::set_def_page(Response::new_from_file(
        "examples_rs/defpage.html",
        "text/html",
    ));

    let server = TcpServer::new(Server::get_server("127.0.0.1:80"), ThreadPool::new(4));

    Server::launch_range_port(server, 443..444);
}

struct Server;

impl SeverControl for Server {
    #[inline]
    fn match_methods(request: &Request, response: &mut Response) {
        match request.metod.as_str() {
            "GET" => Self::match_get(request, response),
            "POST" => Self::match_post(request, response),
            "PUT" => Self::match_put(request, response),
            _ => {}
        }
    }

    #[inline]
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener {
        let listener = TcpListener::bind(ip_addr).unwrap();
        listener.set_ttl(255).unwrap();
        listener
    }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {
        match request.url.as_str() {
            "/response" => {
                response.set_file("examples_rs/webpage.html", "text/html");
                response.cookie.add("net", "qwe");
                response.cookie.delete("qwe");
            }

            "/giphy.webp" => response.set_file("examples_rs/giphy.webp", "image/webp"),
            "/image.png" => response.set_file("examples_rs/image.png", "image/png"),
            "/video.mp4" => response.set_file("examples_rs/video.mp4", "video/mp4"),
            "/audio.mp3" => response.set_file("examples_rs/audio.mp3", "audio/mp3"),
            "/favicon.ico" => response.set_file("examples_rs/image.png", "image/png"),

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
