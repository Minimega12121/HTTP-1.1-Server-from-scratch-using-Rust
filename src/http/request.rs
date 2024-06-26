use crate::http::request;

use super::method::{Method,MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult,Formatter,Display,Debug};
use std::str;
use std::str::Utf8Error;
use super::{QueryString,QueryStringValue};
//enum is a special type with finite set of values 

//TODO Add Header to both request and response
#[derive(Debug)]
pub struct Request<'buf>{
    path : &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf>{
    pub fn path(&self) -> &str{
        &self.path
    }

    pub fn query_string(&self) -> Option<&QueryString>{
        self.query_string.as_ref()
    }

    pub fn method(&self) -> &Method{
        &self.method
    }

}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }
        //same as above

        let request = str::from_utf8(buf)?;
        
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),
        // }
        //same as above

        let (method,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, __) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     },
        //     None => {},//We dont care for this case
        // };
        
        //same as above, used when we only care about one case and dont care about other
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        Ok(Self{
            path,
            query_string,
            method,
        }) 
    }
}
fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate(){
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

impl ParseError{
    fn message(&self) -> &str{
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
}
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncoding
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f, "{}", self.message())
    }
}


impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult{
        write!(f, "{}", self.message())
    }
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}


impl Error for ParseError{}