# Rust TcpSever

A simple and lightweight crate for launching and using a server. 

# Reasons to choose Rust-TcpSever:
* Have [Request Processing Thread System](https://github.com/Amakesasha/Rust-TcpSever/blob/main/src/thread_pool.rs) (Thread Pool, [Rust-Official-Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html))!
* Have [Open Source](https://github.com/Amakesasha/Rust-TcpSever) and [Simple Documentation](https://docs.rs/rust_tcp_sever/latest/rust_tcp_sever/)!
* Have a [Secure License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file)!
* Have a [Discord Server](https://discord.com/invite/dYz6sYmmuu) with Direct Connection to Me!
---
* Not Used Third Party [Libraries](https://github.com/Amakesasha/Rust-TcpSever/blob/main/Cargo.toml)!
* [Ease of Use](https://github.com/Amakesasha/Rust-TcpSever/blob/examples/default_start.rs)!
* Small Library Size!
* Supports all types of files!

# Contact with the developer.
* [Issues GitHub](https://github.com/Amakesasha/Rust-TcpSever/issues) 
* amakesasha@gmail.com
* [Discord Server](https://discord.com/invite/dYz6sYmmuu)

# Future of the library

The library will be updated as new proposals and ideas are received. If I no longer want to develop this project, I will write about it. 

# Usage example: 
*  [More Examples](https://github.com/Amakesasha/Rust-TcpSever/blob/main/examples)
 ``` Rust
// src/main.rs
extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    // Set Type Http, not necessary.
    TcpServer::set_http("HTTP/2.0");
    // Set Page Code Map, preferably)
    TcpServer::set_map_code_page(vec![
        (
            // Status Code.
            String::from("404 NOT FOUND"),
            // Your Response for this Status Code.
            Response::new_from_file("examples_rs/defpage.html", "text/html"),
        )
    ]);

    // Creating a Server.
    let server = TcpServer::new(Server::get_server("127.0.0.1:80"), ThreadPool::new(4));
    // Running a server on multiple ports (in this case 443).
    Server::launch_range_port(server, 443..444);
}

struct Server;

impl SeverControl for Server {
    #[inline]
    // Your Parsed Request.
    fn match_methods(request: &Request, response: &mut Response) {
        // Delet This Code Line)
        println!("{:#?}", request);

        match request.metod.as_str() {
           "GET" => Self::match_get(request, response),
           "POST" => Self::match_post(request, response),
           "PUT" => Self::match_put(request, response),
           _ => {}
        }

        // Delet This Code Line)
        println!("{:#?}", response);
    }

    #[inline]
    // Create Server, you can leave it like that.
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener { TcpListener::bind(ip_addr).unwrap() }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {
        match request.url.as_str() {
            // Work) Make Html File, as echo() from PHP.
            "/qwe" => response.html(
                |resp| {
                    resp.echo(r#"<meta charset="utf-8">"#);
                    resp.echo(r#"<title>Cat Dark Net</title>"#);
                },
                |resp| {
                    resp.echo("<h1>123</h1>");
                    resp.echo("<h3>123</h3>");
                    resp.echo("<p>123</p>");
                },
            ),
            
            // Work Only Cookie( Just an example.
            "/response" => {
                response.set_file("examples_rs/webpage.html", "text/html");
                response.cookie.add("net", "qwe");
                response.cookie.delete("qwe");
            }

            // Don't Work( Just an example.
            "/giphy.webp" => response.set_file("examples_rs/giphy.webp", "image/webp"),
            "/image.png" => response.set_file("examples_rs/image.png", "image/png"),
            "/video.mp4" => response.set_file("examples_rs/video.mp4", "video/mp4"),
            "/audio.mp3" => response.set_file("examples_rs/audio.mp3", "audio/mp3"),
            "/favicon.ico" => response.set_file("examples_rs/image.png", "image/png"),

            // Work)
            "/wer" => response.set_redirect("/response"),
            // Work)
            "/sleep" => std::thread::sleep(std::time::Duration::from_secs(30)),
            _ => {}
        }
    }

    #[inline]
    fn match_post(_request: &Request, _response: &mut Response) {}
    #[inline]
    fn match_put(_request: &Request, _response: &mut Response) {}
}
 ```
# Author Support

## SWIFT transfer:
Information on SWIFT transfer:

* Full name: Gakh Alexander Nikolaevich
* SWIFT-code Bank: TICSRUMM
* IBAM: RU9104452597440817810500116388926

# License
This project is licensed under a [Proprietary License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file).

## Summary:
* By downloading the library, you automatically agree to the license.
* You can only change your copy of the project downloaded from [GitHub](https://github.com/Amakesasha/Rust-TcpSever).
* You can only download the library from [GitHub](https://github.com/Amakesasha/Rust-TcpSever) or [crates.io](https://crates.io/crates/rust_tcp_sever).
* It is prohibited to forward even an unmodified copy to other people.
* Copyright of this software remains with the author. All rights reserved.
* The author does not bear any responsibility for damage caused.