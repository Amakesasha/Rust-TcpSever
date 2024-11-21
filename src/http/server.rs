use crate::*;

/// HTTP communication map default code and file status.
pub static mut DEF_PAGES: Lazy<HashMap<String, Vec<u8>>> = Lazy::new(|| HashMap::new());

#[macro_export]
/// Set DEF_PAGES.
/// # Examples
/// ```
/// set_def_pages!(
///     ("404", Response::new_from_file("404.html", "text/html")),
///     ("403", Response::new_from_file("403.html", "text/html")),
/// );
/// ```
macro_rules! set_def_pages {
    ($(($code:expr, $page:expr)),* $(,)?) => {{
        use std::collections::HashMap;
        use once_cell::sync::Lazy;

        let def_pages: HashMap<String, Vec<u8>> = {
            let mut map = HashMap::new();
            $(
                map.insert(
                    format!("{}", $code),
                    [
                        $page.to_string().as_bytes(),
                        &$page.body,
                    ]
                    .concat(),
                );
            )*
            map
        };

        unsafe { *DEF_PAGES = def_pages };
    }};
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
        let mut thread_pool = ThreadStream::new(num_thr, Self::handle_connection);

        TypeServer::launch(&listener, TypeServer::Http(num_thr));

        listener
            .incoming()
            .filter_map(Result::ok)
            .for_each(|stream| thread_pool += stream);

        TypeServer::shotdown(&listener, TypeServer::Http(num_thr));
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

        if let Some(page) = unsafe { DEF_PAGES.get(&response.status_code) } {
            buffer.write_all(page).ok()?;
        } else {
            buffer.write_all(response.to_string().as_bytes()).ok()?;
            buffer.write_all(&response.body).ok()?;
        }

        return buffer.flush().ok();
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
