use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Request Structure
pub struct Request {
    /// Metod Request (For example: GET, POST, PUT).
    pub method: String,
    /// Url Request (For example: /sign, /find/qwe).
    pub url: String,
    /// Type Http (HTTP/1.0, HTTP/1.1, HTTP/2.0).
    pub http: String,

    /// Cookies Files. For edit Cookies files, used Response, Not request!
    pub cookie: HashMap<String, String>,
    /// Add Contents. When your site requests the code, the information goes here.
    pub add_content: HashMap<String, String>,
    /// Add Contents which Don't Parsed to add_content. You're parsing it.
    /// (For example: JSON File). But if you don't send anything, there will be garbage here.
    pub last_line: String,
}

/// Functions for Parsed Http into Structure.
impl Request {
    #[inline]
    /// Main Function Parsed.
    /// * data = Http Request.
    pub fn parse_to_self(data: &str) -> Option<Request> {
        let mut cookie = HashMap::new();
        let mut add_content = HashMap::new();

        let mut split_line = data.lines();

        let muh: Vec<&str> = split_line.next()?.split_whitespace().collect();
        let mut url_line = *muh.get(1)?;

        if let Some(index) = url_line.find("?") {
            add_content = Self::get_data(&url_line[(index + 1)..], "&");
            url_line = &url_line[..index];
        }

        let last_line = split_line.next_back().unwrap();
        if !last_line.contains(": ") {
            add_content = Self::get_data(last_line, "&");
        }

        if let Some(cookie_line) = split_line.find(|line| line.starts_with("Cookie: ")) {
            cookie = Self::get_data(cookie_line.trim_start_matches("Cookie: "), "; ");
        } else if last_line.contains("Cookie: ") {
            cookie = Self::get_data(last_line.trim_start_matches("Cookie: "), "; ");
        }

        Some(Request {
            method: String::from(*muh.get(0)?),
            url: String::from(url_line),
            http: String::from(*muh.get(2)?),

            cookie,
            add_content,
            last_line: String::from(last_line),
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
