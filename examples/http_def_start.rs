use http::StatusCode;
use maker_web::{set_def_pages, HttpServer, Request, Response, DEF_PAGES};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    set_def_pages!((
        StatusCode::NOT_FOUND,
        Response::from_body("Page not found :(")
    ));

    HttpServer::launch(TcpListener::bind("127.0.0.1:2").await.unwrap(), work).await;
}

#[inline]
async fn work(_request: Request) -> Response {
    Response::from_body("All Good)")
}
