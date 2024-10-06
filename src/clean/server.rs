use crate::*;

/// Tcp Server.
static mut CLEAN_LISTENER: Option<TcpListener> = None;

/// Tcp Server Structure.
pub struct CleanServer;

/// Functions for Work CleanServer.
impl CleanServer {
    #[inline]
    /// Read Data Send to Stream. Parse this Data Into String. End Return the Line.
    /// * stream = IpAddr, Client for Read and Write. Only from the Server!
    pub fn read(mut stream: &TcpStream) -> Option<String> {
        let mut buffer = [32; 1024];

        return match BufReader::new(&mut stream).read(&mut buffer).ok()? {
            0 => None,
            _ => Some(String::from_utf8_lossy(&buffer).to_string()),
        };
    }

    #[inline]
    /// Write Data in Stream.
    /// * stream = IpAddr client for Read and Write. Only from the Server!
    /// * data = Binary Data (Or String Data Into Binary Data).
    pub fn write<Q: AsRef<[u8]>>(mut stream: &TcpStream, data: Q) {
        BufWriter::new(&mut stream).write(data.as_ref()).unwrap_or(0);
    }
}

/// Functions for Edit Setting CleanServer
impl CleanServer {
    #[inline]
    /// Set CleanServer. Default Value == None
    /// * When Value == None, Will Load Error.
    /// * server = CleanServer.
    pub fn set_server(server: TcpListener) {
        unsafe {
            CLEAN_LISTENER = Some(server);
        }
    }

    #[inline]
    /// Set CleanServer. Default Value == None
    /// * When Value == None, Will Load Error.
    /// * server = CleanServer.
    pub fn get_server<'a>() -> &'static TcpListener {
        unsafe { CLEAN_LISTENER.as_ref().unwrap() }
    }
}

/// Trait Control Server.
pub trait CleanSever {
    #[inline]
    /// Launches Read-Write in the Loop, Server.
    /// * num_thr = Number Workers in ThreadPool.
    fn launch(num_thr: usize) {
        CleanServerInfo::launch();

        let thread_pool = ThreadPool::new(num_thr);

        for stream in CleanServer::get_server().incoming().filter_map(Result::ok) {
            thread_pool.add_job(move || Self::work(&stream));
        }

        CleanServerInfo::shotdown();
    }

    /// Your Functuin for Work with Client.
    /// * stream = Thread Read-Write between Server and Client.
    fn work(stream: &TcpStream);
}

/// Struct For Print Information about Working Server.
pub struct CleanServerInfo;

impl CleanServerInfo {
    #[inline]
    /// Print about Launch Server.
    pub fn launch() {
        println!("SERVER | CLEAN | {} | LAUNCH ", CleanServer::get_server().local_addr().unwrap());
    }

    #[inline]
    /// Print about ShotDown Server.
    pub fn shotdown() {
        println!("SERVER | CLEAN | {} | SHOT DOWN ", CleanServer::get_server().local_addr().unwrap());
    }
}
