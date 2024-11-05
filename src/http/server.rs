use crate::*;

lazy_static! {
    /// HTTP communication map default code and file status.
    pub static ref MAP_CODE_PAGE: Arc<RwLock<HashMap<String, Vec<u8>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

/// Function for reading a request and parsing it into [Request].
pub type HttpRead = for<'staitc> fn(&'staitc TcpStream) -> Option<Request>;
/// Function to write data to [TcpStream].
pub type HttpWrite = for<'a> fn(&'a TcpStream, &'a [u8]);

/// HTTP server.
pub struct HttpServer;

/// Default server functions.
impl HttpServer {
    #[inline]
    /// A function for reading a request and parsing it into [Request].
    /// * stream = Client IP address.
    /// # Examples
    /// ```
    /// let stream = TcpStream::connect("Your Ip").unwrap();
    /// HttpServer::read(&stream).unwrap();
    /// ```
    pub fn read(mut stream: &TcpStream) -> Option<Request> {
        let mut buffer = [32; 1024];

        let str_request = match BufReader::new(&mut stream).read(&mut buffer).ok()? {
            0 => return None,
            _ => String::from_utf8_lossy(&buffer),
        };

        str_request.trim().parse().ok()
    }

    #[inline]
    /// Function to write data to [TcpStream].
    /// * stream = Client IP address.
    /// * data = Output data.
    /// # Examples
    /// ```
    /// let stream = TcpStream::connect("Your Ip").unwrap();
    /// HttpServer::write(&stream, b"qweqwe");
    /// ```
    pub fn write(mut stream: &TcpStream, data: &[u8]) {
        BufWriter::new(&mut stream).write_all(data).unwrap_or_default();
    }
}

/// Functions for changing the server.
impl HttpServer {
    #[inline]
    /// Set [struct@MAP_CODE_PAGE].
    /// * map_code_page = HTTP communication map default code and file status.
    /// # Examples
    /// ```
    /// HttpServer::set_map_code_page(vec![(
    ///     String::from("404 NOT FOUND"),
    ///     Response::new_from_file("examples_rs/defpage.html", "text/html"),
    /// )]);
    /// ```
    pub fn set_map_code_page(map_code_page: Vec<(String, Response)>) {
        *MAP_CODE_PAGE.write().unwrap() = map_code_page
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
            .collect::<HashMap<String, Vec<u8>>>();
    }
}

/// Trait for server operation.
pub trait HttpControl {
    /// A function for reading a request and parsing it into [Request].
    const FN_READ: HttpRead;
    /// Function to write data to [TcpStream].
    const FN_WRITE: HttpWrite;

    #[inline]
    /// Starting the server.
    /// * listener = TcpListener.
    /// * num_thr = Number of threads.
    /// # Examples
    /// ```
    /// fn example() {
    ///     Server::http_launch(TcpListener::bind("127.0.0.1:80").unwrap(), 4);
    /// }
    ///
    /// struct Server;
    ///
    /// impl HttpSever for Server {
    ///     const FN_READ: HttpRead = HttpServer::read;
    ///     const FN_WRITE: HttpWrite = HttpServer::write;
    ///
    ///     fn check_stream(_stream: &TcpStream) -> bool { true }
    ///     fn parser_request(_stream: &TcpStream, _request: &Request, _response: &mut Response) {}
    /// }
    /// ```
    fn http_launch(listener: TcpListener, num_thr: usize) {
        ServerInfo::launch(&listener, ServerInfo::Http);

        let thread_pool = ThreadPool::new(num_thr);

        for stream in listener.incoming().filter_map(Result::ok) {
            thread_pool.add(|| Self::handle_connection(stream).unwrap_or(()));
        }

        ServerInfo::shotdown(&listener, ServerInfo::Http);
    }

    #[inline]
    /// Function for working with a client.
    /// * stream = Client IP address.
    fn handle_connection(stream: TcpStream) -> Option<()> {
        if !Self::check_stream(&stream) {
            return None;
        }

        let request = Self::FN_READ(&stream)?;
        let mut response = RESPONSE_DEF.clone();

        Self::parser_request(&stream, &request, &mut response);

        match MAP_CODE_PAGE.read().ok()?.get(&response.status_code) {
            Some(data) => Self::FN_WRITE(&stream, data),
            None => {
                Self::FN_WRITE(
                    &stream,
                    Response::format_arg(&response.status_code, &response)
                        .as_bytes(),
                );
                Self::FN_WRITE(&stream, &response.binary_data);
            }
        }

        Some(())
    }

    /// Your client ip check.
    /// * stream = Client IP address.
    /// # Examples
    /// ```
    /// fn check_stream(stream: &TcpStream) -> bool { true }
    /// ```
    fn check_stream(stream: &TcpStream) -> bool;

    /// Your work with request and response;
    /// * stream = Client IP address.
    /// * request = [Request].
    /// * response = [Response].
    fn parser_request(stream: &TcpStream, request: &Request, response: &mut Response);
}
