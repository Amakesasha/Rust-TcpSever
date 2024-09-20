// Call this Functions from ['SeverControl::parser_request'].

fn send_file(request: &Request, response: &mut Response) {
    match request.url.as_str() {
        "/webpage.html" => response.set_file("examples_rs/webpage.html", "text/html"),
        "/giphy.webp" => response.set_file("examples_rs/giphy.webp", "image/webp"),
        "/image.png" => response.set_file("examples_rs/image.png", "image/png"),
        "/video.mp4" => response.set_file("examples_rs/video.mp4", "video/mp4"),
        "/audio.mp3" => response.set_file("examples_rs/audio.mp3", "audio/mp3"),
        "/favicon.ico" => response.set_file("examples_rs/image.png", "image/png"),
        _ => {}
    }
}

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

fn cookie_and_setting(response: &mut Response) {
    response.cookie.add("Sample Name", "Sample Text");
    response.cookie.add("Test Cookie", "Test Value");
    response.cookie.add("3141592", "3141592");

    response.cookie.delete("3141592");

    response.setting.add("Content-Type", "text/html");
    response.setting.add("Data", "12-12-1212");
}

fn make_response() {
    let _ = Response::new();
    let _ = Response::new_from_file("examples_rs/webpage.html", "text/html");
    let _ = Response::new_from_fn(|resp| {
        resp.set_response("200 OK", "<p>123<p>");

        resp.cookie.add("Sample Name", "Sample Text");
        resp.setting.add("Content-Type", "text/html");
    });
}

fn rest_function(request: &Request, response: &mut Response) {
    match request.url.as_str() {
        "/all_good" => response.set_response("200 OK", "All Good :>"),
        "/wer" => response.set_redirect("/response"),
        _ => {}
    }
}