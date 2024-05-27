use serde_yaml;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

/// Struct representing the options for generating random data.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Options {
    language: String,
    length: Option<usize>,
    length_passphrase: usize,
    upper_letters: bool,
    lower_letters: bool,
    symbols: bool,
    words: bool
}

/// Struct representing the configuration for the random generator.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    options: Options
}

/// Loads the configuration from a YAML file.
///
/// # Arguments
///
/// * `path` - The path to the YAML file.
/// * `verbose` - Whether to print verbose output.
///
/// # Returns
///
/// The loaded configuration.
pub fn load_config(path: &str, verbose: bool) -> Config {
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
    let handler_result = File::open(path);
    let mut handler = match handler_result {
        Err(err) => {
            if verbose {
                println!("Can't use `config.yaml`: {}", err)
            }
            return default_config
        },
        Ok(f) => f
    };
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
            default_config
        }
    }
}

/// Gets the language from the configuration.
///
/// # Arguments
///
/// * `config` - The configuration.
///
/// # Returns
///
/// The language as an `Option<&str>`.
pub fn get_language<'a>(config: &'a mut Config) -> Option<&'a str> {
    Some(&config.options.language)
}

/// Sets the given flags as default in the config.yaml file.
///
/// If an argument is None, then it won't be set
/// 
/// # Arguments
///
/// * `config` - The configuration.
/// * `path` - The path to the config.yaml file.
/// * `lang` - The language to set as default.
/// * `length` - The length to set as default.
/// * `upper_letters` - Whether to include upper case letters.
/// * `lower_letters` - Whether to include lower case letters.
/// * `symbols` - Whether to include symbols.
/// * `words` - Whether to include words.
pub fn set_defaults(
    config: &mut Config,
    path: &str, 
    lang: Option<&str>,
    length: Option<usize>,
    upper_letters: bool,
    lower_letters: bool,
    symbols: bool,
    words: bool
) -> () {
    let handler = OpenOptions::new().write(true).open(path);
    let is_ok = match handler {
        Err(ref e) => {
            println!("Info: Can't change config to {} because: {}", &lang.unwrap(), e);
            false
        },
        _ => true
    };
    if !is_ok {
        return
    }

    if lang.is_some() {
        config.options.language = lang.unwrap().to_owned();
    }
    config.options.length = length;
    config.options.upper_letters = upper_letters;
    config.options.lower_letters = lower_letters;
    config.options.symbols = symbols;
    config.options.words = words;
    
    serde_yaml::to_writer(handler.unwrap(), &config).unwrap();
}




