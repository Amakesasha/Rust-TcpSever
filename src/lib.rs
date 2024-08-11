/// Metods for Work with HTTP
pub mod http {
    /// Request.
    pub mod request;
    /// Response.
    pub mod response;
}

/// Files Server
pub mod server {
    /// Tcp Server.
    pub mod tcp_server;
}
/// Modified Thread Pool (From [Rust-Official-Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html))
/// Static Query Processing Thread System.
pub mod thread_pool;

pub use crate::{
    http::{request::*, response::*},
    server::tcp_server::*,
    thread_pool::*,
};
pub use std::net::{TcpListener, TcpStream, ToSocketAddrs};
