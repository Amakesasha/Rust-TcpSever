/// Files Server
pub mod server {
    /// Request.
    pub mod request;
    /// Response.
    pub mod response;
    /// Tcp Server.
    pub mod server_tcp;
}
/// Modified Thread Pool (From [Rust-Official-Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html))
/// Static Query Processing Thread System.
pub mod thread_pool;

pub use crate::{
    server::{request::*, response::*, server_tcp::*},
    thread_pool::*,
};
pub use std::net::TcpListener;
