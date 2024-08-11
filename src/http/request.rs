use std::collections::HashMap;

#[derive(Debug, Clone)]
/// Request Structure
pub struct Request {
    /// Metod Request (For example: GET, POST, PUT).
    pub metod: String,
    /// Url Request (For example: /sign, /find/qwe).
    pub url: String,
    /// Type Http (HTTP/1.0, HTTP/1.1, HTTP/2.0).
    pub http: String,

    /// Cookies Files. For edit Cookies files, used Response, Not request!
    pub cookie: HashMap<String, String>,
    /// Add Contents. When your site requests the code, the information goes here.
    pub add_content: HashMap<String, String>,
    /// Add Contents which Don't Parsed to add_content. You're parsing it.
    /// (For example: JSON, TOML). But if you don't send anything, there will be garbage here.
    pub rest_content: String,
}

/// Function for Format Structure into Http.
impl Request {
    /// Formated Structure into Http.
    pub fn format(&self) -> String {
        let request_line = format!("{} {} {}\r\n", self.metod, self.url, self.http);
        
        let mut headers = String::new();

        if !self.cookie.is_empty() {
            let cookie_header = self.cookie.iter()
                .map(|(key, value)| format!("{}={}", key, value))
                .collect::<Vec<_>>()
                .join("; ");
            headers.push_str(&format!("Cookie: {}\r\n", cookie_header));
        }

        for (key, value) in &self.add_content {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }

        format!("{}{}{}", request_line, headers, self.rest_content)
    }
}

/// Functions for Parsed Http into Structure.
impl Request {
    #[inline]
    /// Main Function Parsed. Used null, uncertain and last Line Request.
    /// * data = Http Request. \n
    /// * Null = Metod, Url, Http.
    /// * Uncertain = Parsed, If code find Cookies Line, else Empty.
    /// * Last = Parsed, If Line Not have Cookies, else Empty.
    pub fn parse_to_self(data: &str) -> Option<Request> {
        let mut cookie = HashMap::new();
        let mut add_content = HashMap::new();

        let split_line: Vec<&str> = data.lines().collect();

        if let Some(cookie_line) = split_line.iter().find(|line| line.starts_with("Cookie: ")) {
            cookie = Self::get_data(cookie_line.trim_start_matches("Cookie: "), "; ");
        }

        let last_line = split_line.last()?;
        if let None = last_line.find(": ") {
            add_content = Self::get_data(last_line, "&");
        }

        let muh: Vec<&str> = split_line.get(0)?.split_whitespace().collect();
        let mut url_line = *muh.get(1)?;

        if let Some(index) = url_line.find("?") {
            add_content = Self::get_data(&url_line[(index + 1)..], "&");
            url_line = &url_line[..index];
        }

        Some(Request {
            metod: String::from(*muh.get(0)?),
            url: String::from(url_line),
            http: String::from(*muh.get(2)?),

            cookie,
            add_content,
            rest_content: String::from(*last_line),
        })
    }

    #[inline]
    /// Function for parse Line into HashMap.
    /// * data = Line Parsed.
    /// * char_split = Char used for Split Line.
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
