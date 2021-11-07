use serde::Deserialize;
use std::fs::read;
use std::path::Path;

const CONF_PATH: &str = "/etc/duckdnsv6/duckdnsv6.toml";

#[derive(Deserialize)]
pub struct Configuration {
    pub token: String,
    pub domain: String,
    pub ipv6lookup: String,
    pub fritzbox: bool,
    pub interfaceid: Option<String>,
}

pub fn get_configuration() -> Result<Configuration, String> {
    let path: &Path = Path::new(CONF_PATH);
    if ! path.exists() {
        return Err("Configuration file doesn't exist! (Can be copied from /usr/share/doc/duckdnsv6/duckdnsv6.toml.default)".to_string());
    }

    match toml::from_slice(read(path).unwrap().as_slice()) {
        Ok(data) => Ok(data),
        Err(err) => return Err(format!("{}", err))
    }
}