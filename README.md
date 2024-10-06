# Rust TcpSever

A simple and lightweight crate for launching and using a server. 

# Future of the Library

The library will be updated as new proposals and ideas are received. If I no longer want to develop this project, I will write about it. I will try to post a new update every month.

# Supported Protocols
| Protocol | Description |
|----------|-------------|
 [**CLEAN**](https://github.com/Amakesasha/Rust-TcpSever/tree/main/examples/clean.rs) | Without any specific protocol |
 [**HTTP**](https://github.com/Amakesasha/Rust-TcpSever/tree/main/examples/http) | Standard Hypertext Transfer Protocol |

# Usage example: 
* See [rest Example](https://github.com/Amakesasha/Rust-TcpSever/tree/main/examples) or [Write to Server](https://discord.com/invite/dYz6sYmmuu).
 ``` Rust
extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    TcpServer::set_server(
        TcpListener::bind("127.0.0.1:443").unwrap()
    );
    Server::launch(4);
}

struct Server;

impl SeverControl for Server {
    const FN_READ: FnRead = TcpServer::read_stream;
    const FN_WRITE: FnWrite = TcpServer::write_stream;

    #[inline]
    fn check_stream(_stream: &TcpStream) -> bool { true }

    #[inline]
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) {
        println!("{request:#?}");
        println!("{response:#?}");
    }
}
 ```

# Contact Information
* [Discord Server](https://discord.com/invite/dYz6sYmmuu)
* [Issues GitHub](https://github.com/Amakesasha/Rust-TcpSever/issues) 
* amakesasha@gmail.com

# Used Library:
* [lazy_static](https://crates.io/crates/lazy_static)

# License
This project is licensed under the [MIT license](https://opensource.org/license/MIT).