<div align="center">
  <h1>Rust TcpSever</h1>
  <p>
    <strong>Rust TcpSever is a simple and lightweight crate for launching and using a server.</strong>
  </p>
  <p>
  <!-- prettier-ignore-start -->
  
  [![gitverse.ru](https://img.shields.io/crates/v/rust-tcp-sever?label=gitverse.ru)](https://gitverse.ru/Amakesasha/Rust-TcpSever)
  [![license](https://img.shields.io/crates/l/rust-tcp-sever.svg)](https://gitverse.ru/Amakesasha/Rust-TcpSever/content/main/LICENSE)
  [![crates.io](https://img.shields.io/crates/d/rust-tcp-sever.svg)](https://crates.io/crates/rust_tcp_sever)
  [![Documentation](https://docs.rs/rust_tcp_sever/badge.svg)](https://docs.rs/crate/rust_tcp_sever/latest)

  <!-- prettier-ignore-end -->
  </p>
</div>

# Supported Protocols
* [Without protocol](https://gitverse.ru/Amakesasha/Rust-TcpSever/content/main/examples/clean.rs)
* [HTTP](https://gitverse.ru/Amakesasha/Rust-TcpSever/content/main/examples/http_def_start.rs)

# Usage examples: 
* See [rest Example](https://gitverse.ru/Amakesasha/Rust-TcpSever/content/main/examples).
## Cargo.toml:
``` Toml
[dependencies]
rust_tcp_sever = "0.2.3"
```
## src/main.rs:
``` Rust
use rust_tcp_sever::*;

fn main() {
    Server::http_launch(TcpListener::bind("127.0.0.1:80").unwrap(), 4);
}

struct Server;

impl HttpControl for Server {
    #[inline]
    fn check_stream(stream: &TcpStream) -> bool { true }

    #[inline]
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) { 
        println!("{request:#?}");
        println!("\n\n\n\n");
        println!("{response:#?}");
    }
}
 ```

# Future of the Library

The library will be updated as new proposals and ideas are received. I will try to post a new update every month.
 