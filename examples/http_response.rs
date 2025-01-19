use rust_tcp_sever::*;

#[tokio::main]
async fn main() {
    #[cfg(feature = "check_stream")]
    HttpServer::launch(
        TcpListener::bind("127.0.0.1:80").await.unwrap(),
        check,
        work,
    )
    .await;
    #[cfg(not(feature = "check_stream"))]
    HttpServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
}

#[inline]
#[cfg(feature = "check_stream")]
async fn check(_addr: std::net::SocketAddr) -> bool {
    true
}

#[inline]
async fn work(request: Request) -> Response {
    let mut response = Response::new();

    // Sending files
    match request.url.as_str() {
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

    match request.url.as_str() {
        // Function from PHP.
        "/echo" => echo(&mut response),
        // Example of working with "Cookies" and "Headers".
        "/cookies_headers" => cookies_headers(&mut response),
        // Manually entering a response.
        "/all_good" => response.set_response("200 OK", "All Good :>"),
        // Client redirection.
        "/wer" => response.set_redirect("/response"),
        _ => {}
    }

    match request.url.as_str() {
        "/response/new" => response = Response::new(),
        "/response/clone" => response = RESPONSE_DEF.clone(),
        "/response/from_response" => response = Response::from_response("200 OK", "<p>123<p>"),
        "/response/new_from_fn" => {
            response = Response::from_fn(|resp| {
                resp.set_response("200 OK", "<p>123<p>");

                resp.add_cookie("Sample Name", "Sample Text");
                resp.add_header("Content-Type", "text/html");
            })
        }
        _ => {}
    }

    response
}

// Function from PHP.
fn echo(response: &mut Response) {
    response.set_html(
        |resp| {
            // Head
            resp.echo(r#"<meta charset="utf-8">"#);
            resp.echo(r#"<title>Sample Name WebPage</title>"#);
        },
        |resp| {
            // Body
            resp.echo("<h1>123</h1>");
            resp.echo("<h3>123</h3>");
            resp.echo("<p>123</p>");
        },
    );
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
