use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Response Structure
pub struct Response {
    /// Status Response (For example: 404 NOT FOUND, 302 FOUND, 200 OK).
    pub status_code: String,
    /// Data Response.
    pub binary_data: Vec<u8>,
    /// Cookies Files, Write into structure for easy development.
    pub cookie: Cookie,
    /// Add Setting Response (For example: Content-Type, Data).
    pub setting: SettingResponse,
}

lazy_static! {
    static ref HTTP_404: String = String::from("404 NOT FOUND");
    static ref HTTP_302: String = String::from("302 FOUND");

    // Default Response, clone for Edit.
    pub static ref RESPONSE_DEF: Response = Response {
        status_code: HTTP_404.clone(),
        binary_data: Vec::new(),

        cookie: Cookie::const_new(),
        setting: SettingResponse::const_new(),
    };
}

/// Functions: Make from Function and Write Response into Line.
impl Response {
    #[inline]
    /// Make a New Structure from Function.
    /// * fn_edit = Function for Edit Structure.
    pub fn new_from_fn<F: FnOnce(&mut Response)>(fn_edit: F) -> Response {
        let mut response = RESPONSE_DEF.clone();
        fn_edit(&mut response);
        return response;
    }

    #[inline]
    /// Parse Structure into Line. You don't used this function, but you can.
    /// * status_code = Status Response (For example: 404 NOT FOUND, 302 FOUND, 200 OK).
    /// * response = Response which Will into Line.
    pub fn format_arg<W: Display + ?Sized>(status_code: &W, response: &Response) -> String {
        format!(
            "{} {}\r\n{}{}",
            unsafe { TYPE_HTTP },
            status_code,
            response.cookie.0,
            response.setting.0,
        )
    }
}

/// Functions from edit Html.
impl Response {
    #[inline]
    /// Function for Working with Response::echo().
    /// * head = Function on Creating the Html part of the Head.
    /// * body = Function on Creating the Html part of the Body.
    pub fn html<Q: FnOnce(&mut Response), W: FnOnce(&mut Response)>(&mut self, head: Q, body: W) {
        self.set_response("200 OK", "");
        self.binary_data.extend_from_slice(b"<html><head>");
        head(self);
        self.binary_data.extend_from_slice(b"</head><body>");
        body(self);
        self.binary_data.extend_from_slice(b"</body></html>");
    }

    #[inline]
    /// Adding a String to Html. Don't Use this Outside of Response::html().
    /// * data = Data for Add.
    pub fn echo<Q: AsRef<[u8]>>(&mut self, data: Q) {
        self.binary_data.extend_from_slice(data.as_ref());
    }
}

/// Functions for edit Response.
impl Response {
    #[inline]
    /// Set Response. You can used &str or String.
    /// * status = Status Response.
    /// * data = Write Data.
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
    /// Redirect client. You can used &str or String.
    /// Don't used "Content-Type" with this!
    /// * location = Redirect Url.
    pub fn set_redirect<Q: AsRef<[u8]>>(&mut self, location: Q) {
        self.status_code = HTTP_302.clone();

        let location = location.as_ref();

        self.binary_data.clear();
        self.binary_data.reserve(location.len() + 12);
        self.binary_data.extend_from_slice(b"Location: ");
        self.binary_data.extend_from_slice(location);
    }
}

/// Set and Make Response From Files
impl Response {
    #[inline]
    /// Make a New Response from File. If don't open File, status code will set 404 NOT FOUND.
    /// You can used &str or String.
    /// * file_path = Path to File.
    /// * type_file = Type File (For example: image/png, video/mp4).
    pub fn new_from_file<Q: AsRef<Path>, W: Display>(file_path: Q, type_file: W) -> Response {
        let mut response = RESPONSE_DEF.clone();
        response.set_file(file_path, type_file);
        return response;
    }

    #[inline]
    /// Open File, Read, after Write to Client. If don't open file, status code will set 404 NOT FOUND.
    /// You can used &str or String.
    /// * file_path = Path to File.
    /// * type_file = Type File (For example: image/png, video/mp4).
    pub fn set_file<Q: AsRef<Path>, W: Display>(&mut self, file_path: Q, type_file: W) {
        if let Ok(mut file) = File::open(file_path) {
            let mut buffer = Vec::new();

            match file.read_to_end(&mut buffer) {
                Ok(_) => {
                    self.set_response("200 OK", buffer);
                    self.setting.add("Content-Type", type_file);

                    return;
                }
                Err(_) => {}
            }
        }

        self.status_code = HTTP_404.clone();
    }
}

//

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Cookies Files.
pub struct Cookie(pub String);

/// Functions Make and Edit Cookies.
impl Cookie {
    /// Make a new Cookies Files.
    #[inline]
    pub const fn const_new() -> Self {
        Cookie { 0: String::new() }
    }

    #[inline]
    /// Addition Cookie. You can used &str or String.
    /// At Set the cookie Value, then Set the cookie other Value, will be done last action.
    /// * name = Name Cookie.
    /// * value = Name Cookie
    pub fn add<Q: Display, W: Display>(&mut self, name: Q, value: W) {
        self.0
            .push_str(&format!("Set-Cookie: {}={}\r\n", name, value));
    }

    #[inline]
    /// Delete Cookie. You can used &str or String.
    /// At add the cookie, then delete the cookie, will be done last action.
    /// * name = Name Cookie.
    pub fn delete<Q: Display>(&mut self, name: Q) {
        self.0.push_str(&format!(
            "Set-Cookie: {}=; Expires=Thu, 01 Jan 1970 00:00:00 GMT\r\n",
            name
        ));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Setting Response.
pub struct SettingResponse(pub String);

/// Functions Make and Edit Setting Response.
impl SettingResponse {
    /// Make a new Setting Response.
    #[inline]
    pub const fn const_new() -> Self {
        SettingResponse { 0: String::new() }
    }

    #[inline]
    /// Addition Setting Response. You can used &str or String.
    /// * name = Name Setting.
    /// * value = Name Setting
    pub fn add<Q: Display, W: Display>(&mut self, name: Q, value: W) {
        self.0.push_str(&format!("{}: {}\r\n", name, value));
    }
}
