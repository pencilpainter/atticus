use crate::auth_methods::AuthTypes;
use crate::request_methods::Method;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub variables: im::Vector<(String, String)>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub name: String,
    pub headers: im::Vector<(String, String)>,
    pub url: String,
    pub method: Method,
    pub body: String,
    pub auth: (AuthTypes, String),
}
