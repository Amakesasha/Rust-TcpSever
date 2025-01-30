use crate::*;

/// Non-protocol server.
pub struct CleanServer;

/// Built-in functions for reading and writing.
impl CleanServer {
    #[inline]
    /// Function for reading request into line. 4096 byte buffer!
    ///
    /// # Parameters
    /// * `reader` - An object that provides asynchronous reading of bytes.
    ///
    /// # Examples
    ///
    /// Reading from [TcpStream]:
    /// ```no_run
    /// use maker_web::CleanServer;
    /// use tokio::net::TcpStream;
    ///
    /// async fn work(mut stream: TcpStream) {
    ///     let read_data = CleanServer::read_string(&mut stream).await.unwrap();
    ///     println!("{read_data}");
    /// }
    /// ```
    /// Reading from [tokio::io::BufReader]:
    /// ```no_run
    /// use maker_web::CleanServer;
    /// use tokio::{io::BufReader, net::TcpStream};
    ///
    /// async fn work(mut stream: TcpStream) {
    ///     let mut buffer = BufReader::new(stream);
    ///     let read_data = CleanServer::read_string(&mut buffer).await.unwrap();
    ///     println!("{read_data}");
    /// }
    /// ```
    pub async fn read_string<R: AsyncReadExt + Unpin>(
        reader: &mut R,
    ) -> Result<String, ServerError> {
        let mut buffer = [32; 4096];

        match reader.read(&mut buffer).await.map_err(ServerError::Read)? {
            0 => Err(ServerError::EmptyLine),
            _ => Ok(String::from_utf8_lossy(&buffer).to_string()),
        }
    }

    #[inline]
    /// Function for reading request into bytes. 4096 byte buffer!
    ///
    /// # Parameters
    /// * `reader` - An object that provides asynchronous reading of bytes.
    ///
    /// # Examples
    ///
    /// Reading from [TcpStream]:
    /// ```no_run
    /// use maker_web::CleanServer;
    /// use tokio::net::TcpStream;
    ///
    /// async fn work(mut stream: TcpStream) {
    ///     let read_data = CleanServer::read_bytes(&mut stream).await.unwrap();
    ///     println!("{read_data:?}");
    /// }
    /// ```
    /// Reading from [tokio::io::BufReader]:
    /// ```no_run
    /// use maker_web::CleanServer;
    /// use tokio::{io::BufReader, net::TcpStream};
    ///
    /// async fn work(mut stream: TcpStream) {
    ///     let mut buffer = BufReader::new(stream);
    ///     let read_data = CleanServer::read_bytes(&mut buffer).await.unwrap();
    ///     println!("{read_data:?}");
    /// }
    /// ```
    pub async fn read_bytes<R: AsyncReadExt + Unpin>(
        reader: &mut R,
    ) -> Result<Vec<u8>, ServerError> {
        let mut buffer = [32; 4096];

        match reader.read(&mut buffer).await.map_err(ServerError::Read)? {
            0 => Err(ServerError::EmptyLine),
            num => Ok(buffer[0..num].to_vec()),
        }
    }

    #[inline]
    /// Function to write data to [TcpStream].
    ///
    /// # Parameters
    /// * `writer` - An object that provides asynchronous writing of bytes.
    /// * `data` - Data implementing serialization into bytes for sending over a network.
    ///
    /// # Examples
    ///
    /// Writing to [TcpStream]:
    /// ```no_run
    /// use maker_web::CleanServer;
    /// use tokio::net::TcpStream;
    ///
    /// async fn work(mut stream: TcpStream) {
    ///     CleanServer::write(&mut stream, "Sample text xD").await.unwrap();
    /// }
    /// ```
    /// Writing to [tokio::io::BufWriter]:
    /// ```no_run
    /// use maker_web::CleanServer;
    /// use tokio::{io::BufWriter, net::TcpStream};
    ///
    /// async fn work(mut stream: TcpStream) {
    ///     let mut buffer = BufWriter::new(stream);
    ///     CleanServer::write(&mut buffer, "Sample text xD").await.unwrap();
    /// }
    /// ```
    pub async fn write<W: AsyncWriteExt + Unpin, Q: AsRef<[u8]>>(
        writer: &mut W,
        data: Q,
    ) -> Result<(), ServerError> {
        writer
            .write_all(data.as_ref())
            .await
            .map_err(ServerError::Write)
    }
}

/// Functions for starting and running the server.
impl CleanServer {
    /// Starting the server.
    ///
    /// # Parameters
    /// * `listener` - An asynchronous TCP listener designed for listening to incoming connections.
    /// * `function` - Asynchronous function for working with TcpStream.
    ///
    /// # Examples
    /// ```no_run
    /// use maker_web::CleanServer;
    /// use tokio::net::{TcpListener, TcpStream};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     CleanServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
    /// }
    ///
    /// async fn work(mut stream: TcpStream) {}
    /// ```
    pub async fn launch<Fut>(
        listener: TcpListener,
        function: impl Fn(tokio::net::TcpStream) -> Fut + Send + Copy + Sync + 'static,
    ) where
        Fut: Future<Output = ()> + Send + 'static,
    {
        println!(
            "SERVER | CLEAN | {} | LAUCNH",
            listener.local_addr().unwrap()
        );

        loop {
            if let Ok((socket, _)) = listener.accept().await {
                tokio::spawn(async move {
                    function(socket).await;
                });
            }
        }
    }
}
