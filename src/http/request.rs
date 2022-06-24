use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    //Option - to represent the absence of value
    query_string: Option<String>,
    method: Method,
}

impl Request {}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
