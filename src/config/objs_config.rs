use config::{Config, ConfigError, Environment, File};
use log::info;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjsConfig {
    pub test: String
}

impl ObjsConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(Environment::with_prefix("objs"))
            .build()?;

        info!("Using config: {:?}", s);

        s.try_deserialize()
    }
}