use futures::{Async, Poll, Stream};
use ioctls::input_event;
use low_level_hw;
use std::error::Error;
use std::io::Error as IoError;
use std::sync::mpsc::{channel, Receiver};
use std::thread::Builder;

pub struct EventStream {
    recv: Receiver<Result<input_event, IoError>>,
    name: String,
}

impl EventStream {
    /// Creates a new EventStream.
    pub fn new() -> Result<EventStream, Box<Error>> {
        let mut file = low_level_hw::get_device()
            .expect("TODO GET A REAL ERROR HERE");
        let name = low_level_hw::get_device_name(&mut file);
        let (send, recv) = channel();
        try!(Builder::new().spawn(move || {
            while let Ok(()) = send.send(low_level_hw::read_input_event(&mut file)) {}
        }));
        Ok(EventStream { name, recv })
    }

    /// Gets the name of the device.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Stream for EventStream {
    type Item = input_event;
    type Error = Box<Error>;

    fn poll(&mut self) -> Poll<Option<input_event>, Box<Error>> {
        match self.recv.try_recv() {
            Ok(Ok(ev)) => Ok(Async::Ready(Some(ev))),
            Err(err) => Err(From::from(err)),
            _ => unimplemented!(),
        }
    }
}
