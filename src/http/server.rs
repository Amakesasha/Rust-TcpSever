use crate::*;

lazy_static! {
    /// Code Map Page.
    static ref MAP_CODE_PAGE: Arc<RwLock<HashMap<String, Vec<u8>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

/// Http Server.
static mut HTTP_LISTENER: Option<TcpListener> = None;
/// Version HTTP.
pub static mut TYPE_HTTP: &'static str = "HTTP/1.1";

/// Function for Read Request and Parse from HttpStream.
pub type HttpRead = for<'staitc> fn(&'staitc TcpStream) -> Option<Request>;
/// Function for Write Response into HttpStream.
pub type HttpWrite = for<'a> fn(&'a TcpStream, &'a [u8]);

/// Http Server Structure.
pub struct HttpServer;

/// Functions for Work HttpServer.
impl HttpServer {
    #[inline]
    /// Read Data Send to Stream. Parse this Data Into Request. End Return the Request.
    /// * stream = IpAddr, Client for Read and Write. Only from the Server!
    pub fn read(mut stream: &TcpStream) -> Option<Request> {
        let mut buffer = [32; 1024];

        let str_request = match BufReader::new(&mut stream).read(&mut buffer).ok()? {
            0 => return None,
            _ => String::from_utf8_lossy(&buffer),
        };

        Request::parse_to_self(str_request.trim())
    }

    #[inline]
    /// Write Data in Stream.
    /// * stream = IpAddr client for Read and Write. Only from the Server!
    /// * data = Binary Data (Or String Data Into Binary Data).
    pub fn write(mut stream: &TcpStream, data: &[u8]) {
        BufWriter::new(&mut stream).write(data).unwrap_or(0);
    }
}

/// Functions for Edit Setting HttpServer
impl HttpServer {
    #[inline]
    /// Set Code Page Map. Default Value == Empty Vector.
    /// When Response.status_code == Code from the Map, the Page Associated with it Will be Loaded.
    /// * map_code_page = Code Page Map.
    pub fn set_map_code_page(map_code_page: Vec<(String, Response)>) {
        *MAP_CODE_PAGE.write().unwrap() = map_code_page
            .iter()
            .map(|(code, page)| {
                (
                    code.clone(),
                    [
                        Response::format_arg("200 OK", page).as_bytes(),
                        &page.binary_data,
                    ]
                    .concat(),
                )
            })
            .collect::<HashMap<String, Vec<u8>>>();
    }

    #[inline]
    /// Set Http Type. Default Value == HTTP/1.1
    /// * When Invalid HTTP, HTTP = 1.0.
    /// * http = A New Http Type.
    pub fn set_http(http: &'static str) {
        unsafe {
            TYPE_HTTP = http;
        }
    }

    #[inline]
    /// Set Http Server. Default Value == None
    /// * When Value == None, Will Load Error.
    /// * server = Http Server.
    pub fn set_server(server: TcpListener) {
        unsafe {
            HTTP_LISTENER = Some(server);
        }
    }

    #[inline]
    /// Set Http Server. Default Value == None
    /// * When Value == None, Will Load Error.
    /// * server = Http Server.
    pub fn get_server<'a>() -> &'static TcpListener {
        unsafe { HTTP_LISTENER.as_ref().unwrap() }
    }
}

/// Trait Control Server.
pub trait HttpSever {
    const FN_READ: HttpRead;
    const FN_WRITE: HttpWrite;

    #[inline]
    /// Launches Read-Write Server.
    /// * num_thr = Number Workers in ThreadPool.
    fn launch(num_thr: usize) {
        HttpServerInfo::launch();

        let thread_pool = ThreadPool::new(num_thr);

        for stream in HttpServer::get_server().incoming().filter_map(Result::ok) {
            thread_pool.add_job(|| Self::handle_connection(stream).unwrap_or(()));
        }

        HttpServerInfo::shotdown();
    }

    #[inline]
    /// Read HTTP Request, make Response, and Write this Response.
    /// At start, Requst Read and Write to byte Buffer, then byte Buffer transfer to Line.
    /// Launches Parser with Line into Request, and make Response.
    /// You check request and return Request (or Not return).
    /// Response Write into Line, and Write to Client Buffer.
    /// * stream = Thread Read-Write between Server and Client.
    fn handle_connection(stream: TcpStream) -> Option<()> {
        if !Self::check_stream(&stream) {
            return None;
        }

        let request = Self::FN_READ(&stream)?;
        let mut response = RESPONSE_DEF.clone();

        Self::parser_request(&stream, &request, &mut response);

        match MAP_CODE_PAGE.read().ok()?.get(&response.status_code) {
            Some(data) => Self::FN_WRITE(&stream, &data),
            None => {
                let form_arg = Response::format_arg(&response.status_code, &response);

                let mut data = Vec::with_capacity(form_arg.len() + response.binary_data.len());
                data.extend_from_slice(form_arg.as_bytes());
                data.extend_from_slice(&response.binary_data);

                Self::FN_WRITE(&stream, &data);
            }
        }

        Some(())
    }

    /// Your Check Ip. Starting in Start.
    /// * stream = Thread Read-Write between Server and Client.
    fn check_stream(stream: &TcpStream) -> bool;

    /// Your Parser Request. Usually Used GET and POST, sometimes used metod PUT.
    /// The rest are not usually used, but they can be used.
    /// * stream = Thread Read-Write between Server and Client.
    /// * request = Parsed Http Request.
    /// * response = Your Response.
    fn parser_request(stream: &TcpStream, request: &Request, response: &mut Response);
}

/// Struct For Print Information about Working Server.
pub struct HttpServerInfo;

impl HttpServerInfo {
    #[inline]
    /// Print about Launch Server.
    pub fn launch() {
        println!("SERVER | HTTP | {} | LAUNCH ", HttpServer::get_server().local_addr().unwrap());
    }

    #[inline]
    /// Print about ShotDown Server.
    pub fn shotdown() {
        println!("SERVER | HTTP | {} | SHOT DOWN ", HttpServer::get_server().local_addr().unwrap());
    }
}
