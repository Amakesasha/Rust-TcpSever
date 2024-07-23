use crate::*;
use std::{fmt::Display, fs::File, io::Read};

#[derive(Debug, Clone, Default)]
/// Response Structure
pub struct Response {
    /// Status Response (For example: 404 NOT FOUND, 302 FOUND, 200 OK).
    pub status_code: String,
    /// Data Response (For example: HTML/CSS file, Json code).
    pub data: String,
    /// Cookies Files, Write into structure for easy development.
    pub cookie: Cookie,
    /// Add Setting Response (For example: Content-Type, Data).
    pub setting: SettingResponse,
}

/// Functions: Make a new structure and Write structure into Line.
impl Response {
    #[inline]
    /// Make a New Structure.
    pub fn new() -> Response {
        Response {
            status_code: String::from("404 NOT FOUND"),
            data: String::new(),

            cookie: Cookie::new(),
            setting: SettingResponse::new(),
        }
    }

    #[inline]
    /// Make a New Default Structure.
    pub const fn const_new() -> Response {
        Response {
            status_code: String::new(),
            data: String::new(),

            cookie: Cookie::new(),
            setting: SettingResponse::new(),
        }
    }

    #[inline]
    /// Write structure into Line.
    /// * http = Type Http. You can used &str or String.
    pub fn format<Q: Display>(&self, http: Q) -> String {
        if self.status_code == String::from("404 NOT FOUND") || self.status_code.is_empty() {
            return unsafe { Self::format_arg(&http, "200 OK", &DEF_PAGE) };
        }

        return Self::format_arg(&http, &self.status_code, self);
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
            http, status_code, response.cookie.0, response.setting.0, response.data,
        )
    }
}

/// Functions for edit Response.
impl Response {
    #[inline]
    /// Set Response. You can used &str or String.
    /// * status = Status Response.
    /// * data = Write Data.
    pub fn set_response<Q: Display, W: Display>(&mut self, status: Q, data: W) {
        self.status_code = status.to_string();
        self.data = format!("\r\n\r\n{}", data);
    }

    #[inline]
    /// Redirect client. You can used &str or String.
    /// Don't used "Content-Type" with this!
    /// * location = Redirect Url.
    pub fn set_redirect<Q: Display>(&mut self, location: Q) {
        self.status_code = "302 FOUND".to_string();
        self.data = format!("Location: {}", location);
    }

    #[inline]
    /// Read File and Write it Client. If don't open file, status code will set 404 NOT FOUND.
    /// You can used &str or String.
    /// * name_file = Name Readed File.
    pub fn set_file<Q: Display + std::convert::AsRef<std::path::Path>>(&mut self, name_file: Q) {
        if let Ok(mut file) = File::open(name_file) {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    self.set_response("200 OK", contents);

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
