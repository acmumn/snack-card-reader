use ioctls::input_event;
use std::fs::File;
use std::io::Error;
use super::hw;

pub struct EventIter {
    file: File,
}

impl EventIter {
    /// Creates a new EventIter.
    pub fn new() -> Result<EventIter, Error> {
        Ok(EventIter {
            file: try!(hw::get_device()),
        })
    }
}

impl Iterator for EventIter {
    type Item = Result<input_event, Error>;

    fn next(&mut self) -> Option<Result<input_event, Error>> {
        Some(hw::read_input_event(&mut self.file))
    }
}
