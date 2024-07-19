extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    Server::start_def_server("127.0.0.1:8080", 4);
}

struct Server;

impl ServerControl for Server {
    const TYPE_HTTP: Option<&'static str> = Some("HTTP/2.0");

    fn match_get(request: &Request, response: &mut Response) {
        match request.metod_url_http[1].as_str() {
            "/qwe" => {
                response.set_response("200 OK", std::fs::read_to_string("404.html").unwrap());
                response.response_add_content("Content-Type", "text/html");
                response
                    .cookie
                    .add("ASDOhufhiaudhsu", "pidoasdgasfhgdfcxcrasina");
                response.cookie.add("asdf", "fdsa");
                response.cookie.delete("asdf");
            }
            _ => {}
        }
    }
    fn match_post(_request: &Request, _response: &mut Response) {}
    fn match_put(_request: &Request, _response: &mut Response) {}
}
