extern crate barcodes;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate mime;

mod errors;

use barcodes::Iter as BarcodeIter;
use errors::*;
use futures::future::*;
use hyper::Get;
use hyper::header::*;
use hyper::server::*;
use hyper::status::StatusCode;
use mime::{Mime, SubLevel, TopLevel};
use std::net::SocketAddr;
use std::str::FromStr;

/// An index page that gets served to browsers.
const INDEX: &str = include_str!("index.html");

/// A JSON object representing the version of the server.
const VERSION: &str = r#"{"name":"swipe-server","version":"0.1.0"}"#;

fn main() {
    let addr = SocketAddr::from_str("0.0.0.0:8080").unwrap();
    Http::new()
        .bind(&addr, || Ok(Events::new().unwrap()))
        .unwrap()
        .run()
        .unwrap();

    unreachable!()
}

struct Events {
    iter: BarcodeIter,
}

impl Events {
    pub fn new() -> Result<Events> {
        Ok(Events {
            iter: BarcodeIter::new()?,
        })
    }
}

impl Service for Events {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> FutureResult<Response, hyper::Error> {
        ok(match (req.method(), req.path()) {
            (&Get, "/") => {
                Response::new()
                    .with_header(ContentLength(INDEX.len() as u64))
                    .with_header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])))
                    .with_body(INDEX)
            },
            (&Get, "/version") => {
                Response::new()
                    .with_header(ContentLength(VERSION.len() as u64))
                    .with_header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])))
                    .with_body(VERSION)
            },
            _ => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            },
        })
    }
}
