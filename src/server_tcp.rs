use crate::*;
use std::{
    collections::HashMap,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

pub trait ServerControl {
    const TYPE_HTTP: Option<&'static str>;

    #[inline]
    fn start_def_server<A: ToSocketAddrs>(addr: A, num_thread_pool: usize) {
        Self::iter_incomung(
            TcpListener::bind(addr).unwrap(),
            ThreadPool::new(num_thread_pool),
        );
    }

    #[inline]
    fn start_my_server(listener: TcpListener, num_thread_pool: usize) {
        Self::iter_incomung(listener, ThreadPool::new(num_thread_pool));
    }

    #[inline]
    fn iter_incomung(listener: TcpListener, pool: ThreadPool) {
        println!(
            "    Start Server on SocketAddr: {}",
            listener.local_addr().unwrap()
        );
        for stream in listener.incoming() {
            pool.execute(|| Self::handle_connection(stream.unwrap()));
        }
    }

    #[inline]
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

    fn match_get(request: &Request, response: &mut Response);
    fn match_post(request: &Request, response: &mut Response);
    fn match_put(request: &Request, response: &mut Response);
}

//

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request {
    pub metod_url_http: Vec<String>,
    pub cookie_files: HashMap<String, String>,
    pub add_contents: HashMap<String, String>,
}

impl Request {
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

//

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Response {
    pub status_line: String,
    pub data: String,

    pub cookie: Cookie,
    pub setting_content: String,
}

use std::fmt::Display as WritT;

impl Response {
    #[inline]
    pub fn new() -> Response {
        Response {
            status_line: String::from("404 NOT FOUND\r\n"),
            data: String::new(),

            cookie: Cookie::new(),
            setting_content: String::new(),
        }
    }

    #[inline]
    pub fn format<Q: WritT>(&self, http: Q) -> String {
        format!(
            "{} {}\r\n{}{}{}",
            http, self.status_line, self.cookie.0, self.setting_content, self.data,
        )
    }

    #[inline]
    pub fn set_response<Q: WritT, W: WritT>(&mut self, status: Q, data: W) {
        self.status_line = status.to_string();
        self.data = format!("\r\n\r\n{}", data);
    }

    pub fn response_add_content<Q: WritT, W: WritT>(&mut self, sc: Q, value: W) {
        self.setting_content
            .push_str(&format!("{}: {}\r\n", sc, value));
    }

    #[inline]
    pub fn set_redirect<Q: WritT>(&mut self, location: Q) {
        self.status_line = format!("302 FOUND");
        self.data = format!("Location: {}", location);
    }

    #[inline]
    pub fn set_status_line<Q: WritT>(&mut self, error_code: Q) {
        self.status_line = error_code.to_string();
    }

    //
}

#[derive(Debug, Default)]
pub struct Cookie(pub String);

impl Cookie {
    #[inline]
    pub const fn new() -> Self {
        Cookie { 0: String::new() }
    }

    #[inline]
    pub fn add<Q: WritT, W: WritT>(&mut self, name: Q, value: W) {
        self.0
            .push_str(&format!("Set-Cookie: {}={}\r\n", name, value));
    }

    #[inline]
    pub fn delete<Q: WritT>(&mut self, name: Q) {
        self.0.push_str(&format!(
            "Set-Cookie: {}=; Expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n",
            name
        ));
    }
}
