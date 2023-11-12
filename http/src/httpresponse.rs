use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(status_code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>,) -> Self {
        Self {
            version: "HTTP/1.1",
            status_code,
            status_text: match status_code {
                "200" => "OK",
                "400" => "Bad Request",
                "500" => "Internal Server Error",
                _ => "Not Found",
            },
            headers: Some(headers.unwrap_or_else(|| {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                h
            })),
            body,
        }
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let _ = write!(write_stream, "{}", String::from(self));
        Ok(())
    }
}

impl<'a> From<&HttpResponse<'a>> for String {
    fn from(val: &HttpResponse<'a>) -> Self {
        let body = val.body.as_ref().unwrap();
        let header = val.headers.as_ref().unwrap();

        let h_string = {
            let mut result = String::default();
            for (k, v) in header.iter() {
                result.push_str(&format!("{}:{}\r\n", k, v))
            }
            result
        };

        format!("{} {} {}\r\n{} Content-Length: {}\r\n\r\n{}", val.version, val.status_code, val.status_text, h_string, body.len(), body)
    }
}
