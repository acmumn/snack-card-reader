use errors::*;
use num::FromPrimitive;
use std::str::FromStr;
use super::event_iter::EventIter;
use super::scancodes::Key;
use super::Barcode;

pub struct Iter {
    evs: EventIter,
}

impl Iter {
    /// Creates a new Iter.
    pub fn new() -> Result<Iter> {
        match EventIter::new() {
            Ok(evs) => Ok(Iter::wrap(evs)),
            Err(err) => Err(err.into()),
        }
    }

    /// Wraps an EventIter into a Iter.
    fn wrap(evs: EventIter) -> Iter {
        Iter {
            evs: evs,
        }
    }
}

impl Iterator for Iter {
    type Item = Result<Barcode>;

    fn next(&mut self) -> Option<Result<Barcode>> {
        let mut out = String::new();
        while let Some(res) = self.evs.next() {
            match res {
                Ok(event) => if event.value == 1 && event._type == 1 {
                    if let Some(k) = Key::from_u16(event.code) {
                        if k == Key::Enter {
                            break
                        }
                        out += k.to_str();
                    } else {
                        warn!("Unknown scan code: {:?}", event.code);
                    }
                },
                Err(err) => return Some(Err(err.into())),
            }
        }
        Some(Barcode::from_str(&out))
    }
}
