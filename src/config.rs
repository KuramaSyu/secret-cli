use serde_yaml;
use serde::{Serialize, Deserialize};
use std::borrow::BorrowMut;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Options {
    language: String
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    options: Options
}

pub fn load_config(path: &str) -> Config {
    let mut handler: File = File::open(path).unwrap();
    let mut content = String::new();
    handler.read_to_string(&mut content).unwrap();
    let yaml: Config = serde_yaml::from_str(content.as_str()).unwrap();
    return yaml
}

pub fn get_language(config: &mut Config) -> Option<&str> {
    Some(&config.options.language)
}

pub fn set_language(config: &mut Config, lang: &str, path: &str) -> () {
    let mut handler = OpenOptions::new().write(true).open(path);
    let is_ok = match handler {
        Err(ref e) => {
            println!("Info: Can't change config to {} because: {}", lang, e);
            false
        },
        _ => true
    };
    if !is_ok {
        return
    }
    config.options.language = lang.to_owned();
    serde_yaml::to_writer(handler.unwrap(), &config).unwrap();
}




