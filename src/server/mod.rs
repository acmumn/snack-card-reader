//! An HTTP server that serves barcodes as server-sent events.

mod config;

use bus::Bus;
use errors::*;
use hyper::net::HttpsListener;
use hyper::server::{Listening, Request, Response};
use hyper::server::Server as HyperServer;
use hyper_rustls::TlsServer;
pub use self::config::Config;
use std::path::Path;
use std::sync::{Arc, Mutex};
use void::Void;

/// A simple HTTPS-supporting broadcast server-sent event server.
pub struct Server<T: Clone + Send> {
    bus: Arc<Mutex<Bus<T>>>,
    handle: Option<Listening>,
    server: Option<HyperServer<HttpsListener<TlsServer>>>,
}

impl<T: Clone + Send> Server<T> {
    /// Creates a new server, loading configuration from the given file's path.
    pub fn open<P: AsRef<Path>>(config_path: P) -> Result<Server<T>> {
        let config = Config::load_from(config_path)?;
        let mut private_keys = config.tls.load_private_keys()?;
        let certificates = config.tls.load_certificates()?;

        if private_keys.len() != 1 {
            return Err(ErrorKind::InvalidPrivateKeyCount(private_keys.len()).into());
        }
        if certificates.len() == 0 {
            return Err(ErrorKind::NoCertificatesFound.into());
        }

        let tls_server = TlsServer::new(certificates, private_keys.pop().unwrap());
        let listener = HttpsListener::new(config.addr, tls_server)?;
        let server = HyperServer::new(listener);
        Ok(Server {
            bus: Arc::new(Mutex::new(Bus::new(config.tuning.bus_buffer))),
            handle: None,
            server: Some(server),
        })
    }

    /// Starts handling requests. Will send any Events sent to the server's
    /// `send()` function to all clients that are connected.
    pub fn serve(&self) -> Result<()> {
        let bus = self.bus.clone();
        let handle = self.server.take().unwrap().handle(|mut req: Request, mut res: Response| {
            let rx = {
                let lock = bus.try_lock().unwrap();
                lock.add_rx()
            };
            for ev in rx.iter() {
                // TODO Send
                unimplemented!()
            }
            // TODO close conn nicely
            unimplemented!()
        })?;
        self.handle = Some(handle);
        Ok(())
    }

    /// Sends an Event to all connected clients.
    pub fn send(&self, ev: T) {
        self.bus.try_lock().unwrap().broadcast(ev);
    }
}
