# Rust TCP Server. 

A simple and lightweight crate for launching and using a server. 
Source [code link](https://github.com/Amakesasha/Rust-TcpServer)

# Usage example:

# Usage example: 
 ``` Rust
extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
	// Start Default Server: TcpListener::bind(addr).unwrap()
	// 4 == Number Thread Pool (thread_pool.rs)
    Server::start_def_server("127.0.0.1:8080", 4);
}

struct Server;

impl ServerControl for Server {
	// If None, HTTP = 1.1
    const TYPE_HTTP: Option<&'static str> = Some("HTTP/2.0");

    // Match GET requesr
    // If no Make Response, will come back 404 ERROR.
    fn match_get(request: &Request, response: &mut Response) {
        match request.metod_url_http[1].as_str() {
            "/qwe" => {
                response.set_response("200 OK", std::fs::read_to_string("404.html").unwrap());
                response.response_add_content("Content-Type", "text/html");

                response.cookie.add("test_name", "test_value");
                response.cookie.add("asdf", "fdsa");
                response.cookie.delete("asdf");
            }
            _ => {}
        }
    }

    // Match POST requesr
    fn match_post(_request: &Request, response: &mut Response) {
    	response.set_redirect("/qwe");
    }
    // Match PUT requesr
    fn match_put(_request: &Request, _response: &mut Response) {}
}

 ```