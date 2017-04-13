extern crate futures;
extern crate snack_card_reader;

// use snack_card_reader::BarcodeStream;
use snack_card_reader::EventStream;
use futures::future::ok;
use futures::{Future, Stream};

fn main() {
    #[cfg(feature = "x11")]
    snack_card_reader::x11_disable_reader();

    let stream = EventStream::new()
        .expect("couldn't open barcode reader");
    //let stream = BarcodeStream::new()
    //    .expect("couldn't open barcode reader");
    println!("Wrapping EventStream for {:?}", stream.name());
    stream.for_each(|event| {
        println!("Event: {:?}", event);
        ok(())
    }).wait().unwrap();
}
