use rust_tcp_sever::*;

fn main() {
    Server::http_launch(TcpListener::bind("127.0.0.1:80").unwrap(), 4);
}

struct Server;

impl HttpControl for Server {
    #[inline]
    fn check_stream(stream: &TcpStream) -> bool { 
        stream.set_read_timeout(Some(Duration::from_micros(10))).is_ok()
    }

    #[inline]
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) { 
        println!("{request:#?}");
        println!("\n\n\n\n");
        println!("{response:#?}");
    }
}