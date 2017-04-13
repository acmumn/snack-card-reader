use event_stream::EventStream;
use futures::{Poll, Stream};
use ioctls::input_event;
use std::error::Error;

pub struct BarcodeStream {
    buf: String,
    evs: EventStream,
}

impl BarcodeStream {
    /// Creates a new BarcodeStream.
    pub fn new() -> Result<BarcodeStream, Box<Error>> {
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
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Option<String>, Box<Error>> {
        unimplemented!()
    }
}
