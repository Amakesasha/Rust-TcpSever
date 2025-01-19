use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a parsed HTTP request, containing method, URL, headers, body, and more.
/// Includes fields for host, cookies, and an optional socket address.
pub struct Request {
    #[cfg(feature = "get_stream")]
    /// Client socket address, available when `get_stream` feature is enabled.
    pub socket_addr: SocketAddr,
    /// HTTP request method (GET, POST, etc.).
    pub method: HttpMethod,
    /// Requested URL path.
    pub url: String,
    /// Optional `Host` header.
    pub host: Option<String>,
    /// Request cookies as key-value pairs.
    pub cookies: HashMap<String, String>,
    /// HTTP request headers as key-value pairs.
    pub headers: HashMap<String, String>,
    /// Request body as a byte vector.
    pub body: Vec<u8>,
}

/// Functions for creating [Request].
impl Request {
    #[inline]
    pub(crate) async fn result_from(
        read_half: &mut ReadHalf<TcpStream>,
        adder: Option<SocketAddr>,
    ) -> Result<Request, ServerError> {
        #[cfg(not(feature = "get_stream"))]
        let _ = adder;

        let mut reader = BufReader::new(read_half);
        let mut request_line = String::with_capacity(100);

        if reader
            .read_line(&mut request_line)
            .await
            .map_err(ServerError::ReadError)?
            == 0
        {
            return Err(ServerError::EmptyRequest);
        }

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ServerError::BrokenFirstLine);
        }

        let mut request = Request {
            #[cfg(feature = "get_stream")]
            socket_addr: adder.ok_or_else(|| ServerError::SocketAddrEmpty)?,
            method: parts[0].parse()?,
            url: parts[1].to_string(),

            host: None,
            cookies: HashMap::new(),

            headers: HashMap::new(),
            body: Vec::new(),
        };

        loop {
            match Self::read_header_line(&mut reader).await {
                Ok(header_line) => {
                    let header_parts: Vec<&str> = header_line.trim().splitn(2, ':').collect();
                    if header_parts.len() == 2 {
                        request.headers.insert(
                            header_parts[0].trim().to_string(),
                            header_parts[1].trim().to_string(),
                        );
                    }
                }
                Err(ServerError::EmptyLine) => break,
                Err(e) => return Err(e),
            }
        }

        if let Some(length) = request
            .headers
            .get("Content-Length")
            .and_then(|val| val.parse::<usize>().ok())
        {
            let mut body = vec![0; length];
            if reader.read_exact(&mut body).await.is_ok() {
                request.body = body;
            }
        }

        if let Some(cookies) = request.headers.remove("Cookie") {
            request.cookies = Self::get_data(&cookies, "&");
        }
        if let Some(host) = request.headers.remove("Host") {
            request.host = Some(host);
        }

        Ok(request)
    }

    #[inline]
    async fn read_header_line(
        reader: &mut BufReader<&mut ReadHalf<TcpStream>>,
    ) -> Result<String, ServerError> {
        let mut header_line = String::with_capacity(150);
        if reader
            .read_line(&mut header_line)
            .await
            .map_err(ServerError::ReadError)?
            == 0
            || header_line.trim_end().is_empty()
        {
            return Err(ServerError::EmptyLine);
        }

        Ok(header_line)
    }

    #[inline]
    fn get_data(data: &str, char_split: &str) -> HashMap<String, String> {
        data.split(char_split)
            .filter_map(|part| {
                let mut split = part.splitn(2, '=');

                if let (Some(key), Some(value)) = (split.next(), split.next()) {
                    Some((String::from(key.trim()), String::from(value.trim())))
                } else {
                    None
                }
            })
            .collect()
    }
}

//

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// HTTP method. Information taken from [the site](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods).
pub enum HttpMethod {
    #[default]
    /// The GET method requests a representation of the specified resource.
    /// Requests using GET should only retrieve data and should not contain a request content.
    Get,
    /// The HEAD method asks for a response identical to a GET request, but without a response body.
    Head,
    /// The POST method submits an entity to the specified resource,
    /// often causing a change in state or side effects on the server.
    Post,
    /// The PUT method replaces all current representations of the target resource with the request content.
    Put,
    /// The DELETE method deletes the specified resource.
    Delete,
    /// The CONNECT method establishes a tunnel to the server identified by the target resource.
    Connect,
    /// The OPTIONS method describes the communication options for the target resource.
    Options,
    /// The TRACE method performs a message loop-back test along the path to the target resource.
    Trace,
    /// The PATCH method applies partial modifications to a resource.
    Patch,
}

impl FromStr for HttpMethod {
    type Err = ServerError;

    #[inline]
    fn from_str(data: &str) -> Result<HttpMethod, Self::Err> {
        match data {
            "GET" => Ok(HttpMethod::Get),
            "HEAD" => Ok(HttpMethod::Head),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "CONNECT" => Ok(HttpMethod::Connect),
            "OPTIONS" => Ok(HttpMethod::Options),
            "TRACE" => Ok(HttpMethod::Trace),
            "PATCH" => Ok(HttpMethod::Patch),
            _ => Err(ServerError::UnknownMethod(data.to_string())),
        }
    }
}
