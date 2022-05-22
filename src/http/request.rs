use super::method::{Method, MethodError};
use super::{QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

impl<'buf> Request<'buf> {
    pub fn path(&self)->  &str{
        &self.path
    }
    pub fn method(&self)->  &Method{
        &self.method
    }
    pub fn query_string(&self)->  Option<&QueryString>{
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error>{
        // match str::from_utf8(buffer) {
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding); 
        // }

        // match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {},
        //     Err(e) => return Err(e)
        // }

        let request = str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding))?; // return Ok if no error, else error with ? at the end 
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..])) ;
            path = &path[..i];
        }

        // via match case
        // match path.find("?") {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     }
        //     None => {}
        // };

        // via if statement check for some 
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]);
        //     path = &path[..i];
        // } 
            Ok(Self {
                path,
                query_string,
                method,
            })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..])); // string slice, is actually 1 byte each, emoji may contain more than 1 byte hence make this code unsafe

        }
    }
    None
}


pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self{
        return Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self{
        return Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
                        Self::InvalidRequest => "Invalid Request",
                        Self::InvalidEncoding => "Invalid Encoding",
                        Self::InvalidProtocol => "Invalid Protocol",
                        Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}
// impl Request {
//     fn from_byte_array(buffer: &[u8]) -> Result<request, error>{
//         let string = String::from("123aezo");
//         string.encrypt();
//     }
// }

// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }