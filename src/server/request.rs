use std::collections::HashMap;

#[derive(Debug, Clone)]
/// Request Structure
pub struct Request {
    /// Vector Length = 3.
    /// * 0 = Metod Request (For example: GET, POST, PUT).
    /// * 1 = Url Request (For example: /sign, /find/qwe).
    /// * 2 = Type Http (For example: HTTP/1.1, HTTP/2.0).
    pub metod_url_http: Vec<String>,
    /// Cookies Files. For edit Cookies files, used Response, Not request!
    pub cookie: HashMap<String, String>,
    /// Add Contents. When your site requests the code, the information goes here.
    pub add_content: HashMap<String, String>,
}

/// Functions for Parsed Http into Structure.
impl Request {
    #[inline]
    /// Main Function Parsed. Used null, uncertain and last Line Request.
    /// * data = Http Request. \n
    /// * Null = Metod, Url, Http.
    /// * Uncertain = Parsed, If code find Cookies Line, else Empty.
    /// * Last = Parsed, If Line Not have Cookies, else Empty.
    pub fn parse_to_self(data: &str) -> Request {
        let mut cookie = HashMap::new();
        let mut add_content = HashMap::new();

        let split_line: Vec<&str> = data.lines().collect();

        if let Some(cookie_line) = split_line.iter().find(|line| line.starts_with("Cookie: ")) {
            cookie = Self::get_data(cookie_line.trim_start_matches("Cookie: "), "; ");
        }

        let last_line = split_line.last().unwrap_or(&"");
        if !last_line.starts_with("Cookie: ") && !last_line.starts_with("Accept-Language: ") {
            add_content = Self::get_data(last_line, "&");
        }

        Request {
            metod_url_http: split_line[0].split_whitespace().map(String::from).collect(),
            cookie,
            add_content,
        }
    }

    #[inline]
    /// Function for parse Line into HashMap.
    /// * data = Line Parsed.
    /// * char_split = Char used for Split Line.
    fn get_data(data: &str, char_split: &str) -> HashMap<String, String> {
        data.split(char_split)
            .filter_map(|part| {
                if let [key, rest @ ..] = &part.split('=').collect::<Vec<&str>>()[..] {
                    Some((key.trim().to_string(), rest.join("=").trim().to_string()))
                } else {
                    None
                }
            })
            .collect()
    }
}
