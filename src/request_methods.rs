use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    HEAD,
    DELETE,
    PUT,
    PATCH,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::HEAD => write!(f, "HEAD"),
            Method::DELETE => write!(f, "DELETE"),
            Method::PUT => write!(f, "PUT"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
}
