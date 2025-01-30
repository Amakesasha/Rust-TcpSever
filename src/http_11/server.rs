use crate::*;

/// HTTP communication map default code and file status.
pub static DEF_PAGES: Lazy<DashMap<StatusCode, Bytes>> = Lazy::new(DashMap::new);

#[macro_export]
/// Macro for setting DEF_PAGES.
/// # Examples
/// ```
/// use maker_web::{set_def_pages, DEF_PAGES, Response};
/// use http::StatusCode;
///
/// set_def_pages!(
///     (StatusCode::NOT_FOUND, Response::from_body("Status: 404")),
///     (StatusCode::FORBIDDEN, Response::from_body("Status: 403"))
/// );
/// ```
/// or
/// ```
/// use maker_web::{set_def_pages, DEF_PAGES, Response};
/// use http::StatusCode;
///
/// set_def_pages!(StatusCode::NOT_FOUND, Response::from_body("Status: 404"));
/// set_def_pages!(StatusCode::FORBIDDEN, Response::from_body("Status: 403"));
/// ```
macro_rules! set_def_pages {
    ($(($code:expr, $page:expr)),* $(,)? ) => {{
        $(
            match $page.as_bytes() {
                Ok(bytes) => drop(DEF_PAGES.insert($code, bytes)),
                Err(err) => eprintln!("DEF_PAGES | ERROR | {err}"),
            }
        )*
    }};
    ($code:expr, $page:expr) => {{
        match $page.as_bytes() {
            Ok(bytes) => drop(DEF_PAGES.insert($code, bytes)),
            Err(err) => eprintln!("DEF_PAGES | ERROR | {err}"),
        }
    }};
}

/// Server with HTTP protocol
pub struct HttpServer;

/// Functions for starting and running the server.
impl HttpServer {
    #[inline]
    /// Starting the server.
    ///
    /// # Parameters
    /// * `listener` - An asynchronous TCP listener designed for listening to incoming connections.
    /// * `work_fn` - Asynchronous function for creating an HTTP response based on a request.
    /// * `check_fn` - Asynchronous function to check TcpStream, returns true if valid.
    ///
    /// # Examples
    /// ```no_run
    /// use maker_web::{HttpServer, Request, Response};
    /// use tokio::net::TcpListener;
    /// use http::StatusCode;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     HttpServer::launch_with_check(
    ///         TcpListener::bind("127.0.0.1:80").await.unwrap(),
    ///         work,
    ///         check,
    ///     ).await;
    /// }
    ///
    /// async fn check(_: std::net::SocketAddr) -> bool { true }
    /// async fn work(_request: Request) -> Response {
    ///     Response::from_body("All Good :)")
    /// }
    /// ```
    pub async fn launch_with_check<FutC, FutW>(
        listener: TcpListener,
        work_fn: impl Fn(Request) -> FutW + Send + Sync + Copy + 'static,
        check_fn: impl Fn(SocketAddr) -> FutC + Send + Sync + Copy + 'static,
    ) where
        FutC: Future<Output = bool> + Send + Sync + 'static,
        FutW: Future<Output = Response> + Send + Sync + 'static,
    {
        Self::impl_launch(listener, check_fn, work_fn).await;
    }

    #[inline]
    /// Starting the server.
    ///
    /// # Parameters
    /// * `listener` - An asynchronous TCP listener designed for listening to incoming connections.
    /// * `work_fn` - Asynchronous function for creating an HTTP response based on a request.
    ///
    /// # Examples
    /// ```no_run
    /// use maker_web::{HttpServer, Request, Response};
    /// use tokio::net::TcpListener;
    /// use http::StatusCode;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     HttpServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
    /// }
    ///
    /// async fn work(_request: Request) -> Response {
    ///     Response::from_body("All Good :)")
    /// }
    /// ```
    pub async fn launch<FutW>(
        listener: TcpListener,
        work_fn: impl Fn(Request) -> FutW + Send + Sync + Copy + 'static,
    ) where
        FutW: Future<Output = Response> + Send + Sync + 'static,
    {
        async fn check(_: SocketAddr) -> bool {
            true
        }

        Self::impl_launch(listener, check, work_fn).await;
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
                if !check_fn(net_addr).await {
                    return Err(ServerError::VerificationFailed);
                }

                Self::handle_connection(socket, net_addr, work_fn).await
            });
        }
    }

    #[inline]
    async fn handle_connection<FutW>(
        stream: TcpStream,
        addr: SocketAddr,
        work_fn: impl Fn(Request) -> FutW + Send + Sync + Copy + 'static,
    ) -> Result<(), ServerError>
    where FutW: Future<Output = Response> + Send + Sync + 'static {
        let (mut read, mut write) = io::split(stream);

        let request = Request::result_from(&mut read, addr).await?;
        let response = work_fn(request).await;

        if let Some(page) = DEF_PAGES.get(&response.status_code) {
            write.write_all(&page).await.map_err(ServerError::Write)?;
        } else {
            write
                .write_all(&response.as_bytes()?)
                .await
                .map_err(ServerError::Write)?;
        }

        write.flush().await.map_err(ServerError::Flush)
    }
}