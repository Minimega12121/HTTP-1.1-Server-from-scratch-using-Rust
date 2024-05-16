use super::method::Method;

//enum is a special type with finite set of values 
pub struct Request {
    path : String,
    query_string: Option<String>,
    method: Method,
}