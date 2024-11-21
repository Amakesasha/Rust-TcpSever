use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Response.
pub struct Response {
    /// HTTP status code.
    pub status_code: String,
    /// Response body.
    pub body: Vec<u8>,
    /// Response Cookies.
    pub cookie: ResponseCookies,
    /// Response Headers.
    pub setting: ResponseHeaders,
}

/// HTTP status code 404.
static HTTP_404: Lazy<String> = Lazy::new(|| String::from("404 NOT FOUND"));
/// HTTP status code 302.
static HTTP_302: Lazy<String> = Lazy::new(|| String::from("302 FOUND"));
/// HTTP next line.
static HTTP_NEXT_LINE: Lazy<Vec<u8>> = Lazy::new(|| b"\r\n".to_vec());

/// [Response] instance to copy and modify.
pub static RESPONSE_DEF: Lazy<Response> = Lazy::new(|| Response {
    status_code: HTTP_404.clone(),
    body: Vec::new(),

    cookie: ResponseCookies::default(),
    setting: ResponseHeaders::default(),
});

impl Display for Response {
    /// Function for converting [Response] into HTTP Response.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {}\r\n{}{}",
            self.status_code, self.cookie.0, self.setting.0
        )
    }
}

impl<F: FnOnce(&mut Response)> From<F> for Response {
    #[inline]
    /// Creating a new instance of a [Response] from a function.
    /// # Examples
    /// ```
    /// Response::from(|resp: &mut Response| resp.set_response("200 OK", "wer"));
    /// ```
    fn from(fn_edit: F) -> Self {
        let mut response = RESPONSE_DEF.clone();
        fn_edit(&mut response);
        response
    }
}

impl<P: AsRef<Path>, D: Display> From<(P, D)> for Response {
    #[inline]
    /// Creating a new [Response] from files.
    /// If the file cannot be opened or read, the status_code will be written "404 NOT FOUND".
    /// # Examples
    /// ```
    /// Response::from(("/test_path", "text/html"));
    /// ```
    fn from((file_path, type_file): (P, D)) -> Self {
        let mut response = RESPONSE_DEF.clone();
        response.set_file(file_path, type_file);
        response
    }
}

//

/// HTML file builder.
impl Response {
    #[inline]
    /// Function to run [Response::echo].
    /// * head = Function to create HEAD HTML.
    /// * body = Function to create BODY HTML.
    /// # Examples
    /// ```
    /// let mut response = RESPONSE_DEF.clone();
    /// response.html(
    ///     |resp| resp.echo("Example Head"),
    ///     |resp| resp.echo("Example Body");,
    /// );
    /// ```
    pub fn html<Q: FnOnce(&mut Response), W: FnOnce(&mut Response)>(&mut self, head: Q, body: W) {
        self.status_code = HTTP_404.clone();
        self.body = HTTP_NEXT_LINE.clone();

        self.body.extend_from_slice(b"<html><head>");
        head(self);
        self.body.extend_from_slice(b"</head><body>");
        body(self);
        self.body.extend_from_slice(b"</body></html>");
    }

    #[inline]
    /// Adding a line to html. If you use outside [Response::html],
    /// run self.set_response("200 OK", ""); before using.
    /// * data = Line to add.
    /// # Examples
    /// ```
    /// let mut response = RESPONSE_DEF.clone();
    /// response.html(
    ///     |resp| resp.echo("Example Head"),
    ///     |resp| resp.echo("Example Body");,
    /// );
    /// ```
    pub fn echo<Q: AsRef<[u8]>>(&mut self, data: Q) {
        self.body.extend_from_slice(data.as_ref());
    }
}

//

/// Functions to change [Response].
impl Response {
    #[inline]
    /// Inserts HTTP code status and data into Response.
    /// * status = HTTP code status.
    /// * data = Recorded data.
    /// # Examples
    /// ```
    /// let mut response = RESPONSE_DEF.clone();
    /// response.set_response("200 OK", "All good");
    /// ```
    pub fn set_response<Q, W: AsRef<[u8]>>(&mut self, status: Q, string_data: W)
    where
        String: From<Q>,
    {
        self.status_code = String::from(status);

        let data = string_data.as_ref();

        self.body.clear();
        self.body.reserve(data.len() + 4);
        self.body.extend_from_slice(b"\r\n");
        self.body.extend_from_slice(data);
    }

    #[inline]
    /// Redirecting the client to a specific url.
    /// * location = Redirect url.
    /// # Examples
    /// ```
    /// let mut response = RESPONSE_DEF.clone();
    /// response.set_redirect("/test_url");
    /// ```
    pub fn set_redirect<Q: AsRef<[u8]>>(&mut self, location: Q) {
        self.status_code = HTTP_302.clone();

        let location = location.as_ref();

        self.body.clear();
        self.body.reserve(location.len() + 12);
        self.body.extend_from_slice(b"Location: ");
        self.body.extend_from_slice(location);
    }

    #[inline]
    /// Writing a file to [Response].
    /// If the file cannot be opened or read, the status_code will be written "404 NOT FOUND".=
    /// * file_path = Path to file.
    /// * type_file = File type.
    /// # Examples
    /// ```
    /// let mut response = RESPONSE_DEF.clone();
    /// response.set_file("/test_path", "text/html");
    /// ```
    pub fn set_file<Q: AsRef<Path>, W: Display>(&mut self, file_path: Q, type_file: W) {
        if let Ok(mut file) = File::open(file_path) {
            let mut buffer = Vec::new();

            if file.read_to_end(&mut buffer).is_ok() {
                self.set_response("200 OK", buffer);
                self.setting += ("Content-Type", type_file);

                return;
            }
        }

        self.status_code = HTTP_404.clone();
    }
}

//

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// ResponseCookies.
pub struct ResponseCookies(pub String);

impl<Q: Display, W: Display> AddAssign<(Q, W)> for ResponseCookies {
    #[inline]
    /// Adding Responsecookies.
    /// # Examples
    /// ```
    /// let mut cookies = ResponseCookies::default();
    /// cookies += ("testName", "testVale");
    /// ```
    fn add_assign(&mut self, (name, value): (Q, W)) {
        self.0 += &format!("Set-Cookie: {name}={value}\r\n");
    }
}

impl<Q: Display> SubAssign<Q> for ResponseCookies {
    #[inline]
    /// Deleting Responsecookies.
    /// # Examples
    /// ```
    /// let mut cookies = ResponseCookies::default();
    /// cookies -= "testName";
    /// ```
    fn sub_assign(&mut self, name: Q) {
        self.0 += &format!("Set-Cookie: {name}=; Expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// [Response] Settings.
pub struct ResponseHeaders(pub String);

impl<Q: Display, W: Display> AddAssign<(Q, W)> for ResponseHeaders {
    #[inline]
    /// Adding ResponseHeaders.
    /// # Examples
    /// ```
    /// let mut headers = ResponseHeaders::default();
    /// headers += ("testName", "testValue");
    /// ```
    fn add_assign(&mut self, (name, value): (Q, W)) {
        self.0 += &format!("{name}: {value}\r\n");
    }
}
