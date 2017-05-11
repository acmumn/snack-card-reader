//! A userspace driver for the snack machine's barcode reader.

#![deny(missing_docs)]

#[macro_use]
extern crate error_chain;
extern crate ioctls;
#[macro_use]
extern crate log;
extern crate num;

mod event_iter;
mod errors;
mod hw;
mod iter;
mod scancodes;

pub use errors::*;
pub use iter::Iter;
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::str::FromStr;

/// A scanned barcode.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Barcode(Type, String);

impl Display for Barcode {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}://{}", self.0, self.1)
    }
}

impl FromStr for Barcode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Barcode> {
        if let Some(p) = s.bytes().position(|c| c == '=' as u8) {
            let (val, typ) = s.split_at(p);
            Ok(Barcode(Type::from_str(&typ[1..])?, val.to_string()))
        } else {
            Err(ErrorKind::InvalidBarcode(s.to_string()).into())
        }
    }
}

/// The type of card a barcode is from.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// A University of Minnesota student ID.
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

    fn from_str(s: &str) -> Result<Type> {
        match s {
            "99121006009530000" => Ok(Type::UCard),
            _ => Err(ErrorKind::UnknownBarcodeType(s.to_string()).into()),
        }
    }
}
