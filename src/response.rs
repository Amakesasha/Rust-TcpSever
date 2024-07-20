#[allow(dead_code)]
#[derive(Debug, Default)]
/// Response Structure
pub struct Response {
    /// Status Response (For example: 404 NOT FOUND, 302 FOUND, 200 OK).
    pub status_line: String,
    /// Data Response (For example: HTML/CSS file, Json code).
    pub data: String,
    /// Cookies Files, Write into structure for easy development.
    pub cookie: Cookie,
    /// Add Setting Response (For example: Content-Type, Data).
    pub setting: SettingResponse,
}

use std::fmt::{Display, Debug};

/// Functions: Make a new structure and Write structure into Line.
impl Response {
    #[inline]
    /// Make a new structure.
    pub fn new() -> Response {
        Response {
            status_line: String::from("404 NOT FOUND"),
            data: String::new(),

            cookie: Cookie::new(),
            setting: SettingResponse::new(),
        }
    }

    #[inline]
    /// Write structure into Line.
    /// * http = Type Http. You can used &str or String.
    pub fn format<Q: Display>(&self, http: Q) -> String {
        format!(
            "{} {}\r\n{}{}{}",
            http, self.status_line, self.cookie.0, self.setting.0, self.data,
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
        self.status_line = status.to_string();
        self.data.push_str(&format!("\r\n\r\n{}", data));
    }

    #[inline]
    /// Redirect client. You can used &str or String
    /// Don't used "Content-Type" with this!
    /// * location = Redirect Url.
    pub fn set_redirect<Q: Display>(&mut self, location: Q) {
        self.status_line = "302 FOUND".to_string();
        self.data.push_str(&format!("Location: {}", location));
    }
}

//

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
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