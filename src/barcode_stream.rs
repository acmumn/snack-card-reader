use event_stream::EventStream;
use futures::{Poll, Stream};
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
        let ev = try_ready!(self.evs.poll());
        println!("Got event {:?}", ev);
        unimplemented!()
    }
}
