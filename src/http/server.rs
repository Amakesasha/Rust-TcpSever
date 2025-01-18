use crate::*;

/// HTTP communication map default code and file status.
pub static mut DEF_PAGES: Lazy<HashMap<Vec<u8>, Bytes>> = Lazy::new(HashMap::new);

#[macro_export]
/// Set DEF_PAGES.
/// # Examples
/// ```
/// set_def_pages!(
///     ("404", Response::from_file(("404.html", "text/html"))),
///     ("403", Response::from_file(("403.html", "text/html"))),
/// );
/// ```
/// or
/// ```
/// set_def_pages!("404", Response::from_file(("404.html", "text/html")));
/// set_def_pages!("403", Response::from_file(("403.html", "text/html")));
/// ```
macro_rules! set_def_pages {
    ($(($code:expr, $page:expr)),* $(,)? ) => {{
        unsafe {
            $(
                DEF_PAGES.insert(
                    $code.as_bytes().to_vec(),
                    $page.as_bytes()
                );
            )*
        }
    }};
    ($code:expr, $page:expr) => {{
        unsafe {
            DEF_PAGES.insert(
                $code.as_bytes().to_vec(),
                $page.as_bytes()
            );
        }
    }};
}

/// Server with HTTP protocol
pub struct HttpServer;

/// Functions for starting and running the server.
impl HttpServer {
    #[inline]
    #[cfg(feature = "check_stream")]
    /// Starting the server.
    ///
    /// # Parameters
    /// * `listener` - An asynchronous TCP listener designed for listening to incoming connections.
    /// * `check_fn` - Asynchronous function to check TcpStream, returns true if valid.
    /// * `work_fn` - Asynchronous function for creating an HTTP response based on a request.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::*;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     HttpServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), check, work).await;
    /// }
    /// 
    /// async fn check(_stream: std::net::SocketAddr) -> bool { true }
    /// async fn work(_request: Request) -> Response {
    ///     Response::from_response("200 OK", "All Good :)")
    /// }
    /// ```
    pub async fn launch<FutC, FutW>(
        listener: TcpListener,
        check_fn: impl Fn(SocketAddr) -> FutC + Send + Sync + Copy + 'static,
        work_fn: impl Fn(Request) -> FutW + Send + Sync + Copy + 'static,
    ) where
        FutC: Future<Output = bool> + Send + Sync + 'static,
        FutW: Future<Output = Response> + Send + Sync + 'static,
    {
        Self::impl_launch(listener, check_fn, work_fn).await;
    }

    #[inline]
    #[cfg(not(feature = "check_stream"))]
    /// Starting the server.
    ///
    /// # Parameters
    /// * `listener` - An asynchronous TCP listener designed for listening to incoming connections.
    /// * `work_fn` - Asynchronous function for creating an HTTP response based on a request.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::*;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     HttpServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
    /// }
    /// 
    /// async fn work(_request: Request) -> Response {
    ///     Response::from_response("200 OK", "All Good :)")
    /// }
    /// ```
    pub async fn launch<FutW>(
        listener: TcpListener,
        work_fn: impl Fn(Request) -> FutW + Send + Sync + Copy + 'static,
    ) where
        FutW: Future<Output = Response> + Send + Sync + 'static,
    {
        async fn qwe(_: SocketAddr) -> bool {
            true
        }

        Self::impl_launch(listener, qwe, work_fn).await;
    }

    #[inline]
    async fn impl_launch<FutC, FutW>(
        listener: TcpListener,
        check_fn: impl Fn(SocketAddr) -> FutC + Send + Sync + Copy + 'static,
        work_fn: impl Fn(Request) -> FutW + Send + Sync + Copy + 'static,
    ) where
        FutC: Future<Output = bool> + Send + Sync + 'static,
        FutW: Future<Output = Response> + Send + Sync + 'static,
    {
        #[cfg(not(feature = "check_stream"))]
        let _ = check_fn;

        println!(
            "SERVER | HTTP | {} | LAUCNH",
            listener.local_addr().unwrap()
        );

        loop {
            let (socket, net_addr) = match listener.accept().await {
                Ok((socket, net_addr)) => (socket, net_addr),
                Err(_) => continue,
            };

            tokio::spawn(async move {
                #[cfg(not(any(feature = "get_stream", feature = "check_stream")))]
                let _ = net_addr;
                #[cfg(feature = "check_stream")]
                if !check_fn(net_addr).await {
                    return Err(ServerError::VerificationFailed);
                }

                let (mut read, mut write) = io::split(socket);

                #[cfg(not(feature = "get_stream"))]
                let request = Request::result_from(&mut read, None).await?;
                #[cfg(feature = "get_stream")]
                let request = Request::result_from(&mut read, Some(net_addr)).await?;

                let response = work_fn(request).await;

                if let Some(page) = unsafe { DEF_PAGES.get(&response.status_code) } {
                    write
                        .write_all(page)
                        .await
                        .map_err(ServerError::WriteError)?;
                } else {
                    write
                        .write_all(&response.as_bytes())
                        .await
                        .map_err(ServerError::WriteError)?;
                }

                write.flush().await.map_err(ServerError::FlushError)
            });
        }
    }
}
