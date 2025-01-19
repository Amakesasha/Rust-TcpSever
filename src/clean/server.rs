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
    /// ```
    /// async fn work(stream: TcpStream) {
    ///     let read_data = CleanServer::read_string(stream).await.unwrap();
    ///     println!("{read_data}");
    /// }
    /// ```
    /// or
    /// ```
    /// async fn work(stream: TcpStream) {
    ///     let mut buffer = tokio::io::BufReader::new(stream);
    ///     let read_data = CleanServer::read_string(&mut buffer).await.unwrap();
    ///     println!("{read_data}");
    /// }
    /// ```
    pub async fn read_string<R: AsyncReadExt + Unpin>(
        reader: &mut R,
    ) -> Result<String, ServerError> {
        let mut buffer = [32; 4096];

        match reader
            .read(&mut buffer)
            .await
            .map_err(ServerError::WriteError)?
        {
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
    /// ```
    /// async fn work(stream: TcpStream) {
    ///     let read_data = CleanServer::read_bytes(stream).await.unwrap();
    ///     println!("{read_data:?}");
    /// }
    /// ```
    /// or
    /// ```
    /// async fn work(stream: TcpStream) {
    ///     let mut buffer = tokio::io::BufReader::new(stream);
    ///     let read_data = CleanServer::read_bytes(&mut buffer).await.unwrap();
    ///     println!("{read_data:?}");
    /// }
    /// ```
    pub async fn read_bytes<R: AsyncReadExt + Unpin>(
        reader: &mut R,
    ) -> Result<Vec<u8>, ServerError> {
        let mut buffer = [32; 4096];

        match reader
            .read(&mut buffer)
            .await
            .map_err(ServerError::WriteError)?
        {
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
    /// ```
    /// async fn work(stream: TcpStream) {
    ///     CleanServer::write(stream, "Sample text xD").await.unwrap();
    /// }
    /// ```
    /// or
    /// ```
    /// async fn work(stream: TcpStream) {
    ///     let mut buffer = tokio::io::BufWriter::new(stream);
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
            .map_err(ServerError::WriteError)
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
    /// ```
    /// use rust_tcp_sever::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     CleanServer::launch(TcpListener::bind("127.0.0.1:80").await.unwrap(), work).await;
    /// }
    ///
    /// async fn work(stream: TcpStream) {}
    /// ```
    pub async fn launch<Fut>(
        listener: TcpListener,
        function: impl Fn(TcpStream) -> Fut + Send + Copy + Sync + 'static,
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
