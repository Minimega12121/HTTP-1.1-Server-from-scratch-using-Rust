use super::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};


#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body : Option<String>) -> Self {
        Response {
            status_code,
            body,
        }
    }
    //stream: &mut dyn Write => dynamic dispatch for TcpStream using Write, concrete implementation of function will be done during run-time
    //below is static dispatch to reduce runtime overhead 
    //static increases the memory usage while dynamic increases the runtime overhead
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(
             stream,
             "HTTP/1.1 {} {}\r\n\r\n{}", 
             self.status_code, 
             self.status_code.reason_phrase(), 
             body
         )
    }
}


