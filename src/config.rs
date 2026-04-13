use std::{
    env,
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
};

/// 127.0.0.1 if debugging, 0.0.0.0 otherwise
const DEFAULT_ADDRESS: IpAddr = if cfg!(debug_assertions) {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
} else {
    IpAddr::V4(Ipv4Addr::UNSPECIFIED)
};
const DEFAULT_PORT: u16 = 3000;

/// struct that represents configuration options and holds their values
#[derive(Debug)]
pub struct Config {
    address: IpAddr,
    port: u16,
    cert_path: Option<PathBuf>,
    key_path: Option<PathBuf>,
    db_url: String,
}

impl Config {
    /// read environment variables and build a [Config] from them
    ///
    /// currently silently ignores errors when reading the variables or parsing them
    /// variables:
    /// - `MOERDERSPIEL_ADDRESS`: address to bind on
    /// - `MOERDERSPIEL_PORT`: port to bind on
    /// - `MOERDERSPIEL_TLS_CERT_PATH`: path to the tls certificate chain
    /// - `MOERDERSPIEL_TLS_KEY_PATH`: path to the tls private key
    ///
    /// will panic if `DATABASE_URL` is not set
    ///
    /// both the cert path and the key path need to be set to enable tls
    pub fn from_env() -> Self {
        Self {
            address: if let Ok(v) = env::var("MOERDERSPIEL_ADDRESS") {
                v.parse().unwrap_or(DEFAULT_ADDRESS)
            } else {
                DEFAULT_ADDRESS
            },
            port: if let Ok(v) = env::var("MOERDERSPIEL_PORT") {
                v.parse().unwrap_or(DEFAULT_PORT)
            } else {
                DEFAULT_PORT
            },
            cert_path: env::var("MOERDERSPIEL_TLS_CERT_PATH").map(Into::into).ok(),
            key_path: env::var("MOERDERSPIEL_TLS_KEY_PATH").map(Into::into).ok(),
            db_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }

    pub fn address(&self) -> IpAddr {
        self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn cert_path(&self) -> Option<&Path> {
        self.cert_path.as_deref()
    }

    pub fn key_path(&self) -> Option<&Path> {
        self.key_path.as_deref()
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }
}
