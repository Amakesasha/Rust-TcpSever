use rust_tcp_sever::*;

#[tokio::main]
async fn main() {
    CleanServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
}

async fn work(mut stream: TcpStream) {
    for _ in 0..3 {
        println!("{}", CleanServer::read_string(&mut stream).await.unwrap());

        CleanServer::write(&mut stream, "qwe").await.unwrap();
    }
}
