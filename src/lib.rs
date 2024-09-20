extern crate lazy_static;

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
/// Other Files for Work Server.
pub mod other {
    /// Modified Thread Pool (From [Rust-Official-Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html))
    /// Static Query Processing Thread System.
    pub mod thread_pool;
}

pub(crate) use lazy_static::lazy_static;
pub(crate) use std::{
    collections::HashMap,
    convert::AsRef,
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
    sync::{mpsc, Arc, Mutex, RwLock},
    thread,
};

pub use crate::{
    http::{request::*, response::*},
    other::thread_pool::*,
    server::tcp_server::*,
};
pub use std::net::{TcpListener, TcpStream};
