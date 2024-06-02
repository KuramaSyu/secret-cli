use serde::{Serialize, Deserialize};
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;
use dirs::config_dir;
use toml;
use colored::Colorize;
/// Struct representing the options for generating random data.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Options {
    pub language: String,
    pub length: Option<usize>,
    pub upper_letters: bool,
    pub lower_letters: bool,
    pub symbols: bool,
    pub words: bool,
    pub numbers: bool,
}

/// Struct representing the configuration for the random generator.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub options: Options
}

static FILE_NAME: &str = "config.toml";

impl Config {
    pub fn is_valid(&self) -> bool {
        let options = &self.options;
        if 
        !options.upper_letters 
        && !options.lower_letters 
        && !options.symbols
        && !options.words 
        && !options.numbers {
            return false
        }
        true
    }
}

pub fn get_config_path(verbose: bool) -> Option<PathBuf> {
    let config_dir= config_dir();
    if config_dir.is_none() {
        if verbose {
            println!("Can't find configuration directory. Fallback to default config");
        }
        return config_dir
    }

    // make config path
    let mut path = config_dir.unwrap();
    path.push("secret");

    // ensure it exists
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
        if verbose {println!("Creating directory tree: {:?}", path)};
    } else {
        if verbose {println!("Using path: {}", path.to_string_lossy())}
    }
    path.push(&FILE_NAME);
    if !path.exists() {
        if verbose {println!("Creating {}", path.to_string_lossy())}
        File::create(&path).unwrap();
    }
    Some(path)
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
pub fn load_config(verbose: bool) -> Config {
    let default_config: Config = Config {
        options: Options {
            language: "ger".to_owned(),
            length: None,
            upper_letters: false,
            lower_letters: true,
            symbols: false,
            words: true,
            numbers: false,
        }
    };
    let path = get_config_path(verbose);
    if path.is_none() {
        return default_config
    }
    let mut handler = match File::open(path.as_ref().unwrap()) {
        Err(err) => {
            if verbose {
                println!("Can't use `{FILE_NAME}`: {}", err.to_string().red())
            }
            return default_config
        },
        Ok(f) => f
    };
    let mut content = String::new();
    handler.read_to_string(&mut content).unwrap();
    match toml::from_str(&content) {
        Ok(toml_content) => {
            if verbose {
                println!("load config: {}", path.unwrap().to_string_lossy().green());
            }
            toml_content
        },
        Err(error) => {
            if verbose {
                println!("`{}` is wrong formatted: {}", FILE_NAME, error.to_string().red());
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
    verbose: bool,
    config: &mut Config,
    lang: Option<&str>,
    length: Option<usize>,
    upper_letters: bool,
    lower_letters: bool,
    symbols: bool,
    words: bool,
    numbers: bool,
) -> () {
    let path = get_config_path(verbose);
    if path.is_none() {
        return
    }
    let mut handler = match OpenOptions::new().write(true).open(path.as_ref().unwrap()) {
        Err(ref e) => {
            println!("Info: Can't change config to {} because: {}", &lang.unwrap(), e.to_string().red());
            return
        },
        Ok(handler)=> handler
    };

    if lang.is_some() {
        config.options.language = lang.unwrap().to_owned();
    }
    config.options.length = length;
    config.options.upper_letters = upper_letters;
    config.options.lower_letters = lower_letters;
    config.options.symbols = symbols;
    config.options.words = words;
    config.options.numbers = numbers;
    
    let toml_content = toml::to_string_pretty(&config).unwrap();
    if verbose {
        println!("Writing config:\n```\n{}\n```\nto {}", &toml_content.green(), path.unwrap().to_string_lossy().green())
    }
    handler.write_all(toml_content.as_bytes()).unwrap();
}




