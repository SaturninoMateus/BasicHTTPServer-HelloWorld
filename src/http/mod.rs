/*
if main.rs tries to import from the http
folder, for eg: use http::request::Request,
the "use http::" will try to look for a file called
"http.rs"  or "mod.rs", hence the need of this file
 */


// in order to be able to do: use http::Method; rather than use http::method::Method
pub use method::Method;
pub use request::Request;

pub mod request;
pub mod method;
