use crate::*;
use std::{
    io::{BufReader, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
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

/// Functions for Build TcpServer.
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
}

/// Functions for Work TcpServer.
impl TcpServer {
    #[inline]
    /// Read Data Send to Stream. Parse this data into Request. End Return the Request.
    /// * stream = IpAddr client for Read and Write. Only from the server!
    pub fn read_stream_to_request(mut stream: &TcpStream) -> Option<Request> {
        let mut buffer = [32; 1024];

        let str_request = match BufReader::new(&mut stream).read(&mut buffer).ok()? {
            0 => return None,
            _ => String::from_utf8_lossy(&buffer),
        };

        Request::parse_to_self(str_request.trim())
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
}

/// Functions for Edit Setting TcpServer
impl TcpServer {
    #[inline]
    /// Set Default Page on 404 Error Server.
    /// At status code = 404 NOT FOUND, this page will load.
    /// * def_page = Default Load Page.
    pub fn set_def_page(def_page: Response) {
        unsafe {
            DEF_PAGE = def_page;
        }
    }

    #[inline]
    /// Set Http Type. Default Value == HTTP/1.1
    /// * When Invalid HTTP, HTTP = 1.0.
    /// * When const = None, HTTP = HTTP type Request (usually 1.1).
    /// * http = A New Http Type.
    pub fn set_http(http: &'static str) {
        unsafe {
            TYPE_HTTP_SERVER = Some(http);
        }
    }

    #[inline]
    /// Set Type Add Job on ThreadPool. Default Value == true.
    /// * If true, add == add_static_job.
    /// * If false, add == add_const_job.
    /// * add_job= Type Add Job.
    pub fn set_add_job(add_job: bool) {
        unsafe {
            TYPE_THREAD_POOL = add_job;
        }
    }
}

/// Default Page Server.
/// At status code = 404 NOT FOUND, this page will load.
pub static mut DEF_PAGE: Response = Response::const_new();

/// Version HTTP.
pub static mut TYPE_HTTP_SERVER: Option<&'static str> = None;

/// Type Add Job on ThreadPool.
pub static mut TYPE_THREAD_POOL: bool = true;

//

/// Trait Control Server.
pub trait SeverControl {
    #[inline]
    /// Launches Some Servers, Number Servers = Range. The Port from Creation Does Not Count!
    /// * server = TcpServer.
    /// * range = Number Servers.
    fn launch_range_port(server: TcpServer, range: Range<usize>) {
        drop(server.listener);

        let mut thread_pool = ThreadPool::new(range.len());
        let num_thr = server.thread_pool.num_thr;
        let ip = server.socket_addr.ip();

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
    /// Launches Read-Write Server.
    /// * server = TcpServer.
    fn launch(mut server: TcpServer) {
        PrintInfoServer::server_launch(&server);

        let fn_add = match unsafe { TYPE_THREAD_POOL } {
            true => ThreadPool::add_static_job,
            false => ThreadPool::add_const_job,
        };

        for stream in server.listener.incoming().filter_map(Result::ok) {
            fn_add(&mut server.thread_pool, || Self::handle_connection(stream));
        }

        PrintInfoServer::server_shotdown(&server);
    }

    #[inline]
    /// Handle Connection type One Write, One Read, break Connection.
    /// Read HTTP Request, make Response, and Write this Response.
    /// At start, Requst Read and Write to byte Buffer, then byte Buffer transfer to Line.
    /// Launches Parser with Line into Request, and make Response.
    /// You check request and return Request (or Not return).
    /// Response Write into Line, and Write to Client Buffer.
    /// * _q = No Need.
    fn handle_connection(stream: TcpStream) {
        match TcpServer::read_stream_to_request(&stream) {
            Some(request) => {
                //println!("{}\n\n\n", Request::format(&request));
                let mut response = Response::const_new();

                Self::match_methods(&request, &mut response);

                let fus = response.format(unsafe { TYPE_HTTP_SERVER }.unwrap_or(&request.http));

                let binary_data = match fus.1 {
                    true => &response.binary_data,
                    false => unsafe { &DEF_PAGE.binary_data },
                };

                TcpServer::write_stream(&stream, &fus.0, binary_data);
            }
            None => return,
        }
    }

    /// Your Check Methods Request. Usually Used GET and POST, sometimes used metod PUT.
    /// The rest are not usually used, but they can be used.
    /// * request = Parsed Http Request.
    /// * response = Your Response.
    fn match_methods(request: &Request, response: &mut Response);
    /// Function Get Your Custom Server.
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener;
}

/// Struct For Print Information about Working Server.
pub struct PrintInfoServer;

impl PrintInfoServer {
    #[inline]
    /// Print about Launch Server.
    pub fn server_launch(server: &TcpServer) {
        println!(
            "SERVER | LAUNCH | {} | {} ",
            server.thread_pool.num_thr, server.socket_addr
        );
    }

    #[inline]
    /// Print about Shot Down Server.
    pub fn server_shotdown(server: &TcpServer) {
        println!(
            "SERVER | SHOT DOWN | {} | {}",
            server.thread_pool.num_thr, server.socket_addr
        );
    }
}
