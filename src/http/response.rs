use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Response
pub struct Response {
    /// HTTP status code.
    pub status_code: String,
    /// Response data.
    pub binary_data: Vec<u8>,
    /// Cookies.
    pub cookie: Cookies,
    /// SettingResponse.
    pub setting: SettingResponse,
}

lazy_static! {
    /// HTTP status code 404.
    static ref HTTP_404: String = String::from("404 NOT FOUND");
    /// HTTP status code 302.
    static ref HTTP_302: String = String::from("302 FOUND");

    /// [Response] instance to copy and modify.
    pub static ref RESPONSE_DEF: Response = Response {
        status_code: HTTP_404.clone(),
        binary_data: Vec::new(),

        cookie: Cookies::const_new(),
        setting: SettingResponse::const_new(),
    };
}

/// Function for creating [Response] and converting it into HTTP Response.
impl Response {
    #[inline]
    /// Creating a new instance of a [Response] from a function.
    /// * fn_edit = Function to change the created Response.
    /// # Examples
    /// ```
    /// Response::new_from_fn(|resp| {
    ///     resp.set_response("200 OK", "123");
    ///     resp.cookie.add("Sample Name", "Sample Text");
    ///     resp.setting.add("Content-Type", "text/html");
    /// });
    /// ```
    pub fn new_from_fn<F: FnOnce(&mut Response)>(fn_edit: F) -> Response {
        let mut response = RESPONSE_DEF.clone();
        fn_edit(&mut response);
        response
    }

    #[inline]
    /// Formatting the [Response] in an HTTP response.
    /// * http = HTTP type.
    /// * status_code = HTTP response status.
    /// * response = [Response] to translate to string.
    pub fn format_arg<W: Display + ?Sized>(status_code: &W, response: &Response) -> String {
        format!("HTTP/1.1 {}\r\n{}{}", status_code, response.cookie.0, response.setting.0)
    }
}

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
        self.set_response("200 OK", "");
        self.binary_data.extend_from_slice(b"<html><head>");
        head(self);
        self.binary_data.extend_from_slice(b"</head><body>");
        body(self);
        self.binary_data.extend_from_slice(b"</body></html>");
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
        self.binary_data.extend_from_slice(data.as_ref());
    }
}

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

        self.binary_data.clear();
        self.binary_data.reserve(data.len() + 4);
        self.binary_data.extend_from_slice(b"\r\n");
        self.binary_data.extend_from_slice(data);
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

        self.binary_data.clear();
        self.binary_data.reserve(location.len() + 12);
        self.binary_data.extend_from_slice(b"Location: ");
        self.binary_data.extend_from_slice(location);
    }
}

/// Working with files.
impl Response {
    #[inline]
    /// Creating a new [Response] from files.
    /// If the file cannot be opened or read, the status_code will be written "404 NOT FOUND".
    /// * file_path = Path to file.
    /// * type_file = File type.
    /// # Examples
    /// ```
    /// Response::new_from_file("/test_path", "text/html");
    /// ```
    pub fn new_from_file<Q: AsRef<Path>, W: Display>(file_path: Q, type_file: W) -> Response {
        let mut response = RESPONSE_DEF.clone();
        response.set_file(file_path, type_file);
        response
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
                self.setting.add("Content-Type", type_file);

                return;
            }
        }

        self.status_code = HTTP_404.clone();
    }
}

//

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Cookies.
pub struct Cookies(pub String);

/// Functions for creating and changing [Cookies].
impl Cookies {
    /// Creating New [Cookies].
    /// # Examples
    /// ```
    /// Cookies::const_new();
    /// ```
    #[inline]
    pub const fn const_new() -> Self {
        Cookies(String::new())
    }

    #[inline]
    /// Adding cookies.
    /// * name = Cookie name.
    /// * value = Cookie value.
    /// # Examples
    /// ```
    /// let mut cookies = Cookies::const_new();
    /// cookies.add("testName", "testVale");
    /// ```
    pub fn add<Q: Display, W: Display>(&mut self, name: Q, value: W) {
        self.0
            .push_str(&format!("Set-Cookie: {}={}\r\n", name, value));
    }

    #[inline]
    /// Deleting cookies.
    /// * name = Cookie name.
    /// # Examples
    /// ```
    /// let mut cookies = Cookies::const_new();
    /// cookies.delete("testName");
    /// ```
    pub fn delete<Q: Display>(&mut self, name: Q) {
        self.0.push_str(&format!(
            "Set-Cookie: {}=; Expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n",
            name
        ));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// [Response] Settings.
pub struct SettingResponse(pub String);

/// Functions for creating and changing [SettingResponse];
impl SettingResponse {
    /// Creating New [SettingResponse].
    /// # Examples
    /// ```
    /// SettingResponse::const_new();
    /// ```
    #[inline]
    pub const fn const_new() -> Self {
        SettingResponse(String::new())
    }

    #[inline]
    /// Adding SettingResponse.
    /// * name = Setting name.
    /// * value = Setting value.
    /// # Examples
    /// ```
    /// let mut setting = SettingResponse::const_new();
    /// setting.add("testName", "testValue");
    /// ```
    pub fn add<Q: Display, W: Display>(&mut self, name: Q, value: W) {
        self.0.push_str(&format!("{}: {}\r\n", name, value));
    }
}
