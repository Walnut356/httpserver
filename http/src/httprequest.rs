use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninit,
}

impl Default for Method {
    fn default() -> Self {
        Self::Uninit
    }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Uninit,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninit,
}

impl Default for Version {
    fn default() -> Self {
        Self::Uninit
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::V1_1,
            "POST" => Self::V2_0,
            _ => Self::Uninit,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

impl Default for Resource {
    fn default() -> Self {
        Self::Path("".to_string())
    }
}

#[derive(Debug, Default)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<&str> for HttpRequest {
    fn from(request: &str) -> Self {
        let mut result = Self::default();

        for line in request.lines() {
            if line.contains("HTTP") {
                todo!();
            } else if line.contains(':') {
                let (key, value) = Self::header_line(line);
                result.headers.insert(key, value);
            } else if line.is_empty() {
                let (method, res, vers) = Self::request_line(line);
                result.method = method;
                result.resource = res;
                result.version = vers;
            } else {
                todo!();
            }
        }

        result
    }
}

impl HttpRequest {
    fn header_line(s: &str) -> (String, String) {
        let mut header_items = s.split(':');
        (header_items.next().unwrap().to_string(), header_items.next().unwrap().to_string())
    }

    fn request_line(s: &str) -> (Method, Resource, Version) {
        let mut words = s.split_whitespace();
        let method = words.next().unwrap().into();
        let resource = Resource::Path(words.next().unwrap().to_string());
        let version = words.next().unwrap().into();


        (method, resource, version)
    }
}