use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Response.
pub struct Response {
    /// HTTP status code.
    pub status_code: StatusCode,
    /// Response body.
    pub body: BytesMut,
    /// Response Cookies.
    pub cookies: BytesMut,
    /// Response Headers.
    pub headers: BytesMut,
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions for formatting [Response].
impl Response {
    #[inline]
    /// Translation of [Response] into byte code.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    /// use http::StatusCode;
    /// use bytes::Bytes;
    ///
    /// let mut response = Response::from_response(StatusCode::OK, "body_data");
    /// response.add_cookie("cName", "cValue");
    /// response.add_header("Name", "Value");
    ///
    /// let data = format!(
    ///     "HTTP/1.1 200 OK\r\nSet-Cookie: cName=cValue\r\nName: Value\r\n\r\nbody_data"
    /// );
    ///
    /// assert_eq!(response.as_bytes().unwrap(), Bytes::from(data));
    /// ```
    pub fn as_bytes(&self) -> Result<Bytes, ServerError> {
        let mut binding = itoa::Buffer::new();
        let status_code = binding.format(self.status_code.as_u16()).as_bytes();
        let reason = self
            .status_code
            .canonical_reason()
            .ok_or_else(|| ServerError::UnknownHttpStatus(self.status_code.as_u16()))?
            .as_bytes();

        let mut bytes = BytesMut::with_capacity(
            15 + reason.len() + self.cookies.len() + self.headers.len() + self.body.len(),
        );

        bytes.extend_from_slice(b"HTTP/1.1 ");
        bytes.extend_from_slice(status_code);
        bytes.extend_from_slice(b" ");
        bytes.extend_from_slice(reason);
        bytes.extend_from_slice(b"\r\n");

        bytes.extend_from_slice(&self.cookies);
        bytes.extend_from_slice(&self.headers);
        bytes.extend_from_slice(&self.body);

        Ok(bytes.freeze())
    }
}

/// Functions for creating [Response].
impl Response {
    #[inline]
    /// Creating a default instance of a [Response].
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    /// use http::StatusCode;
    ///
    /// let response = Response::new();
    ///
    /// assert_eq!(response.status_code, StatusCode::NOT_FOUND);
    ///
    /// assert!(response.body.is_empty());
    /// assert!(response.cookies.is_empty());
    /// assert!(response.headers.is_empty());
    /// ```
    pub fn new() -> Self {
        Response {
            status_code: http::StatusCode::NOT_FOUND,
            body: BytesMut::new(),
            cookies: BytesMut::new(),
            headers: BytesMut::new(),
        }
    }

    #[inline]
    /// Creating a new instance of a [Response] from a body.
    ///
    /// # Parameters
    /// * `data` - Response body data.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    /// use http::StatusCode;
    ///
    /// let response = Response::from_body("data");
    ///
    /// assert_eq!(response.status_code, StatusCode::OK);
    /// assert_eq!(response.body.as_ref(), b"\r\ndata");
    /// ```
    pub fn from_body<W: AsRef<[u8]>>(data: W) -> Self {
        let mut response = Response::new();
        response.set_response(StatusCode::OK, data);
        response
    }

    #[inline]
    /// Creating a new instance of a [Response] from a status and body.
    ///
    /// # Parameters
    /// * `status` - HTTP status code (e.g., StatusCode::OK, StatusCode::NOT_FOUND).
    /// * `data` - Response body data.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    /// use http::StatusCode;
    ///
    /// let response = Response::from_response(StatusCode::OK, "data");
    ///
    /// assert_eq!(response.status_code, StatusCode::OK);
    /// assert_eq!(response.body.as_ref(), b"\r\ndata");
    /// ```
    pub fn from_response<W: AsRef<[u8]>>(status: StatusCode, data: W) -> Self {
        let mut response = Response::new();
        response.set_response(status, data);
        response
    }

    #[inline]
    /// Creating a new instance of a [Response] from a function.
    ///
    /// # Parameters
    /// * `fn_edit` - Function to change [Response].
    ///
    /// # Examples
    /// ```   
    /// use rust_tcp_sever::Response;
    /// use http::StatusCode;
    ///
    /// let response = Response::from_fn(|resp| {
    ///     resp.set_response(StatusCode::OK, "<p>123<p>");
    /// });
    ///
    /// assert_eq!(response.status_code, StatusCode::OK);
    /// assert_eq!(response.body.as_ref(), b"\r\n<p>123<p>");
    /// ```
    pub fn from_fn<F: FnOnce(&mut Response)>(fn_edit: F) -> Self {
        let mut response = Response::new();
        fn_edit(&mut response);
        response
    }

    #[inline]
    /// Creating a new instance of a [Response] from files.
    /// If the file cannot be opened or read, the status_code will be written "404 NOT FOUND".
    ///
    /// # Parameters
    /// * `file_path` - Path to the file.
    /// * `type_file` - Content type of the file.
    ///
    /// # Examples
    /// ```no_run
    /// use rust_tcp_sever::Response;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut response = Response::new();
    ///     response.set_file("path", "html/text").await.unwrap();
    /// }
    /// ```
    pub async fn from_file<P: AsRef<Path>, D: AsRef<[u8]>>(
        file_path: P,
        type_file: D,
    ) -> Result<Self, ServerError> {
        let mut response = Response::new();
        response.set_file(file_path, type_file).await?;
        Ok(response)
    }
}

/// Functions to change [Response].
impl Response {
    #[inline]
    /// Inserts HTTP code status and data into Response.
    ///
    /// # Parameters
    /// * `status` - HTTP status code (e.g., StatusCode::OK, StatusCode::NOT_FOUND).
    /// * `data` - Response body data.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    /// use http::StatusCode;
    ///
    /// let mut response = Response::new();
    /// response.set_response(StatusCode::OK, "data");
    ///
    /// assert_eq!(response.status_code, StatusCode::OK);
    /// assert_eq!(response.body.as_ref(), b"\r\ndata");
    /// ```
    pub fn set_response<W: AsRef<[u8]>>(&mut self, status: StatusCode, data: W) {
        self.status_code = status;

        let data = data.as_ref();

        self.body = BytesMut::with_capacity(data.len() + 4);
        self.body.extend_from_slice(b"\r\n");
        self.body.extend_from_slice(data);
    }

    #[inline]
    /// Redirecting the client to a specific url.
    ///
    /// # Parameters
    /// * `location` - The URL to redirect to in [&str].
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    /// use http::StatusCode;
    ///
    /// let mut response = Response::new();
    /// response.set_redirect_str("/qwe/qwe");
    ///
    /// assert_eq!(response.status_code, StatusCode::FOUND);
    /// assert_eq!(response.body.as_ref(), b"Location: /qwe/qwe");
    /// ```
    pub fn set_redirect_str<Q: AsRef<[u8]>>(&mut self, location: Q) {
        self.status_code = StatusCode::FOUND;

        let location = location.as_ref();

        self.body = BytesMut::with_capacity(location.len() + 12);
        self.body.extend_from_slice(b"Location: ");
        self.body.extend_from_slice(location);
    }

    #[inline]
    /// Redirecting the client to a specific url.
    ///
    /// # Parameters
    /// * `location` - The URL to redirect to in [http::Uri].
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    /// use http::{Uri, StatusCode};
    ///
    /// let uri: Uri = "/qwe/qwe".parse().unwrap();
    /// let mut response = Response::new();
    /// response.set_redirect_uri(uri);
    ///
    /// assert_eq!(response.status_code, StatusCode::FOUND);
    /// assert_eq!(response.body.as_ref(), b"Location: /qwe/qwe");
    /// ```
    pub fn set_redirect_uri(&mut self, location: Uri) {
        self.status_code = StatusCode::FOUND;

        let mut len = 12 + location.path().len();

        if let Some(scheme) = location.scheme() {
            len += scheme.as_str().len() + 3;
        }
        if let Some(authority) = location.authority() {
            len += authority.as_str().len();
        }
        if let Some(query) = location.query() {
            len += query.len() + 1;
        }

        self.body = BytesMut::with_capacity(len);
        self.body.extend_from_slice(b"Location: ");

        if let Some(scheme) = location.scheme() {
            self.body.extend_from_slice(scheme.as_str().as_bytes());
            self.body.extend_from_slice(b"://");
        }
        if let Some(authority) = location.authority() {
            self.body.extend_from_slice(authority.as_str().as_bytes());
        }

        self.body.extend_from_slice(location.path().as_bytes());

        if let Some(query) = location.query() {
            self.body.extend_from_slice(b"?");
            self.body.extend_from_slice(query.as_bytes());
        }
    }

    #[inline]
    /// Writing a file to [Response].
    ///
    /// # Parameters
    /// * `file_path` - Path to the file.
    /// * `type_file` - Content type of the file.
    ///
    /// # Examples
    /// ```no_run
    /// use rust_tcp_sever::Response;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut response = Response::new();
    ///     response.set_file("path", "html/text").await.unwrap();
    /// }
    /// ```
    pub async fn set_file<Q: AsRef<Path>, W: AsRef<[u8]>>(
        &mut self,
        file_path: Q,
        type_file: W,
    ) -> Result<(), ServerError> {
        let metadata = fs::metadata(&file_path)
            .await
            .map_err(ServerError::OpeningFile)?;
        if !metadata.is_file() {
            return Err(ServerError::FolderInsteadFile);
        }

        let mut file = File::open(file_path)
            .await
            .map_err(ServerError::OpeningFile)?;

        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer)
            .await
            .map_err(ServerError::Read)?;

        self.status_code = StatusCode::OK;
        self.body = BytesMut::from(buffer.as_slice());
        self.add_header("Content-Type", type_file);
        Ok(())
    }
}

/// Functions to change Cookies and HTTP Headers.
impl Response {
    #[inline]
    /// Add a cookie.
    ///
    /// # Parameters
    /// * `name` - The cookie name.
    /// * `value` - The cookie value.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    ///
    /// let mut response = Response::new();
    /// response.add_cookie("Name", "Value");
    ///
    /// assert_eq!(response.cookies.as_ref(), b"Set-Cookie: Name=Value\r\n");
    /// ```
    pub fn add_cookie<Q: AsRef<[u8]>, W: AsRef<[u8]>>(&mut self, name: Q, value: W) {
        self.cookies.extend_from_slice(b"Set-Cookie: ");
        self.cookies.extend_from_slice(name.as_ref());
        self.cookies.extend_from_slice(b"=");
        self.cookies.extend_from_slice(value.as_ref());
        self.cookies.extend_from_slice(b"\r\n");
    }

    #[inline]
    /// Delete a cookie.
    ///
    /// # Parameters
    /// * `name` - The cookie name to delete.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    ///
    /// let mut response = Response::new();
    /// response.delete_cookie("Name");
    ///
    /// assert_eq!(
    ///     response.cookies.as_ref(),
    ///     b"Set-Cookie: Name=; Expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n"
    /// );
    /// ```
    pub fn delete_cookie<Q: AsRef<[u8]>>(&mut self, name: Q) {
        self.cookies.extend_from_slice(b"Set-Cookie: ");
        self.cookies.extend_from_slice(name.as_ref());
        self.cookies
            .extend_from_slice(b"=; Expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n");
    }

    #[inline]
    /// Add a HTTP header.
    ///
    /// # Parameters
    /// * `name` - The header name.
    /// * `value` - The header value.
    ///
    /// # Examples
    /// ```
    /// use rust_tcp_sever::Response;
    ///
    /// let mut response = Response::new();
    /// response.add_header("Name", "Value");
    ///
    /// assert_eq!(response.headers.as_ref(), b"Name: Value\r\n");
    /// ```
    pub fn add_header<Q: AsRef<[u8]>, W: AsRef<[u8]>>(&mut self, name: Q, value: W) {
        self.headers.extend_from_slice(name.as_ref());
        self.headers.extend_from_slice(b": ");
        self.headers.extend_from_slice(value.as_ref());
        self.headers.extend_from_slice(b"\r\n");
    }
}
