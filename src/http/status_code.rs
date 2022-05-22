use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)] //using derive attribute to implement the traits
pub enum StatusCode {
    Ok = 200, //each enum has a positional index as default, but we are assigning status code number onto it instead
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str{
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "BadRequest",
            Self::NotFound => "NotFound",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
     }
}