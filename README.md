# Rust TCP Server. 

A simple and lightweight crate for launching and using a server. 

# Reasons to choose Rust-TcpSever:
* Not Used Third Party Library!
* Ease of Use!
* Lightness!
* Have [Open Source](https://github.com/Amakesasha/Rust-TcpServer)!

# Find Bug or Malfunction?

If you find bug or malfunction, please, [Write to Issues GitHub](https://github.com/Amakesasha/Rust-TcpServer/issues)


# Usage example: 
 ``` Rust
extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    Server::launch(TcpServer::new(
        TcpListener::bind("127.0.0.1:8080").unwrap(),
        ThreadPool::new(4),
    ));
}

struct Server;

impl SeverControl for Server {
    const TYPE_HTTP: Option<&'static str> = Some("HTTP/2.0");

    fn match_get(request: &Request, response: &mut Response) {
        match request.metod_url_http[1].as_str() {
            "/response" => {
                response.set_response("200 OK", "All Good");

                response.setting.add("Content-Type", "text/html");

                response.cookie.add("testName", "testValue");
                response.cookie.delete("asdf");
            }
            "/wer" => response.set_redirect("/response"),
            _ => {}
        }
    }
    fn match_post(_request: &Request, response: &mut Response) {
        response.set_redirect("/response");
    }
    fn match_put(_request: &Request, _response: &mut Response) {}
}
 ```

# License
This project is licensed under either of

Apache License, Version 2.0, [LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0)
MIT license [LICENSE-MIT](http://opensource.org/licenses/MIT)
at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in web by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.