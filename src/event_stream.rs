use futures::{Async, Poll, Stream};
use ioctls::input_event;
use low_level_hw;
use std::io::Error;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::thread::Builder;

pub struct EventStream {
    recv: Receiver<Result<input_event, Error>>,
    name: String,
}

impl EventStream {
    /// Creates a new EventStream.
    pub fn new() -> Result<EventStream, Error> {
        let mut file = low_level_hw::get_device()
            .expect("TODO GET A REAL ERROR HERE");
        let name = low_level_hw::get_device_name(&mut file);
        let (send, recv) = channel();
        try!(Builder::new().spawn(move || {
            loop {
                let result = low_level_hw::read_input_event(&mut file);
                if let Err(err) = send.send(result) {
                    println!("Error: {:?}", err);
                }
            }
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
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<input_event>, Error> {
        let res = match self.recv.try_recv() {
            Ok(Ok(ev)) => Ok(Async::Ready(Some(ev))),
            Ok(Err(err)) => Err(err),
            Err(TryRecvError::Empty) => Ok(Async::NotReady),
            Err(TryRecvError::Disconnected) => Ok(Async::Ready(None)),
        };
        println!("EVENT POLL -> {:?}", res);
        res
    }
}
