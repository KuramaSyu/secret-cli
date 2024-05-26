use serde_yaml;
use serde::{Serialize, Deserialize};
use std::borrow::BorrowMut;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Options {
    language: String,
    length: Option<usize>,
    length_passphrase: usize,
    upper_letters: bool,
    lower_letters: bool,
    symbols: bool,
    words: bool,

}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    options: Options
}



pub fn load_config(path: &str, verbose: bool) -> Config {
    let mut handler: File = File::open(path).unwrap();
    let mut content = String::new();
    handler.read_to_string(&mut content).unwrap();
    let yaml_result: Result<Config, serde_yaml::Error> = serde_yaml::from_str(content.as_str());
    match yaml_result {
        Ok(yaml_content) => yaml_content,
        Err(error) => {
            if verbose {
                println!("`config.yaml` is wrong formatted: {}", error.to_string());
                println!("Using default config");
            }
            let default_config: Config = Config {
                options: Options {
                    language: "ger".to_owned(),
                    length: None,
                    length_passphrase: 5,
                    upper_letters: false,
                    lower_letters: true,
                    symbols: false,
                    words: true,
                }
            };
            default_config
        }
    }
}

pub fn get_language<'a>(config: &'a mut Config) -> Option<&'a str> {
    Some(&config.options.language)
}

pub fn set_language(config: &mut Config, lang: &str, path: &str) -> () {
    let handler = OpenOptions::new().write(true).open(path);
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




