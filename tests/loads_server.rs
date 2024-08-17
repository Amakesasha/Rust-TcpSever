const TEST_TEXT: &str = r#"GET /responseqwe HTTP/1.1
Host: 127.0.0.1:443
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua: "Not_A Brand";v="99", "Google Chrome";v="109", "Chromium";v="109"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "Windows"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: en-US,en;q=0.9,ru;q=0.8
Cookie: net=qwe"#;

use std::{
    io::Write,
    net::TcpStream,
    time::{Duration, Instant},
};

fn main() {
    println!("TEST | LOADS_SERVER | START");

    let mut total_duration = Duration::new(0, 0);
    let mut count = 0;

    for _ in 0..10 {
        let mut i = 0;
        let start = Instant::now();

        for _ in 0..10_000 {
            match TcpStream::connect("127.0.0.1:443") {
                Ok(mut stream) => match stream.write_all(TEST_TEXT.as_bytes()) {
                    Ok(_) => {}
                    Err(_) => i += 1,
                },
                Err(_) => i += 1,
            }
        }

        let elapsed = start.elapsed();

        total_duration += elapsed;
        count += 1;

        println!("TEST | LOADS_SERVER | {:.2?} | {i}", elapsed);
    }

    println!("TEST | LOADS_SERVER | END | {:.2?}", total_duration / count);

    loop {}
}
