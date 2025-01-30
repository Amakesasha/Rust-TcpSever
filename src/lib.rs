//! A simple and lightweight asynchronous TCP server crate.
//!
//! # Install
//! Run the following Cargo command in your project directory:
//! ```terminal
//! cargo add maker_web
//! ```
//! Or add the following line to your Cargo.toml:
//! ```toml
//! maker_web = "0.3.0"
//! ```
//!
//! # Examples
//!
//! `HTTP`:
//! ```no_run
//! use maker_web::{HttpServer, Request, Response};
//! use tokio::net::TcpListener;
//!
//! #[tokio::main]
//! async fn main() {
//!     HttpServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
//! }
//!
//! async fn work(request: Request) -> Response {
//!     Response::from_body("All Good :)")
//! }
//! ```
//! `Without protocol`:
//! ```no_run
//! use maker_web::CleanServer;
//! use tokio::net::{TcpListener, TcpStream};
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

/*
#![feature(async_fn_in_trait)]
#![feature(type_alias_impl_trait)]
#![deny(warnings)]
#![deny(missing_docs)]
#![deny(dead_code)]
#![deny(unused)]
#![deny(unreachable_code)]
#![deny(private_in_public)]
#![deny(nonstandard_style)]
*/

/// Clean server.
pub mod clean {
    /// Server.
    pub mod server;
}
/// HTTP/1.1 server.
pub mod http_11 {
    /// Request.
    pub mod request;
    /// Response.
    pub mod response;
    /// Server.
    pub mod server;
}
/// Server error file.
pub mod errors;

use std::{
    collections::HashMap,
    convert::AsRef,
    future::Future,
    marker::{Copy, Send, Sync, Unpin},
    net::SocketAddr,
    path::Path,
    str::FromStr,
};
use {
    bytes::{Bytes, BytesMut},
    dashmap::DashMap,
    http::{
        header::{CONTENT_LENGTH, COOKIE},
        HeaderMap, HeaderName, HeaderValue, Method, StatusCode, Uri,
    },
    once_cell::sync::Lazy,
    thiserror::Error,
    tokio::{
        fs::{self, File},
        io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, ReadHalf},
        net::{TcpListener, TcpStream},
    },
};

pub use crate::clean::server::CleanServer;
pub use crate::errors::ServerError;
pub use crate::http_11::request::Request;
pub use crate::http_11::response::Response;
pub use crate::http_11::server::HttpServer;
pub use crate::http_11::server::DEF_PAGES;
