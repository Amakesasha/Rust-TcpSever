use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Request.
pub struct Request {
    /// Request method.
    pub method: HttpMethod,
    /// Request URL.
    pub url: String,

    /// Host.
    pub host: Option<SocketAddr>,

    /// Cookies.
    pub cookie: HashMap<String, String>,
    /// Additional Content.
    pub add_content: HashMap<String, String>,
    /// Last line of request.
    pub last_line: String,
}

/// Functions for parsing HTTP request in [Request].
impl FromStr for Request {
    type Err = bool;

    #[inline]
    /// Function for parsing a request
    /// * data = HTTP request.
    /// # Examples
    /// ```
    /// const DATA: &str = "GET /response HTTP/1.1 \r\nHost: 127.0.0.1:443 \r\nCookie: net=qwe";
    /// DATA.parse::<Request>().unwrap();
    /// ```
    fn from_str(data: &str) -> Result<Request, Self::Err> {
        let mut split_line: Vec<&str> = data.lines().collect();

        let muh: Vec<&str> = split_line.first().ok_or(false)?.split_whitespace().collect();
        let (method, mut url, last_line) = (
            muh.first().ok_or(false)?,
            muh.get(1).ok_or(false)?.to_string(),
            split_line.pop().ok_or(false)?.to_string(),
        );

        let host = split_line.iter()
            .find(|line| line.starts_with("Host: "))
            .map(|host_line| host_line.trim_start_matches("Host: ").to_socket_addrs())
            .and_then(|addr| addr.ok().and_then(|mut addrs| addrs.next()));

        let cookie = split_line.iter()
            .find(|line| line.starts_with("Cookie: "))
            .map(|cookie_line| Self::get_data(cookie_line.trim_start_matches("Cookie: "), "; "))
            .unwrap_or_default();

        let add_content = if !last_line.contains(": ") {
            Self::get_data(&last_line, "&")
        } else if let Some(index) = url.find('?') {
            Self::get_data(&url.split_off(index + 1), "&")
        } else {
            HashMap::new()
        };

        Ok(Request {
            method: method.parse()?,
            url,

            host,

            cookie,
            add_content,
            last_line,

        })
    }
}

impl Request {
    #[inline]
    /// Function for parsing a string in a [HashMap].
    /// * data = Parsing string.
    /// * char_split = Divide symbol.
    /// # Examples
    /// ```
    /// const DATA: &str = "net=qwe&qwe=qwe&asd=asd";
    /// Request::get_data(DATA, "&");
    /// ```
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

#[derive(Debug, Clone, PartialEq, Eq)]
/// HTTP method. Information taken from [the site](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods).
pub enum HttpMethod {
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
    /// Function for parsing a request
    /// * data = HTTP request.
    /// # Examples
    /// ```
    /// const DATA: &str = "GET";
    /// DATA.parse::<HttpMethod>().unwrap();
    /// ```
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
            _ => Err(false)
        }
    }
}