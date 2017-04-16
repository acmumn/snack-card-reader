use errors::*;
use rustls::internal::pemfile;
use rustls::{Certificate, PrivateKey, ProtocolVersion, ServerConfig};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub addr: String,
    pub tls: TLSConfig,
    pub tuning: TuningConfig,
}

impl Config {
    /// Loads the config from the file at the given path.
    pub fn load_from<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut file = File::open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(toml::from_str(&buf)?)
    }

    /// Saves the config to a file, which willbe created at the given path.
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;
        let s = toml::to_string(self)?;
        Ok(write!(file, "{}", s)?)
    }

    /// Creates a TLS config from the settings in the file.
    pub fn tls(&self) -> ServerConfig {
        let mut tls_config = ServerConfig::new();
        tls_config.versions = vec![
            ProtocolVersion::TLSv1_0,
            ProtocolVersion::TLSv1_1,
            ProtocolVersion::TLSv1_2,
            ProtocolVersion::TLSv1_3,
        ];

        tls_config
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            addr: ":8080".to_string(),
            tls: TLSConfig::default(),
            tuning: TuningConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TLSConfig {
    pub cert_file: String,
    pub private_key_file: String,
}

impl TLSConfig {
    /// Loads the private keys.
    pub fn load_private_keys(&self) -> Result<Vec<PrivateKey>> {
        let mut file = BufReader::new(File::open(&self.private_key_file)?);
        pemfile::rsa_private_keys(&mut file).map_err(|()| {
            ErrorKind::InvalidPrivateKeys(self.private_key_file.clone()).into()
        })
    }

    /// Loads the certificates.
    pub fn load_certificates(&self) -> Result<Vec<Certificate>> {
        let mut file = BufReader::new(File::open(&self.cert_file)?);
        pemfile::certs(&mut file).map_err(|()| {
            ErrorKind::InvalidCertificates(self.cert_file.clone()).into()
        })
    }
}

impl Default for TLSConfig {
    fn default() -> TLSConfig {
        TLSConfig {
            cert_file: "cert.pem".to_string(),
            private_key_file: "key.pem".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TuningConfig {
    pub bus_buffer: usize,
}

impl Default for TuningConfig {
    fn default() -> TuningConfig {
        TuningConfig {
            bus_buffer: 10,
        }
    }
}
