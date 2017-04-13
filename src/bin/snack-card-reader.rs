extern crate futures;
extern crate snack_card_reader;

use snack_card_reader::BarcodeStream;
use futures::future::empty;
use futures::{Future, Stream};

fn main() {
    let stream = BarcodeStream::new()
        .expect("couldn't open barcode reader");
    stream.for_each(|barcode| {
        println!("Barcode: {:?}", barcode);
        empty()
    }).wait().unwrap();
}
