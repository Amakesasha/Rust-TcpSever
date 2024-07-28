# Rust TcpSever. 

A simple and lightweight crate for launching and using a server. 

# Reasons to choose Rust-TcpSever:
* Not Used Third Party [Libraries](https://github.com/Amakesasha/Rust-TcpSever/blob/main/Cargo.toml)!
* [Ease of Use](https://github.com/Amakesasha/Rust-TcpSever/blob/main/examples/default_start.rs)!
* Small Library Size!
* Have [Static Query Processing Thread System](https://github.com/Amakesasha/Rust-TcpSever/blob/main/src/thread_pool.rs)!
* Have [Open Source](https://github.com/Amakesasha/Rust-TcpSever) and [Simple Documentation](https://docs.rs/rust_tcp_sever/latest/rust_tcp_sever/)!
* Have a [Secure License](https://github.com/Amakesasha/Rust-TcpSever/?tab=License-1-ov-file)!
* The Server can [Send any Files](https://github.com/Amakesasha/Rust-TcpSever/blob/main/examples/default_start.rs), and the Site Will Understand them!

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

# Installing and Running the Library: 

## Installing: 
``` CMD
cargo add rust_tcp_sever
```
## Running:
``` CMD
cargo run 
``` 

# Usage example: 
 ``` Rust
 // file src/main.rs
extern crate rust_tcp_sever;
pub use rust_tcp_sever::*;

fn main() {
    TcpServer::set_def_page(Response::new_from_file("src/lib.rs", "text/html"));

    let server = TcpServer::new(Server::get_server("127.0.0.1:8077"), ThreadPool::new(4));

    Server::launch_range_port(server, 8075..8080);
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

    #[inline]
    fn get_server<T: ToSocketAddrs>(ip_addr: T) -> TcpListener {
        TcpListener::bind(ip_addr).unwrap()
    }
}

impl Server {
    #[inline]
    fn match_get(request: &Request, response: &mut Response) {
        match request.metod_url_http[1].as_str() {
            "/response" => {
                response.set_file("src/main.rs", "text/html"),

                response.setting.add("Data", "Now");

               response.cookie.add("testName", "testValue");
               response.cookie.delete("asdf");
            }

            // Not Inserted Due to Weight Restrictions :(

            //"/image.png" => response.set_file("examples/image.png", "image/png"),
            //"/video.mp4" => response.set_file("examples/video.mp4", "video/mp4"),
            //"/audio.mp3" => response.set_file("examples/audio.mp3", "audio/mp3"),

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

## Other

I can't use other translations yet :)

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