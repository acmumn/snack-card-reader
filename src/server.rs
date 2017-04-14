//! An HTTP server that serves barcodes as server-sent events.

use ::Error;
use hyper::
use rustls::ServerConfig;
use std::net::{ToSocketAddrs};
use toml;

pub struct Server {}

impl Server {
    /// Creates a new server, loading configuration from the given file's path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Server, Error> {
        let config = Config::load_from(config_path)?;
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub addr: String,
    pub tls: ServerConfig,
}

impl Config {
    /// Loads the config from the file at the given path.
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut file = File::open(p)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(toml::from_str(&buf)?)
    }

    /// Saves the config to a file, which willbe created at the given path.
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut file = File::create(path)?;
        write!(file, "{}", toml::to_string(self)?)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            addr: ":8080",
            tls: ServerConfig {}
        }
    }
}
