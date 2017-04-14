extern crate env_logger;
#[macro_use]
extern crate log;
extern crate snack_card_reader;

use snack_card_reader::BarcodeIter;

fn main() {
    env_logger::init().unwrap();
    loop {
        let iter = match BarcodeIter::new() {
            Ok(iter) => iter,
            Err(err) => {
                error!("Error: {:?}", err);
                continue
            },
        };
        for result in iter {
            match result {
                Ok(ev) => {
                    println!("{}", ev);
                },
                Err(err) => {
                    error!("Error: {:?}", err);
                    break
                },
            }
        }
    }
}
