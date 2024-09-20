# Rust TcpSever

A simple and lightweight crate for launching and using a server. 

# Reasons to choose Rust-TcpSever:
* Have [Request Processing Thread System](https://github.com/Amakesasha/Rust-TcpSever/blob/main/src/thread_pool.rs) (Thread Pool, [Rust-Official-Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html))!
* Have [Open Source](https://github.com/Amakesasha/Rust-TcpSever) and [Simple Documentation](https://docs.rs/rust_tcp_sever/latest/rust_tcp_sever/)!
* Have a [Secure License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file)!
* Have a [Discord Server](https://discord.com/invite/dYz6sYmmuu) with Direct Connection to Me!
* Have a [YouTube Channel](https://www.youtube.com/@rust-tcpsever)!
---
* [Ease of Use](https://github.com/Amakesasha/Rust-TcpSever/blob/main/examples/default_start.rs)!
* Small Library Size!
* Supports all Types of Files!

# Used Library:
* [lazy_static](https://crates.io/crates/lazy_static)

# Contact with the developer.
* [YouTube Channel](https://www.youtube.com/@rust-tcpsever)
* [Discord Server](https://discord.com/invite/dYz6sYmmuu)
* [Issues GitHub](https://github.com/Amakesasha/Rust-TcpSever/issues) 
* amakesasha@gmail.com

# Future of the library

The library will be updated as new proposals and ideas are received. If I no longer want to develop this project, I will write about it. I will try to post a new update every month

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

# License
This project is licensed under a [Proprietary License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file).

## Summary:
* By downloading the library, you automatically agree to the license.
* You can only change your copy of the project downloaded from [GitHub](https://github.com/Amakesasha/Rust-TcpSever).
* You can only download the library from [GitHub](https://github.com/Amakesasha/Rust-TcpSever) or [crates.io](https://crates.io/crates/rust_tcp_sever).
* It is prohibited to forward even an unmodified copy to other people.
* Copyright of this software remains with the author. All rights reserved.
* The author does not bear any responsibility for damage caused.