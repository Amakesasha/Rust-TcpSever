use crate::*;
use std::{
    collections::HashMap,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread::spawn as thread_spawn,
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
}

/// Trait Control Server.
pub trait SeverControl {
    /// Const write version HTTP.
    /// * When Invalid HTTP, HTTP = 1.0.
    /// * When const = None, HTTP = 1.1.
    const TYPE_HTTP: Option<&'static str>;

    #[inline]
    /// Launches Read-Write Server, in Many Thread Mode.
    /// * server = TcpServer.
    fn launch(server: TcpServer) {
        server.listener.incoming().for_each(|stream| {
            server
                .thread_pool
                .execute(|| Self::handle_connection(stream.unwrap()))
        });
    }

    #[inline]
    /// Launches Read-Write Server, in One Thread Mode. Not recommended!
    /// * listener = TcpListener, the basis of the entire server.
    fn launc_one_thread(listener: TcpListener) {
        listener.incoming().for_each(|stream| {
            thread_spawn(|| Self::handle_connection(stream.unwrap()));
        });
    }

    #[inline]
    /// Read HTTP Request, make Response, and Write this Response.
    /// At start, Requst Read and Write to byte Buffer, then byte Buffer transfer to Line.
    /// Launches Parser with Line into Request, and make Response.
    /// You check request and return Request (or Not return).
    /// Response Write into Line, and Write to Client Buffer.
    /// * stream = IpAddr client for Read and Write. Only from the server!
    fn handle_connection(mut stream: TcpStream) {
        let mut buf_reader = BufReader::new(&mut stream);
        let mut buffer = [32; 1024];

        let string_request = match buf_reader.read(&mut buffer) {
            Ok(_) => String::from_utf8_lossy(&buffer[0..]),
            Err(_) => return,
        };

        if string_request.trim().is_empty() {
            return;
        }

        let request = Request::parse_to_self(&string_request.trim());
        let mut response = Response::new();

        match request.metod_url_http[0].as_str() {
            "GET" => Self::match_get(&request, &mut response),
            "POST" => Self::match_post(&request, &mut response),
            "PUT" => Self::match_put(&request, &mut response),
            _ => {}
        }

        match stream.write_all(
            response
                .format(Self::TYPE_HTTP.unwrap_or(&request.metod_url_http[2]))
                .as_bytes(),
        ) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        }
    }

    /// Your check request with metod GET (usually for send html/css file).
    /// * requset = Parsed Http Request.
    /// * response = Your Response.
    fn match_get(request: &Request, response: &mut Response);
    /// Your check request with metod POST (usually to redirect the user).
    /// * requset = Parsed Http Request.
    /// * response = Your Response.
    fn match_post(request: &Request, response: &mut Response);
    /// Your check request with metod PUT (usually to send data requested by the site).
    /// * requset = Parsed Http Request.
    /// * response = Your Response.
    fn match_put(request: &Request, response: &mut Response);
}

#[allow(dead_code)]
#[derive(Debug)]
/// Request Structure
pub struct Request {
    /// Vector Length = 3.
    /// * 0 = Metod Request (For example: GET, POST, PUT).
    /// * 1 = Url Request (For example: /sign, /find/qwe).
    /// * 2 = Type Http (For example: HTTP/1.1, HTTP/2.0).
    pub metod_url_http: Vec<String>,
    /// Cookies Files. For edit Cookies files, used Response, Not request!
    /// For find, you can used metod .find(YourName).unwrap()
    pub cookie_files: HashMap<String, String>,
    /// Add Contents. When your site requests the code, the information goes here.
    /// For find, you can used metod .find(YourName).unwrap()
    pub add_contents: HashMap<String, String>,
}

/// Functions for Parsed Http into Structure.
impl Request {
    /// Main Function Parsed. Used null, uncertain and last Line Request.
    /// * data = Http Request. \n
    /// * Null = Metod, Url, Http.
    /// * Uncertain = Parsed, If code find Cookies Line, else Empty.
    /// * Last = Parsed, If Line Not have Cookies, else Empty.
    pub fn parse_to_self(data: &str) -> Request {
        let mut cookie_files = HashMap::new();
        let mut add_contents = HashMap::new();

        let split_line: Vec<&str> = data.lines().collect();

        if let Some(cookie_line) = split_line.iter().find(|line| line.starts_with("Cookie: ")) {
            cookie_files = Self::get_data(cookie_line.trim_start_matches("Cookie: "), "; ");
        }

        let last_line = split_line.last().unwrap_or(&"");
        if !last_line.starts_with("Cookie: ") {
            add_contents = Self::get_data(last_line, "&");
        }

        Request {
            metod_url_http: split_line[0].split_whitespace().map(String::from).collect(),
            cookie_files,
            add_contents,
        }
    }

    /// Function for parse Line into HashMap.
    /// * data = Line Parsed.
    /// * char_split = Char used for Split Line.
    fn get_data(data: &str, char_split: &str) -> HashMap<String, String> {
        data.split(char_split)
            .filter_map(
                |part| match part.split('=').collect::<Vec<&str>>().as_slice() {
                    [key, value] => Some((key.to_string(), value.trim().to_string())),
                    _ => None,
                },
            )
            .collect()
    }
}