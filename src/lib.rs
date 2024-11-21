//! A simple and lightweight crate for launching and using a server.

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(dead_code)]
#![deny(unused)]
#![deny(unreachable_code)]
#![deny(private_in_public)]
#![deny(nonstandard_style)]

/// Clean server.
pub mod clean {
    /// Server.
    pub mod server;
}
/// HTTP server.
pub mod http {
    /// Request.
    pub mod request;
    /// Response.
    pub mod response;
    /// Server.
    pub mod server;
}

/// The remaining files for the server to work.
pub mod rest {
    /// Enumeration for displaying information about the server's operation.
    pub mod server_info;
    /// Modified ThreadPool (From [Rust-Official-Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html))
    pub mod thread_stream;
}

pub(crate) use crate::rest::{server_info::*, thread_stream::*};
pub(crate) use once_cell::sync::Lazy;
pub(crate) use std::{
    collections::HashMap,
    convert::AsRef,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
    ops::{AddAssign, SubAssign},
    path::Path,
    str::FromStr,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub use crate::{
    clean::server::*,
    http::{request::*, response::*, server::*},
};
pub use std::{
    net::{TcpListener, TcpStream},
    time::Duration,
};

/// Trait for converting 1 type to another, in the Option wrapper.
pub trait OptionFrom<T>: Sized {
    /// Function for converting 1 type to another, in the Option shell.
    fn option_from(value: T) -> Option<Self>;
}
