use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Request.
pub struct Request {
    /// Request method.
    pub method: HttpMethod,
    /// Request URL.
    pub url: String,

    /// Host.
    pub host: Option<String>,
    /// Cookies.
    pub cookies: HashMap<String, String>,

    /// HTTP headers.
    pub headers: HashMap<String, String>,
    /// HTTP body.
    pub body: Vec<u8>,
}

impl OptionFrom<&mut TcpStream> for Request {
    #[inline]
    fn option_from(stream: &mut TcpStream) -> Option<Request> {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();

        if reader.read_line(&mut request_line).ok()? == 0 {
            return None;
        }

        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return None;
        }

        let mut request = Request {
            method: parts[0].parse().ok()?,
            url: parts[1].to_string(),

            host: None,
            cookies: HashMap::new(),

            headers: HashMap::new(),
            body: Vec::new(),
        };

        while let Some(header_line) = Self::read_header_line(&mut reader) {
            let header_parts: Vec<&str> = header_line.trim().splitn(2, ':').collect();
            if header_parts.len() == 2 {
                request.headers.insert(
                    header_parts[0].trim().to_string(),
                    header_parts[1].trim().to_string(),
                );
            }
        }

        if let Some(length) = request
            .headers
            .get("Content-Length")
            .and_then(|val| val.parse::<usize>().ok())
        {
            let mut body = vec![0; length];
            if reader.read_exact(&mut body).is_ok() {
                request.body = body;
            }
        }

        if let Some(cookies) = request.headers.remove("Cookie") {
            request.cookies = Self::get_data(&cookies, "&");
        }
        if let Some(host) = request.headers.remove("Host") {
            request.host = Some(host);
        }

        Some(request)
    }
}

impl Request {
    #[inline]
    /// Helper function for reading header lines
    /// * reader = Buffer Reader.
    pub fn read_header_line(reader: &mut BufReader<&mut TcpStream>) -> Option<String> {
        let mut header_line = String::new();
        if reader.read_line(&mut header_line).ok()? == 0 || header_line.trim().is_empty() {
            return None;
        }

        Some(header_line)
    }

    #[inline]
    /// Function for parsing a string in a [HashMap].
    /// * data = Parsing string.
    /// * char_split = Divide symbol.
    pub fn get_data(data: &str, char_split: &str) -> HashMap<String, String> {
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
    type Err = bool;

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
            _ => Err(false),
        }
    }
}
