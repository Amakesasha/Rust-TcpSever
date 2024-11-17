use rust_tcp_sever::*;

fn main() {
    // Running the server in 4 threads.
    Server::clean_launch(TcpListener::bind("127.0.0.1:4").unwrap(), 4);
}

struct Server;

impl CleanControl for Server {
    #[inline]
    // Function for working with Stream.
    fn work(stream: &mut TcpStream) -> Option<()> {
        for _ in 0..3 {
            println!("{}", CleanServer::read(stream).unwrap());

            CleanServer::write(stream, "qwe");
        }

        Some(())
    }
}
