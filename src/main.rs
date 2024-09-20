extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    // Set Tcp Server.
    TcpServer::set_server(TcpListener::bind("127.0.0.1:443").unwrap());
    // Launch Server in 4 Thread Mode.
    Server::launch(4);
}

struct Server;

impl SeverControl for Server {
    // Function for Read and Parse Request.
    const FN_READ: FnRead = TcpServer::read_stream;
    // Function for Write Response.
    const FN_WRITE: FnWrite = TcpServer::write_stream;

    #[inline]
    // Check Client.
    fn check_stream(_stream: &TcpStream) -> bool { true }

    #[inline]
    // Your Parse Request and Make Response.
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