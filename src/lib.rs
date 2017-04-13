extern crate byteorder;
extern crate futures;
extern crate ioctls;
extern crate libc;
extern crate void;

mod barcode_stream;
mod event_stream;
mod low_level_hw;

pub use barcode_stream::BarcodeStream;
pub use event_stream::EventStream;

#[cfg(feature = "x11")]
extern crate x11_highlevel;
#[cfg(feature = "x11")]
mod disable;
#[cfg(feature = "x11")]
pub use disable::disable as x11_disable_reader;
