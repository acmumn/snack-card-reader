#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate snack_card_reader;

use snack_card_reader::{BarcodeIter, Error, Server};

quick_main!(|| -> Result<(), Error> {
    let matches = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!(", "))
        (about: crate_description!())
        (@arg CONFIG: -c --config +takes_value "Sets the path of the configuration file (defaults to \"snack-card-reader.toml\").")
    ).get_matches();
    println!("{:?}", matches);

    env_logger::init().unwrap();

    let mut server = Server::open(matches.value_of("CONFIG").unwrap_or("snack-card-reader.toml"))?;
    server.serve()?;

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
});
