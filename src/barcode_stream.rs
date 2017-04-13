use event_stream::EventStream;
use futures::{Async, Poll, Stream};
use std::io::Error;

pub struct BarcodeStream {
    buf: String,
    evs: EventStream,
}

impl BarcodeStream {
    /// Creates a new BarcodeStream.
    pub fn new() -> Result<BarcodeStream, Error> {
        EventStream::new().map(BarcodeStream::wrap)
    }

    /// Wraps an EventStream into a BarcodeStream.
    pub fn wrap(evs: EventStream) -> BarcodeStream {
        BarcodeStream {
            buf: String::new(),
            evs: evs,
        }
    }
}

impl Stream for BarcodeStream {
    type Item = String;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<String>, Error> {
        println!("BCODE POLL");
        let ev = match self.evs.poll() {
            Ok(Async::Ready(Some(ev))) => ev,
            Ok(Async::Ready(None)) => return Ok(Async::Ready(None)),
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => return Err(err),
        };
        println!("Got event {:?}", ev);
        unimplemented!()
    }
}
