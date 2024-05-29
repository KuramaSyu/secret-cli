use clap::{Parser, ArgAction}; //Subcommand
use rand::prelude::*;
use core::panic;
use std::io::{self, BufRead};
use include_dir::{include_dir, Dir};

use crate::config::set_defaults;
mod config;

// Embed the entire `wordlists` directory
static WORDLISTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/wordlists");

#[derive(Parser)]
#[command(author, version, about, long_about = "A tool to generate secrets. By default it uses numbers, lower case letters and upper case letters (-naA)")]
struct Args {
    #[arg(default_value = None)]
    length: Option<usize>,
    #[arg(short = 'n', long, action = ArgAction::SetTrue, help = "Whether to use numbers")]
    numbers: bool,
    #[arg(short = 'a', long, action = ArgAction::SetTrue, help = "Whether to use lower case letters")]
    lower_letters: bool,
    #[arg(short = 'A', long, action = ArgAction::SetTrue, help = "Whether to use upper case letters")]
    upper_letters: bool,
    #[arg(short = 's', long, action = ArgAction::SetTrue, help = "Whether to use symbols")]
    symbols: bool,
    #[arg(short = 'w', long, action = ArgAction::SetTrue, help = "Whether to use words instead of characters")]
    words: bool,
    #[arg(short = 'l', long, default_value = None, help = "Set the language. [ger | eng]")]
    language: Option<String>,
    #[arg(short = 'v', long, default_value_t = false, help = "Whether or not the program should be verbose")]
    verbose: bool,
    #[arg(short = 'd', long, default_value_t = false, help = "Whether or not to set used flags in this command as default")]
    set_default: bool
}

fn get_by_arg_or_config<T>(use_config: bool, cli_arg: T, config_arg: T) -> T {
    if use_config {config_arg} else {cli_arg}
}

fn main() {
    let args = Args::parse();
    let length: Option<usize> = args.length;
    let character_set: String;
    let secret: String;
    let current_dir_bind = std::env::current_dir().unwrap();
    let _current_dir = current_dir_bind.to_str().unwrap();
    let verbose = args.verbose;
    let mut conf = config::load_config(verbose);
    let language = {
        if args.language.is_some() {
            // given by flag
            args.language.unwrap()
        } else {
            let language: Option<&str> = config::get_language(&mut conf);
            match language  {
                Some(l) => l.to_owned(), // given by config
                None => String::from("ger"), // not given
            }
        }
    };
    if args.set_default {
        set_defaults(
            &mut conf, 
            Some(&language),
            length,
            args.upper_letters,
            args.lower_letters,
            args.symbols,
            args.words,
            args.numbers
        );
    }

    
    // whether or not to use the config file
    let use_config: bool = {
        if 
            !args.numbers 
            && !args.lower_letters 
            && !args.upper_letters
            && !args.symbols
            && !args.words
        {
            true
        } else {
            false
        }
    };

    if use_config && !conf.is_valid() {
        let error = r#"
        There where no flags provided
        -> use config.yaml
        Issue: seems like all flags in the configs are set to false.
        Change it by using the --set-default flag.
        Example:
        secret 10 -naAs --set-default
        "#;
        panic!("{}", error.lines().map(|line| line.trim()).collect::<Vec<&str>>().join("\n"));
    }


    let words = get_by_arg_or_config(use_config, args.words, conf.options.words);
    let numbers = get_by_arg_or_config(use_config, args.numbers, conf.options.numbers);
    let lower_letters = get_by_arg_or_config(use_config, args.lower_letters, conf.options.lower_letters);
    let upper_letters = get_by_arg_or_config(use_config, args.upper_letters, conf.options.upper_letters);
    let symbols = get_by_arg_or_config(use_config, args.symbols, conf.options.symbols);
    let length = get_by_arg_or_config(use_config, length, conf.options.length);

    // generate the secret...
    if words {
        // ...with words
        let wordlist = load_wordlist_from_embedded(
            &format!("{0}.txt", language)
        ).unwrap();
        secret = generate_word_secret(
            wordlist, 
            length.unwrap_or(5) as i32, 
            numbers,
            lower_letters,
            upper_letters,
            symbols
        );
    } else {
        // ...with a character sequence
        character_set = make_character_set(
            numbers, 
            lower_letters, 
            upper_letters,
            symbols
        );
        secret = generate_character_secret(
            character_set, 
            length.unwrap_or(20) as i32
        );
    }
    println!("{secret}");
}

fn make_character_set(
    numbers: bool, 
    letters_lower: bool,
    letters_upper: bool,
    symbols: bool
) -> String {
    let mut character_set: String = String::from("");
    
    if numbers {
        character_set.push_str("1234567890");
    }
    if letters_lower {
        let chars: String = (b'a'..b'z').map(|x| x as char).collect();
        character_set.push_str(&chars);
    }
    if letters_upper {
        let chars: String = (b'A'..b'Z').map(|x| x as char).collect();
        character_set.push_str(&chars);      
    }
    if symbols {
        character_set.push_str("!@#$%&*_+-=.?");
    }
    character_set
}

fn generate_character_secret(character_set: String, length: i32) -> String {
    let mut secret: String = String::from("");
    let mut rng = rand::thread_rng();
    for _ in 0..length{
        let index = rng.gen_range(0..character_set.len());
        secret.push_str(
            &character_set.chars().nth(index).unwrap().to_string()
        );
    }
    secret
}

fn load_wordlist_from_embedded(filename: &str) -> io::Result<Vec<String>> {
    let file = WORDLISTS_DIR.get_file(filename).ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, format!("File {} not found", filename))
    })?;
    let content = file.contents_utf8().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, format!("Invalid UTF-8 in file {}", filename))
    })?;
    let reader = io::BufReader::new(content.as_bytes());
    let mut words = Vec::new();
    for line in reader.lines() {
        words.push(line?);
    }
    Ok(words)
}

fn generate_word_secret(
    words: Vec<String>, 
    length: i32, 
    with_numbers: bool,
    _with_lowercase_letters: bool,
    with_uppercase_letters: bool,
    _with_symbols: bool,
) -> String {
    let mut secret = String::new();
    let mut rng = rand::thread_rng();
    for n_ in 0..length {
        let index = rng.gen_range(0..words.len());
        let word = &words[index];
        if n_ > 0 {
            secret.push_str(&format!("-{word}"));
        } else {
            secret.push_str(word);
        }
    }
    secret = secret.to_lowercase();
    const LEETSPEAK: [[&str; 2]; 4] = [
        ["a", "4"],
        ["e", "3"],
        ["o", "0"],
        ["i", "1"],
    ];
    if with_uppercase_letters {
        let amount: usize = secret.len() / 5;
        let mut uppercase_indices: Vec<usize> = Vec::new();
        for _ in 0..amount {
            let mut num: usize = rng.gen_range(0..secret.len());
            while uppercase_indices.contains(&num) {
                num += 1;
                num = num % secret.len();
            }
            uppercase_indices.push(num)
        }
        secret = secret.chars().enumerate().map(|(i, char)| {
            if uppercase_indices.contains(&i) {
                return char.to_ascii_uppercase();
            }
            return char
        }).collect::<String>();
    }
    if with_numbers {
        for [key, value] in LEETSPEAK {
            secret = secret.replace(key, value);
        }
    }

    secret
}
