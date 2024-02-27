extern crate core;

mod config;

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::config::objs_config::ObjsConfig;

fn main() {

    //Initialize logging
    SimpleLogger::new()
                .with_level(LevelFilter::Info)
                .without_timestamps()
                .init()
                .unwrap();

    //Load the config from the environment or fallback to the default
    let config = ObjsConfig::new().unwrap();
}