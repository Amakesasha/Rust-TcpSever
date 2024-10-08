extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    HttpServer::set_http("HTTP/2.0");
    HttpServer::set_server(TcpListener::bind("127.0.0.1:443").unwrap());

    HttpServer::set_map_code_page(vec![
        (String::from("404 NOT FOUND"), Response::new_from_file("examples_rs/defpage.html", "text/html"))
    ]);

    Server::launch(1);
}

struct Server;

impl HttpSever for Server {
    const FN_READ: HttpRead = HttpServer::read;
    const FN_WRITE: HttpWrite = HttpServer::write;

    #[inline]
    fn check_stream(_stream: &TcpStream) -> bool {
        true
    }

    #[inline]
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) {
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

            "/sign/check_pan" => println!("{:#?}", request.add_content),
            _ => {}
        }
    }
}