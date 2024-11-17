use crate::*;

/// HTTP communication map default code and file status.
pub static mut DEF_PAGES: Option<HashMap<String, Vec<u8>>> = None;

/// HTTP server.
pub struct HttpServer;

/// Functions for changing the server.
impl HttpServer {
    #[inline]
    /// Set DEF_PAGES.
    /// * def_pages = HTTP communication map default code and file status.
    /// # Examples
    /// ```
    /// HttpServer::set_def_pages(vec![(
    ///     String::from("404 NOT FOUND"),
    ///     Response::new_from_file("examples_rs/defpage.html", "text/html"),
    /// )]);
    /// ```
    pub fn set_def_pages(pree_def_pages: Vec<(String, Response)>) {
        let def_pages = pree_def_pages
            .iter()
            .map(|(code, page)| {
                (
                    code.clone(),
                    [
                        page.to_string().as_bytes(),
                        &page.body,
                    ]
                    .concat(),
                )
            })
            .collect::<HashMap<String, Vec<u8>>>();

        unsafe { DEF_PAGES = Some(def_pages) };
    }
}

/// Trait for server operation.
pub trait HttpControl {
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
    /// impl HttpControl for Server {
    ///     fn check_stream(_stream: &TcpStream) -> bool { true }
    ///     fn parser_request(_stream: &TcpStream, _request: &Request, _response: &mut Response) {}
    /// }
    /// ```
    fn http_launch(listener: TcpListener, num_thr: usize) {
        ServerInfo::launch(&listener, ServerInfo::Http);

        let mut thread_pool = ThreadStream::new(num_thr, Self::handle_connection);

        for stream in listener.incoming().filter_map(Result::ok) {
            thread_pool += stream;
        }

        ServerInfo::shotdown(&listener, ServerInfo::Http);
    }

    #[inline]
    /// Function for working with a client.
    /// * stream = Client IP address.
    fn handle_connection<'a>(stream: &mut TcpStream) -> Option<()> {
        if !Self::check_stream(&stream) {
            return None;
        }

        let request = Request::option_from(stream)?;
        let mut response = RESPONSE_DEF.clone();

        Self::parser_request(&stream, &request, &mut response);

        let mut buffer = BufWriter::new(stream);

        if let Some(def_pages) = unsafe { &DEF_PAGES } {
            if let Some(page) = def_pages.get(&response.status_code) {
                buffer.write_all(page).ok()?;

                return Some(());
            }
        }

        buffer.write_all(response.to_string().as_bytes()).ok()?;
        buffer.write_all(&response.body).ok()?;

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

