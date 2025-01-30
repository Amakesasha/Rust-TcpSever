<div align="center">
  <h1>MakerWeb</h1>
  <p>
    <strong>MakerWeb is a simple and lightweight asynchronous library for running and using a server.</strong>
  </p>
  <p>
  <!-- prettier-ignore-start -->
  
  [![github.com](https://img.shields.io/crates/v/maker-web?label=github.com)](https://github.com/Amakesasha/MakerWeb)
  [![license](https://img.shields.io/crates/l/maker-web.svg)](https://github.com/Amakesasha/MakerWeb/blob/main/README.md)
  [![crates.io](https://img.shields.io/crates/d/maker-web.svg)](https://crates.io/crates/maker-web)
  [![Documentation](https://docs.rs/maker-web/badge.svg)](https://docs.rs/crate/maker-webr/latest)

  <!-- prettier-ignore-end -->
  </p>
</div>

# Supported Protocols
* [Without protocol](https://github.com/Amakesasha/MakerWeb/blob/main/examples/clean.rs)
* [HTTP](https://github.com/Amakesasha/MakerWeb/blob/main/examples/http_def_start.rs)

# Usage examples: 
* See [rest Example](https://github.com/Amakesasha/MakerWeb/blob/main/examples).
## Cargo.toml:
``` Toml
[dependencies]
maker_web = "0.1.0"
```
## src/main.rs:
``` Rust
use maker_web::{HttpServer, Request, Response};
use tokio::net::TcpListener;
use http::StatusCode;

#[tokio::main]
async fn main() {
    HttpServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
}

#[inline]
async fn work(_request: Request) -> Response {
    Response::from_response(StatusCode::OK, "All Good :)")
}
```

# Future of the Library

The library will be updated as new proposals and ideas are received. I will try to post a new update every month.
 
