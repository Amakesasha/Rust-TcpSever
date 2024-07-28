use crate::*;
use std::{
    io::{BufReader, Read, Write},
    net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    ops::Range,
};

/// Tcp Server Structure.
pub struct TcpServer {
    /// TcpListener.
    pub listener: TcpListener,
    /// Thread Pool for no queue.
    pub thread_pool: ThreadPool,
    /// IpAddr, Ip and Port.
    pub socket_addr: SocketAddr,
}

/// Functions for work with TcpServer.
impl TcpServer {
    #[inline]
    /// Make a New TcpServer.
    /// * listener = TcpListener, the basis of the entire server.
    /// * thread_pool = Thread Pool for no queue.
    pub fn new(listener: TcpListener, thread_pool: ThreadPool) -> Self {
        Self {
            socket_addr: listener.local_addr().unwrap(),
            thread_pool,
            listener,
        }
    }

    #[inline]
    /// Read Data Send to Stream. Parse this data into Request. End Return the Request.
    /// * stream = IpAddr client for Read and Write. Only from the server!
    pub fn read_stream_to_request(mut stream: &TcpStream) -> Option<Request> {
        let mut buffer = [32; 1024];

        let str_request = match BufReader::new(&mut stream).read(&mut buffer).unwrap_or(0) {
            0 => return None,
            _ => String::from_utf8_lossy(&buffer[0..]),
        };

        Some(Request::parse_to_self(str_request.trim()))
    }

    #[inline]
    /// Write Data in Stream.
    /// * stream = IpAddr client for Read and Write. Only from the server!
    /// * string_data = Line Data (For example: HTML, CSS, TextFile).
    /// * binary_data = Binary Data (For example: Image, Video, GIF).
    pub fn write_stream(mut stream: &TcpStream, string_data: &String, binary_data: &Vec<u8>) {
        stream.write_all(string_data.as_bytes()).unwrap_or(());
        stream.write_all(&binary_data).unwrap_or(());
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
    /// Launches Read-Write Server.
    /// * server = TcpServer.
    fn launch(mut server: TcpServer) {
        println!(
            "SERVER | LAUNCH | {} | {}",
            server.thread_pool.num_thr, server.socket_addr
        );

        for stream in server.listener.incoming().filter_map(Result::ok) {
            server
                .thread_pool
                .add_static_job(|| Self::handle_connection(stream));
        }

        println!(
            "SERVER | SHOT DOWN | {} | {}",
            server.thread_pool.num_thr, server.socket_addr
        );
    }

    #[inline]
    /// Launches Some Servers, Number Servers = Range. The Port from Creation Does Not Count!
    /// * server = TcpServer.
    /// * range = Number Servers.
    fn launch_range_port(server: TcpServer, range: Range<u16>) {
        println!(
            "SERVERS | LAUNCH | {} | {}..{}\n",
            range.len(),
            range.start,
            range.end - 1
        );

        let mut thread_pool = ThreadPool::new(range.len());
        let num_thr = server.thread_pool.num_thr;
        let ip = server.socket_addr.ip();

        drop(server.listener);

        for port in range {
            thread_pool.add_const_job(move || {
                Self::launch(TcpServer::new(
                    Self::get_server(format!("{ip}:{port}")),
                    ThreadPool::new(num_thr),
                ));
            });
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
                let mut response = Response::const_new();

                Self::match_methods(&request, &mut response);

                let (format, use_self) =
                    response.format(Self::TYPE_HTTP.unwrap_or(&request.metod_url_http[2]));

                let binary_data = if use_self {
                    &response.binary_data
                } else {
                    unsafe { &DEF_PAGE.binary_data }
                };

                TcpServer::write_stream(&stream, &format, binary_data);

                stream.shutdown(Shutdown::Both).unwrap_or(());
            }
            None => return,
        }
    }

    /// Your Check Methods Request. Usually Used GET and POST, sometimes used metod PUT.
    /// The rest are not usually used, but they can be used.
    /// * requset = Parsed Http Request.
    /// * response = Your Response.
    fn match_methods(request: &Request, response: &mut Response);
    /// Function Get Your Custom Server.
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener;
}
