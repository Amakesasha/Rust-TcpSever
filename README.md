# Rust TcpSever. 

A simple and lightweight crate for launching and using a server. 

# Reasons to choose Rust-TcpSever:
* Not Used Third Party [Libraries](https://github.com/Amakesasha/Rust-TcpSever/blob/main/Cargo.toml)!
* [Ease of Use](https://github.com/Amakesasha/Rust-TcpSever/blob/main/examples/default_start.rs)!
* Small Library Size!
* Have [Static Query Processing Thread System](https://github.com/Amakesasha/Rust-TcpSever/blob/main/src/thread_pool.rs)!
* Have [Open Source](https://github.com/Amakesasha/Rust-TcpSever) and [Simple Documentation](https://docs.rs/rust_tcp_sever/latest/rust_tcp_sever/)!
* Have a [Secure License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file).

## Find Bug or Malfunction?

If you find bug or malfunction, please, [Write to Issues GitHub](https://github.com/Amakesasha/Rust-TcpSever/issues) or Email: amakesasha@gmail.com.

## You came up with an improvement to the project?

Please write any suggestions for adding or changing the functionality of the library to the [Discussion](https://github.com/Amakesasha/Rust-TcpSever/issues) or Email: amakesasha@gmail.com. I have no more ideas what to do(((

## What this "Static Query Processing Thread System"?

This is Modified Thread Pool (From [Rust-Official-Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)), with Static System Control Number Worker (Thread Parse Request).
* When Jobs become More or Qqual to the Number of Worker, Workers are Added until they can handle all of them.
* When there are Fewer Jobs than the Number of Worker, Worker are Removed, Leaving one Spare.

# Future of the library

The library will be updated as new proposals and ideas are received. If I no longer want to develop this project, I will write about it. 

#
#

# Installing and Running the Library: 

## Installing: 
``` CMD
cargo add rust_tcp_sever
```
## Running:
``` CMD
cargo run 
``` 
or 
``` CMD
cargo run --release 
```

# Usage example: 
 ``` Rust
 // file src/main.rs
extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    { // Not Necessary!
        let mut response = Response::const_new();
        response.set_file("src/main.rs");

        TcpServer::set_def_page(response);
    }

    Server::launch(TcpServer::new(
        TcpListener::bind("127.0.0.1:8080").unwrap(),
        // number = Minimum Number of Workers (Request Processing Threads)
        ThreadPool::new(4),
    ));
}

struct Server;

impl SeverControl for Server {
    const TYPE_HTTP: Option<&'static str> = Some("HTTP/2.0");

    #[inline]
    fn match_methods(request: &Request, response: &mut Response) {
        match request.metod_url_http[0].as_str() {
            "GET" => Self::match_get(request, response),
            "POST" => Self::match_post(request, response),
            "PUT" => Self::match_put(request, response),
            _ => {}
        }
    }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {
        match request.metod_url_http[1].as_str() {
            "/response" => {
                response.set_response("200 OK", "All Good");

                response.setting.add("Content-Type", "text/html");

                response.cookie.add("testName", "testValue");
                response.cookie.delete("asdf");
            }
            "/wer" => response.set_redirect("/response"),
            "/sleep" => std::thread::sleep(std::time::Duration::from_secs(30)),
            _ => {}
        }
    }

    #[inline]
    fn match_post(_request: &Request, response: &mut Response) {
        response.set_redirect("/response");
    }

    #[inline]
    fn match_put(_request: &Request, _response: &mut Response) {}
}
 ```
#
#
# License
This project is licensed under a [Proprietary License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file).

## Summary:
* You can only change your copy of the project downloaded from [GitHub](https://github.com/Amakesasha/Rust-TcpSever).
* You can only download the library from [GitHub](https://github.com/Amakesasha/Rust-TcpSever) or [crates.io](https://crates.io/crates/rust_tcp_sever).
* It is prohibited to forward even an unmodified copy to other people.
* Copyright of this software remains with the author. All rights reserved.
* The author does not bear any responsibility for damage caused.
* Any suggestions for changes should be written in the [Discussion on Github](https://github.com/Amakesasha/Rust-TcpSever/issues) or Email: amakesasha@gmail.com.