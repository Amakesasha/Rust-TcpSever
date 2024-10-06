extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    /// Set Clean Server.
    CleanServer::set_server(TcpListener::bind("127.0.0.1:443").unwrap());
    // Launch Server in 4 Thread Mode.
    Server::launch(4);
}

struct Server;

impl CleanSever for Server {
    #[inline]
    /// Function for Work with Stream.
    fn work(stream: &TcpStream) {
        for _ in 0..3 {
            println!("{}", CleanServer::read(stream).unwrap());

            // You Can Edit this Functions (read, write).

            CleanServer::write(stream, "qwe");
        }
    }
}