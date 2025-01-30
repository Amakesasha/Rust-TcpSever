use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a parsed HTTP request, containing method, URL, headers, body, and more.
/// Includes fields for host, cookies, and an optional socket address.
pub struct Request {
    /// Client socket address.
    pub socket_addr: SocketAddr,
    /// HTTP request method (GET, POST, etc.).
    pub method: Method,
    /// Requested URL path.
    pub url: Uri,
    /// Request cookies as key-value pairs.
    pub cookies: HashMap<String, String>,
    /// HTTP request headers as key-value pairs.
    pub headers: HeaderMap,
    /// Request body as a byte vector.
    pub body: Vec<u8>,
}

/// Functions for creating [Request].
impl Request {
    #[inline]
    pub(crate) async fn result_from(
        read_half: &mut ReadHalf<TcpStream>,
        adder: SocketAddr,
    ) -> Result<Request, ServerError> {
        let mut reader = BufReader::new(read_half);
        let mut request_line = String::with_capacity(100);

        if reader
            .read_line(&mut request_line)
            .await
            .map_err(ServerError::Read)?
            == 0
        {
            return Err(ServerError::EmptyRequest);
        }

        let mut parts = request_line.split_whitespace();
        let (Some(method), Some(url), Some(_)) = (parts.next(), parts.next(), parts.next()) else {
            return Err(ServerError::BrokenFirstLine);
        };

        let mut request = Request {
            socket_addr: adder,
            method: Method::from_str(method).map_err(ServerError::InvalidMethod)?,
            url: Uri::from_str(url).map_err(ServerError::InvalidUrl)?,

            cookies: HashMap::new(),
            headers: HeaderMap::with_capacity(20),
            body: Vec::new(),
        };

        let mut header_line = String::with_capacity(150);
        loop {
            match Self::read_header_line(&mut reader, &mut header_line, &mut request.headers).await
            {
                Ok(_) => continue,
                Err(ServerError::EmptyLine) => break,
                Err(e) => return Err(e),
            }
        }

        if let Some(length) = request
            .headers
            .get(CONTENT_LENGTH)
            .and_then(|val| val.to_str().ok())
            .and_then(|val| val.parse::<usize>().ok())
        {
            let mut body = vec![0; length];
            if reader.read_exact(&mut body).await.is_ok() {
                request.body = body;
            }
        }

        if let Some(cookies) = request
            .headers
            .get(COOKIE)
            .and_then(|val| val.to_str().ok())
        {
            request.cookies = Self::parse_query_string(cookies, '&');
        }

        Ok(request)
    }

    #[inline]
    async fn read_header_line(
        reader: &mut BufReader<&mut ReadHalf<TcpStream>>,
        header_line: &mut String,
        headers: &mut HeaderMap,
    ) -> Result<(), ServerError> {
        header_line.clear();

        if reader
            .read_line(header_line)
            .await
            .map_err(ServerError::Read)?
            == 0
            || header_line.trim_end().is_empty()
        {
            return Err(ServerError::EmptyLine);
        }

        let mut parts = header_line.trim().splitn(2, ':');

        let name = parts
            .next()
            .ok_or_else(|| ServerError::InvalidHeader)?
            .trim();
        let value = parts
            .next()
            .ok_or_else(|| ServerError::InvalidHeader)?
            .trim();

        headers.insert(HeaderName::from_str(name)?, HeaderValue::from_str(value)?);

        Ok(())
    }

    #[inline]
    /// Parses the input string, splitting it into parts based on the given delimiter.
    /// Then, for each part, attempts to extract a key-value pair separated by the `=` character.
    /// Returns a [HashMap], where the keys and values are strings.
    ///
    /// # Parameters
    /// * `data` - The string to be parsed.
    /// * `char_split` - string pairing symbol.
    ///
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use rust_tcp_sever::Request;
    ///
    /// let data = "key1=value1&key2=value2";
    /// let result = Request::parse_query_string(data, '&');
    ///
    /// assert_eq!(result.len(), 2);
    /// assert_eq!(result.get("key1"), Some(&"value1".to_string()));
    /// assert_eq!(result.get("key2"), Some(&"value2".to_string()));
    /// ```
    pub fn parse_query_string(data: &str, char_split: char) -> HashMap<String, String> {
        data.split(char_split)
            .filter_map(|part| {
                let mut split = part.splitn(2, '=');

                if let (Some(key), Some(value)) = (split.next(), split.next()) {
                    Some((key.trim().to_string(), value.trim().to_string()))
                } else {
                    None
                }
            })
            .collect()
    }
}
