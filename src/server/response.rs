use crate::*;
use std::{fmt::Display, fs::File, io::Read};

#[derive(Debug, Clone, Default)]
/// Response Structure
pub struct Response {
    /// Status Response (For example: 404 NOT FOUND, 302 FOUND, 200 OK).
    pub status_code: String,
    /// String Data Response (For example: HTML/CSS file, Json code).
    pub string_data: String,
    /// Binary Data Response (For example: Video, Music, Image).
    pub binary_data: Vec<u8>,
    /// Cookies Files, Write into structure for easy development.
    pub cookie: Cookie,
    /// Add Setting Response (For example: Content-Type, Data).
    pub setting: SettingResponse,
}

/// Functions: Make a new structure and Write structure into Line.
impl Response {
    #[inline]
    /// Make a New Default Structure.
    pub const fn const_new() -> Response {
        Response {
            status_code: String::new(),
            string_data: String::new(),

            binary_data: Vec::new(),

            cookie: Cookie::new(),
            setting: SettingResponse::new(),
        }
    }

    #[inline]
    /// Write structure into Line.
    /// * http = Type Http. You can used &str or String.
    pub fn format<Q: Display>(&self, http: Q) -> (String, bool) {
        if self.status_code == String::from("404 NOT FOUND") || self.status_code.is_empty() {
            return (
                Self::format_arg(&http, "200 OK", unsafe { &DEF_PAGE }),
                false,
            );
        }

        return (Self::format_arg(&http, &self.status_code, self), true);
    }

    #[inline]
    /// Parse Structure into Line. You don't used this function, but you can.
    /// * http = Type Http.
    /// * response = Response which will into Line.
    pub fn format_arg<Q: Display, W: Display + ?Sized>(
        http: &Q,
        status_code: &W,
        response: &Response,
    ) -> String {
        format!(
            "{} {}\r\n{}{}{}",
            http, status_code, response.cookie.0, response.setting.0, response.string_data
        )
    }
}

/// Functions for edit Response.
impl Response {
    #[inline]
    /// Set Response. You can used &str or String.
    /// * status = Status Response.
    /// * data = Write Data.
    pub fn set_response<Q: Display, W: Display>(&mut self, status: Q, string_data: W) {
        self.status_code = status.to_string();
        self.string_data = format!("\r\n{}", string_data);
    }

    #[inline]
    /// Redirect client. You can used &str or String.
    /// Don't used "Content-Type" with this!
    /// * location = Redirect Url.
    pub fn set_redirect<Q: Display>(&mut self, location: Q) {
        self.status_code = "302 FOUND".to_string();
        self.string_data = format!("Location: {}", location);
    }
}

/// Set and Make Response From Files
impl Response {
    #[inline]
    /// Make a New Response from File. If don't open File, status code will set 404 NOT FOUND.
    /// You can used &str or String.
    /// * file_path = Path to File.
    /// * type_file = Type File (For example: image/png, video/mp4).
    pub fn new_from_file<Q, W>(file_path: Q, type_file: W) -> Response
    where
        Q: std::convert::AsRef<std::path::Path>,
        W: Display,
    {
        let mut response = Response::const_new();
        response.set_file(file_path, type_file);
        return response;
    }

    #[inline]
    /// Open File, Read, after Write to Client. If don't open file, status code will set 404 NOT FOUND.
    /// You can used &str or String.
    /// * file_path = Path to File.
    /// * type_file = Type File (For example: image/png, video/mp4).
    pub fn set_file<Q, W>(&mut self, file_path: Q, type_file: W)
    where
        Q: std::convert::AsRef<std::path::Path>,
        W: Display,
    {
        if let Ok(mut file) = File::open(file_path) {
            let mut buffer = Vec::new();

            match file.read_to_end(&mut buffer) {
                Ok(_) => {
                    self.set_response("200 OK", "");
                    self.binary_data = buffer;
                    self.setting.add("Content-Type", type_file);

                    return;
                }
                Err(_) => {}
            }
        }

        self.status_code = "404 NOT FOUND".to_string();
    }
}

//

#[derive(Debug, Clone, Default)]
/// Cookies Files.
pub struct Cookie(pub String);

/// Functions Make and Edit Cookies.
impl Cookie {
    /// Make a new Cookies Files.
    #[inline]
    pub const fn new() -> Self {
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

#[derive(Debug, Clone, Default)]
/// Setting Response.
pub struct SettingResponse(pub String);

/// Functions Make and Edit Setting Response.
impl SettingResponse {
    /// Make a new Setting Response.
    #[inline]
    pub const fn new() -> Self {
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
