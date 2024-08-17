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
    /// * data = Binary Data (Or String Data Into Binary Data).
    pub fn write_stream(mut stream: &TcpStream, data: &[u8]) {
        stream.write_all(data).unwrap_or(());
    }
}

/// Functions for Edit Setting TcpServer
impl TcpServer {
    #[inline]
    /// Set Code Page Map. Default Value == Empty Vector.
    /// When Response.status_code == Code from the Map, the Page Associated with it Will be Loaded.
    /// * map_code_page = Code Page Map.
    pub fn set_map_code_page(map_code_page: Vec<(String, Response)>) {
        let mcp = map_code_page
            .iter()
            .map(|(code, page)| {
                (
                    code.clone(),
                    [
                        Response::format_arg("200 OK", page).as_bytes(),
                        &page.binary_data,
                    ]
                    .concat(),
                )
            })
            .collect::<Vec<(String, Vec<u8>)>>();

        unsafe {
            MAP_CODE_PAGE = mcp;
        }
    }

    #[inline]
    /// Set Http Type. Default Value == HTTP/1.1
    /// * When Invalid HTTP, HTTP = 1.0.
    /// * http = A New Http Type.
    pub fn set_http(http: &'static str) {
        unsafe {
            TYPE_HTTP = http;
        }
    }
}

/// Code Page Map
pub static mut MAP_CODE_PAGE: Vec<(String, Vec<u8>)> = Vec::new();

/// Version HTTP.
pub static mut TYPE_HTTP: &'static str = "HTTP/1.1";

/// Trait Control Server.
pub trait SeverControl {
    #[inline]
    /// Launches Some Servers, Number Servers = Range. The Port from Creation Does Not Count!
    /// * server = TcpServer.
    /// * range = Number Servers.
    fn launch_range_port(server: TcpServer, range: Range<usize>) {
        drop(server.listener);

        let thread_pool = ThreadPool::new(range.len());
        let num_thr = server.thread_pool.num_thr;
        let ip = server.socket_addr.ip();

        for port in range {
            thread_pool.add_job(move || {
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
    fn launch(server: TcpServer) {
        PrintInfoServer::server_launch(&server);

        for stream in server.listener.incoming().filter_map(Result::ok) {
            server
                .thread_pool
                .add_job(|| Self::handle_connection(stream));
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
    /// * stream = Thread Read-Write between Server and Client.
    fn handle_connection(stream: TcpStream) {
        match TcpServer::read_stream_to_request(&stream) {
            Some(request) => {
                let mut response = Response::new();

                Self::match_methods(&request, &mut response);

                match unsafe { &MAP_CODE_PAGE }
                    .iter()
                    .find(|x| x.0 == response.status_code)
                {
                    Some((_, data)) => TcpServer::write_stream(&stream, data),
                    None => {
                        TcpServer::write_stream(
                            &stream,
                            &Response::format_arg(&response.status_code, &response).as_bytes(),
                        );
                        TcpServer::write_stream(&stream, &response.binary_data);
                    }
                }
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
    /// * ip_addr = Ip for Create Server.
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
