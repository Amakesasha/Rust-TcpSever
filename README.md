# Rust TcpSever. 

A simple and lightweight crate for launching and using a server. 

# Reasons to choose Rust-TcpSever:
* Not Used Third Party [Libraries](https://github.com/Amakesasha/Rust-TcpSever/blob/main/Cargo.toml)!
* Ease of Use!
* Small Library Size!
* Have [Static Query Processing Thread System](https://github.com/Amakesasha/Rust-TcpSever/blob/main/src/thread_pool.rs)!
* Have [Open Source](https://github.com/Amakesasha/Rust-TcpSever) and [Simple Documentation](https://docs.rs/rust_tcp_sever/latest/rust_tcp_sever/)!
* Have a [Secure License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file)!
* Supports all types of files!

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

# Usage example: 
 ``` Rust
 // file src/main.rs
extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    TcpServer::set_http("HTTP/2.0");
    TcpServer::set_add_job(true);
    TcpServer::set_def_page(Response::new_from_file("src/main.rs", "text/html"));

    let server = TcpServer::new(Server::get_server("127.0.0.1:80"), ThreadPool::new(4));

    Server::launch_range_port(server, 443..444);
}

struct Server;

impl SeverControl for Server {
    #[inline]
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
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener { TcpListener::bind(ip_addr).unwrap() }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {
        match request.url.as_str() {
            "/response" => {
                response.set_file("examples_rs/webpage.html", "text/html");
                response.cookie.add("net", "qwe");
                response.cookie.delete("qwe");
            }

            "/giphy.webp" => response.set_file("examples_rs/giphy.webp", "image/webp"),
            "/image.png" => response.set_file("examples_rs/image.png", "image/png"),
            "/video.mp4" => response.set_file("examples_rs/video.mp4", "video/mp4"),
            "/audio.mp3" => response.set_file("examples_rs/audio.mp3", "audio/mp3"),
            "/favicon.ico" => response.set_file("examples_rs/image.png", "image/png"),

            "/wer" => response.set_redirect("/response"),

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
* Any suggestions for changes should be written in the [Discussion on Github](https://github.com/Amakesasha/Rust-TcpSever/issues) or Email: amakesasha@gmail.com.