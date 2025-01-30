use http::StatusCode;
use rust_tcp_sever::{set_def_pages, HttpServer, Request, Response, DEF_PAGES};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    set_def_pages!((StatusCode::NOT_FOUND, Response::from_body("All Good)")));

    HttpServer::launch_with_check(
        TcpListener::bind("127.0.0.1:80").await.unwrap(),
        work,
        check,
    ).await;
}

#[inline]
async fn check(_addr: std::net::SocketAddr) -> bool {
    true
}

#[inline]
async fn work(_request: Request) -> Response {
    Response::new()
}
