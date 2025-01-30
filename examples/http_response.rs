use http::{StatusCode, Uri};
use maker_web::{HttpServer, Request, Response};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    HttpServer::launch(TcpListener::bind("127.0.0.1:3").await.unwrap(), work).await;
}

#[inline]
async fn work(request: Request) -> Response {
    let mut response = Response::new();

    // Sending files
    match request.url.path() {
        "/webpage.html" => response
            .set_file("examples_rs/webpage.html", "text/html")
            .await
            .unwrap(),
        "/giphy.webp" => response
            .set_file("examples_rs/giphy.webp", "image/webp")
            .await
            .unwrap(),
        "/image.png" => response
            .set_file("examples_rs/image.png", "image/png")
            .await
            .unwrap(),
        "/video.mp4" => response
            .set_file("examples_rs/video.mp4", "video/mp4")
            .await
            .unwrap(),
        "/audio.mp3" => response
            .set_file("examples_rs/audio.mp3", "audio/mp3")
            .await
            .unwrap(),
        "/favicon.ico" => response
            .set_file("examples_rs/image.png", "image/png")
            .await
            .unwrap(),
        _ => {}
    }

    match request.url.path() {
        // Example of working with "Cookies" and "Headers".
        "/cookies_headers" => cookies_headers(&mut response),
        // Manually entering a response.
        "/all_good" => response.set_response(StatusCode::OK, "All Good :>"),
        // Client redirection.
        "/redirect_str" => response.set_redirect_str("/response"),
        "/redirect_uri" => response.set_redirect_uri("/foo/bar?baz".parse::<Uri>().unwrap()),
        _ => {}
    }

    match request.url.path() {
        "/response/new" => response = Response::new(),
        "/response/from_response" => {
            response = Response::from_response(StatusCode::OK, "<p>123<p>")
        }
        "/response/from_file" => {
            response = Response::from_file("examples_rs/webpage.html", "text/html")
                .await
                .unwrap()
        }
        "/response/new_from_fn" => {
            response = Response::from_fn(|resp| {
                resp.set_response(StatusCode::OK, "<p>123<p>");

                resp.add_cookie("Sample Name", "Sample Text");
                resp.add_header("Content-Type", "text/html");
            })
        }
        _ => {}
    }

    response
}

// Example of working with "Cookies" and "Headers".
fn cookies_headers(response: &mut Response) {
    response.add_cookie("Sample Name", "Sample Text");
    response.add_cookie("Test Cookie", "Test Value");
    response.add_cookie("3141592", "3141592");

    response.delete_cookie("3141592");

    response.add_header("Content-Type", "text/html");
    response.add_header("Data", "12-12-1212");
}
