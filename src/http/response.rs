use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Response.
pub struct Response {
    /// HTTP status code.
    pub status_code: Vec<u8>,
    /// Response body.
    pub body: BytesMut,
    /// Response Cookies.
    pub cookies: BytesMut,
    /// Response Headers.
    pub headers: BytesMut,
}

static HTTP_404: Lazy<Vec<u8>> = Lazy::new(|| b"404 NOT FOUND".to_vec());
static HTTP_302: Lazy<Vec<u8>> = Lazy::new(|| b"302 FOUND".to_vec());
static HTTP_200: Lazy<Vec<u8>> = Lazy::new(|| b"200 OK".to_vec());

/// [Response] instance to copy and modify.
pub static RESPONSE_DEF: Lazy<Response> = Lazy::new(|| Response {
    status_code: HTTP_404.clone(),
    ..Response::default()
});

/// Functions for formatting [Response].
impl Response {
    #[inline]
    /// Translation of [Response] into byte code.
    /// # Examples
    /// ```
    /// Response::new().as_bytes();
    /// ```
    pub fn as_bytes(&self) -> Bytes {
        let mut bytes = BytesMut::with_capacity(
            11 + self.status_code.len() + self.cookies.len() + self.headers.len() + self.body.len(),
        );

        bytes.extend_from_slice(b"HTTP/1.1 ");
        bytes.extend_from_slice(&self.status_code);
        bytes.extend_from_slice(b"\r\n");
        bytes.extend_from_slice(&self.cookies);
        bytes.extend_from_slice(&self.headers);
        bytes.extend_from_slice(&self.body);

        bytes.freeze()
    }
}

/// Functions for creating [Response].
impl Response {
    #[inline]
    /// Creating a default instance of a [Response].
    /// # Examples
    /// ```
    /// Response::new();
    /// ```
    pub fn new() -> Self {
        RESPONSE_DEF.clone()
    }

    #[inline]
    /// Creating a new instance of a [Response] from a status and body.
    ///
    /// # Parameters
    /// * `status` - HTTP status code (e.g., "200 OK", "404 Not Found").
    /// * `data` - Response body data.
    ///
    /// # Examples
    /// ```
    /// Response::from_response("200 OK", "All Good :)");
    /// ```
    pub fn from_response<Q: AsRef<[u8]>, W: AsRef<[u8]>>(status: Q, data: W) -> Self {
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
    /// Response::from_fn(|resp| {
    ///     resp.set_response("200 OK", "wer");
    /// });
    /// ```
    /// or
    /// ```
    /// Response::from_fn(qwe);
    ///
    /// fn qwe(resp: &mut Response) {
    ///     resp.set_response("200 OK", "wer");
    /// }
    /// ```
    pub fn from_fn<F: FnOnce(&mut Response)>(fn_edit: F) -> Self {
        let mut response = Response::new();
        fn_edit(&mut response);
        response
    }

    #[inline]
    /// Creating a new instance of a [Response] from construct an HTML.
    ///
    /// # Parameters
    /// * `head` - Function to add content to HTML `<head>`.
    /// * `body` - Function to add content to HTML `<body>`.
    ///
    /// # Examples
    /// ```
    /// Response::from_html(
    ///     |resp| { resp.echo("Example Head"); },
    ///     |resp| { resp.echo("Example Body"); },
    /// );
    /// ```
    /// or
    /// ```
    /// Response::from_html(head, body);
    ///     
    /// fn head(resp: &mut Response) {
    ///     resp.echo("Example Head");
    /// }
    /// fn body(resp: &mut Response) {
    ///     resp.echo("Example Body");
    /// }
    /// ```
    pub fn from_html<Q: FnOnce(&mut Response), W: FnOnce(&mut Response)>(head: Q, body: W) -> Self {
        let mut response = Response::new();
        response.set_html(head, body);
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
    /// ```
    /// Response::from_file("/test_path", "text/html").await.unwrap();
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
    /// * `status` - HTTP status code (e.g., "200 OK", "404 Not Found").
    /// * `data` - Response body data.
    ///
    /// # Examples
    /// ```
    /// let mut response = Response::new();
    /// response.set_response("200 OK", "All good");
    /// ```
    pub fn set_response<Q: AsRef<[u8]>, W: AsRef<[u8]>>(&mut self, status: Q, data: W) {
        self.status_code = status.as_ref().to_vec();

        let data = data.as_ref();

        self.body = BytesMut::with_capacity(data.len() + 4);
        self.body.extend_from_slice(b"\r\n");
        self.body.extend_from_slice(data);
    }

    #[inline]
    /// Redirecting the client to a specific url.
    ///
    /// # Parameters
    /// * `location` - The URL to redirect to.
    ///
    /// # Examples
    /// ```
    /// let mut response = Response::new();
    /// response.set_redirect("/test_url");
    /// ```
    pub fn set_redirect<Q: AsRef<[u8]>>(&mut self, location: Q) {
        self.status_code = HTTP_302.to_vec();

        let location = location.as_ref();

        self.body = BytesMut::with_capacity(location.len() + 12);
        self.body.extend_from_slice(b"Location: ");
        self.body.extend_from_slice(location);
    }

    #[inline]
    /// Writing a file to [Response].
    /// If the file cannot be opened or read, the status_code will be written "404 NOT FOUND".
    ///
    /// # Parameters
    /// * `file_path` - Path to the file.
    /// * `type_file` - Content type of the file.
    ///
    /// # Examples
    /// ```
    /// let mut response = Response::new();
    /// response.set_file("/test_path", "text/html").await.unwrap();
    /// ```
    pub async fn set_file<Q: AsRef<Path>, W: AsRef<[u8]>>(
        &mut self,
        file_path: Q,
        type_file: W,
    ) -> Result<(), ServerError> {
        let metadata = fs::metadata(&file_path)
            .await
            .map_err(ServerError::ErrorOpeningFile)?;
        if !metadata.is_file() {
            return Err(ServerError::FolderInsteadFile);
        }

        let mut file = File::open(file_path)
            .await
            .map_err(ServerError::ErrorOpeningFile)?;
        let mut buffer = BytesMut::with_capacity(metadata.len() as usize);

        file.read_buf(&mut buffer)
            .await
            .map_err(ServerError::ReadError)?;

        self.status_code = HTTP_200.to_vec();
        self.body = buffer;
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
    /// let mut response = Response::new();
    /// response.add_cookie("Name", "Value");
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
    /// let mut response = Response::new();
    /// response.delete_cookie("Name");
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
    /// let mut response = Response::new();
    /// response.set_header("Name", "Value");
    /// ```
    pub fn add_header<Q: AsRef<[u8]>, W: AsRef<[u8]>>(&mut self, name: Q, value: W) {
        self.headers.extend_from_slice(name.as_ref());
        self.headers.extend_from_slice(b": ");
        self.headers.extend_from_slice(value.as_ref());
        self.headers.extend_from_slice(b"\r\n");
    }
}

/// HTML file builder.
impl Response {
    #[inline]
    /// Constructs an HTML response.
    ///
    /// # Parameters
    /// * `head` - Function to add content to HTML `<head>`.
    /// * `body` - Function to add content to HTML `<body>`.
    ///
    /// # Examples
    /// ```
    /// let mut response = Response::new();
    /// response.set_html(
    ///     |resp| { resp.echo("Example Head"); },
    ///     |resp| { resp.echo("Example Body"); },
    /// );
    /// ```
    /// or
    /// ```
    /// let mut response = Response::new();
    /// response.set_html(head, body);
    ///     
    /// fn head(resp: &mut Response) {
    ///     resp.echo("Example Head");
    /// }
    /// fn body(resp: &mut Response) {
    ///     resp.echo("Example Body");
    /// }
    /// ```
    pub fn set_html<Q: FnOnce(&mut Response), W: FnOnce(&mut Response)>(
        &mut self,
        head: Q,
        body: W,
    ) {
        self.set_response(b"200 OK", "");

        self.body.extend_from_slice(b"<html><head>");
        head(self);
        self.body.extend_from_slice(b"</head><body>");
        body(self);
        self.body.extend_from_slice(b"</body></html>");
    }

    #[inline]
    /// Adding a line to html. If you use outside [Response::set_html],
    /// run resp.set_response("200 OK", ""); before using.
    ///
    /// # Parameters
    /// * `data` - The data to be added to the response body.
    ///
    /// # Examples
    /// ```
    /// let mut resp = Response::new();
    /// resp.set_response("200 OK", "");
    /// resp.echo("Example Body");
    /// ```
    pub fn echo<Q: AsRef<[u8]>>(&mut self, data: Q) {
        self.body.extend_from_slice(data.as_ref());
    }
}
