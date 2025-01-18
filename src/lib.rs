//! A simple and lightweight asynchronous TCP server crate.
//!
//! # Install
//! Paste this text into your `Cargo.toml`:
//! ```toml
//! rust_tcp_sever = "0.3.0"
//! ```
//! or
//! ```
//! cargo add rust_tcp_sever
//! ```
//! 
//! # Examples
//! ```
//! #[tokio::main]
//! async fn main() {
//!     HttpServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
//! }
//! 
//! async fn work(request: Request) -> Response {
//!     Response::from_response("200 OK", "All Good :)")
//! }
//! ```
//! or 
//! ```
//! use rust_tcp_sever::*;
//! 
//! #[tokio::main]
//! async fn main() {
//!     CleanServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
//! }
//! 
//! async fn work(stream: TcpStream) {}
//! ```
//! 
//! # Supported Protocols
//! * `Without protocol`: [CleanServer]
//! * `HTTP`: [HttpServer]
//! 
//! # Feature flags
//! * `get_stream`: Adds a `socket_addr` field to the [Request].
//! * `check_stream`: Allows you to implement custom security measures by enabling address 
//! verification logic in [HttpServer::launch].

#![feature(async_fn_in_trait)]
#![feature(type_alias_impl_trait)]
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
    /// Server error file.
    pub mod errors;
}

pub(crate) use crate::rest::errors::*;
pub(crate) use std::{
    collections::HashMap,
    convert::AsRef,
    future::Future,
    marker::{Copy, Send, Sync, Unpin},
    net::SocketAddr,
    path::Path,
    str::FromStr,
};
pub(crate) use {
    bytes::{Bytes, BytesMut},
    once_cell::sync::Lazy,
    thiserror::Error,
    tokio::{
        fs::File,
        io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, ReadHalf},
    },
};

pub use crate::{
    clean::server::*,
    http::{request::*, response::*, server::*},
};
pub use tokio::net::{TcpListener, TcpStream};
