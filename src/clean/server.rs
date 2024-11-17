use crate::*;

/// TCP server.
pub struct CleanServer;

/// Default server functions.
impl CleanServer {
    #[inline]
    /// Function for reading request.
    /// * stream = Client IP address.
    /// # Examples
    /// ```
    /// let stream = TcpStream::connect("Your Ip").unwrap();
    /// CleanServer::read(&stream).unwrap();
    /// ```
    pub fn read(mut stream: &TcpStream) -> Option<String> {
        let mut buffer = [32; 1024];

        return match BufReader::new(&mut stream).read(&mut buffer).ok()? {
            0 => None,
            _ => Some(String::from_utf8_lossy(&buffer).to_string()),
        };
    }

    #[inline]
    /// Function to write data to [TcpStream].
    /// * stream = Client IP address.
    /// * data = Output data.
    /// # Examples
    /// ```
    /// let stream = TcpStream::connect("Your Ip").unwrap();
    /// HttpServer::write(&stream, b"qweqwe");
    /// ```
    pub fn write<Q: AsRef<[u8]>>(mut stream: &TcpStream, data: Q) {
        BufWriter::new(&mut stream)
            .write_all(data.as_ref())
            .unwrap_or(());
    }
}

/// Trait for server operation.
pub trait CleanControl {
    #[inline]
    /// Starting the server.
    /// * listener = TcpListener.
    /// * num_thr = Number of threads.
    /// # Examples
    /// ```
    /// fn example() {
    ///     Server::clean_launch(TcpListener::bind("127.0.0.1:80").unwrap(), 4);
    /// }
    ///
    /// struct Server;
    ///
    /// impl CleanControl for Server {
    ///     fn work(_stream: &TcpStream) {}
    /// }
    /// ```
    fn clean_launch(listener: TcpListener, num_thr: usize) {
        ServerInfo::launch(&listener, ServerInfo::Clean);

        let mut thread_pool = ThreadStream::new(num_thr, Self::work);

        for stream in listener.incoming().filter_map(Result::ok) {
            thread_pool += stream;
        }

        ServerInfo::shotdown(&listener, ServerInfo::Clean);
    }

    /// Your work with TcpStream;
    /// * stream = Client IP address.
    fn work(stream: &mut TcpStream) -> Option<()>;
}
