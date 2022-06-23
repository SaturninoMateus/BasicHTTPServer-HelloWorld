use super::method::Method;

pub struct Request {
    path: String,
    //Option - to represent the absence of value
    query_string: Option<String>,
    method: Method,
}
