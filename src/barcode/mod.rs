mod event_iter;
mod hw;
mod iter;
mod scancodes;

pub use self::iter::Iter;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Barcode(Type, String);

impl Display for Barcode {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}://{}", self.0, self.1)
    }
}

impl FromStr for Barcode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Barcode, Error> {
        if let Some(p) = s.bytes().position(|c| c == '=' as u8) {
            let (val, typ) = s.split_at(p);
            Ok(Barcode(Type::from_str(&typ[1..])?, val.to_string()))
        } else {
            Err(Error::NotACard)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidType,
    Io(IoError),
    NotACard,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    UCard,
}

impl Display for Type {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Type::UCard => write!(fmt, "ucard"),
        }
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Type, Error> {
        match s {
            "99121006009530000" => Ok(Type::UCard),
            _ => Err(Error::InvalidType),
        }
    }
}
