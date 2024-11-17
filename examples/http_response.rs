use rust_tcp_sever::*;

fn main() {
    HttpServer::set_def_pages(vec![(
        String::from("404 NOT FOUND"),
        Response::new_from_file("examples_rs/defpage.html", "text/html"),
    )]);

    Server::http_launch(TcpListener::bind("127.0.0.1:3").unwrap(), 4);
}

struct Server;

impl HttpControl for Server {
    #[inline]
    fn check_stream(_stream: &TcpStream) -> bool { true }

    #[inline]
    fn parser_request(_stream: &TcpStream, request: &Request, response: &mut Response) {
        // Sending files
        match request.url.as_str() {
            "/webpage.html" => response.set_file("examples_rs/webpage.html", "text/html"),
            "/giphy.webp" => response.set_file("examples_rs/giphy.webp", "image/webp"),
            "/image.png" => response.set_file("examples_rs/image.png", "image/png"),
            "/video.mp4" => response.set_file("examples_rs/video.mp4", "video/mp4"),
            "/audio.mp3" => response.set_file("examples_rs/audio.mp3", "audio/mp3"),
            "/favicon.ico" => response.set_file("examples_rs/image.png", "image/png"),
            _ => {}
        }

        match request.url.as_str() {
            // Function from PHP.
            "/echo" => Self::echo(response),
            // Example of working with "ResponseCookies" and "ResponseSetting".
            "/cookie_setting" => Self::cookie_setting(response),
            // Manually entering a response.
            "/all_good" => response.set_response("200 OK", "All Good :>"),
            // Client redirection.
            "/wer" => response.set_redirect("/response"),
            _ => {}
        }

        match request.url.as_str() {
            "/response/clone" => *response = RESPONSE_DEF.clone(),
            "/response/new_from_file" => *response = Response::new_from_file("page.html", "text/html"),
            "/response/new_from_fn" => *response = Response::new_from_fn(|resp| {
                resp.set_response("200 OK", "<p>123<p>");

                resp.cookie += ("Sample Name", "Sample Text");
                resp.setting += ("Content-Type", "text/html");
            }),
            _ => {}
        };
    }
}

impl Server {
    // Function from PHP.
    fn echo(response: &mut Response) {
        response.html(
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

    // Example of working with "ResponseCookies" and "ResponseSetting".
    fn cookie_setting(response: &mut Response) {
        response.cookie += ("Sample Name", "Sample Text");
        response.cookie += ("Test Cookie", "Test Value");
        response.cookie += ("3141592", "3141592");

        response.cookie -= "3141592";

        response.setting += ("Content-Type", "text/html");
        response.setting += ("Data", "12-12-1212");
    }
}