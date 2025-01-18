use rust_tcp_sever::*;

#[tokio::main]
async fn main() {
    set_def_pages!((
        "404 NOT FOUND",
        Response::from_file("404.html", "text/html").await.unwrap()
    ));

    #[cfg(feature = "check_stream")]
    HttpServer::launch(TcpListener::bind("127.0.0.1:2").await.unwrap(), check, work).await;
    #[cfg(not(feature = "check_stream"))]
    HttpServer::launch(TcpListener::bind("127.0.0.1:2").await.unwrap(), work).await;
}

#[inline]
#[cfg(feature = "check_stream")]
async fn check(_addr: std::net::SocketAddr) -> bool {
    true
}

#[inline]
async fn work(_request: Request) -> Response {
    Response::new()
}
