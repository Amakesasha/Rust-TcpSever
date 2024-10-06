#![crate_name = "rust_tcp_sever"]
#![crate_type = "lib"]

extern crate lazy_static;

/// Clean Server.
pub mod clean {
    /// Server.
    pub mod server;
}

/// HTTP Server.
pub mod http {
    /// Request.
    pub mod request;
    /// Response.
    pub mod response;
    /// Server.
    pub mod server;
}

/// Rest Files for Work Server.
pub mod rest {
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
    http::{request::*, response::*, server::*},
    rest::{thread_pool::*},
    clean::{server::*}
};
pub use std::net::{TcpListener, TcpStream};
