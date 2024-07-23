use crate::*;
use std::{
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

/// Tcp Server Structure.
pub struct TcpServer {
    /// TcpListener.
    pub listener: TcpListener,
    /// Thread Pool for no queue.
    pub thread_pool: ThreadPool,
}

/// Functions for work with TcpServer.
impl TcpServer {
    #[inline]
    /// Make a new TcpServer.
    /// * listener = TcpListener, the basis of the entire server.
    /// * thread_pool = Thread Pool for no queue.
    pub fn new(listener: TcpListener, thread_pool: ThreadPool) -> Self {
        Self {
            listener,
            thread_pool,
        }
    }

    #[inline]
    /// Read Data Send to Stream. Parse this data into Request. End Return the Request.
    /// * stream = IpAddr client for Read and Write. Only from the server!
    pub fn read_stream_to_request(mut stream: &TcpStream) -> Option<Request> {
        let mut buffer = [32; 1024];

        let str_request = match BufReader::new(&mut stream).read(&mut buffer) {
            Ok(len) => match len == 0 {
                true => return None,
                false => String::from_utf8_lossy(&buffer[0..]),
            },
            Err(_) => return None,
        };

        Some(Request::parse_to_self(str_request.trim()))
    }

    #[inline]
    /// Write Data in Stream.
    /// * data = Data Writed Stream.
    /// * stream = IpAddr client for Read and Write. Only from the server!
    pub fn write_stream(mut stream: &TcpStream, data: &String) -> () {
        stream.write_all(data.as_bytes()).unwrap_or(())
    }

    #[inline]
    /// Set Default Page on 404 Error Server.
    /// At status code = 404 NOT FOUND, this page will load.
    /// * def_page = Default Load Page.
    pub fn set_def_page(def_page: Response) {
        unsafe {
            DEF_PAGE = def_page;
        }
    }
}

/// Default Page Server.
/// At status code = 404 NOT FOUND, this page will load.
pub static mut DEF_PAGE: Response = Response::const_new();

/// Trait Control Server.
pub trait SeverControl {
    /// Const write version HTTP.
    /// * When Invalid HTTP, HTTP = 1.0.
    /// * When const = None, HTTP = HTTP type Request (usually 1.1).
    const TYPE_HTTP: Option<&'static str>;

    #[inline]
    /// Launches Read-Write Server, in Many Thread Mode.
    /// * server = TcpServer.
    fn launch(mut server: TcpServer) {
        for stream in server.listener.incoming().filter_map(Result::ok) {
            server
                .thread_pool
                .execute(|| Self::handle_connection(stream))
        }
    }

    #[inline]
    /// Read HTTP Request, make Response, and Write this Response.
    /// At start, Requst Read and Write to byte Buffer, then byte Buffer transfer to Line.
    /// Launches Parser with Line into Request, and make Response.
    /// You check request and return Request (or Not return).
    /// Response Write into Line, and Write to Client Buffer.
    /// * stream = IpAddr client for Read and Write. Only from the server!
    fn handle_connection(stream: TcpStream) {
        match TcpServer::read_stream_to_request(&stream) {
            Some(request) => {
                let mut response = Response::new();

                Self::match_methods(&request, &mut response);

                TcpServer::write_stream(
                    &stream,
                    &response.format(Self::TYPE_HTTP.unwrap_or(&request.metod_url_http[2])),
                )
            }
            None => return,
        }
    }

    /// Your Check Methods Request. Usually Used GET and POST, sometimes used metod PUT.
    /// The rest are not usually used, but they can be used.
    /// * requset = Parsed Http Request.
    /// * response = Your Response.
    fn match_methods(request: &Request, response: &mut Response);
}
