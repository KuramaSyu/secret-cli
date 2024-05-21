use clap::{Parser, ArgAction}; //Subcommand
use rand::prelude::*;
use std::io::{self, BufRead};
use std::collections::HashMap;
use include_dir::{include_dir, Dir};

// Embed the entire `wordlists` directory
static WORDLISTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/wordlists");

#[derive(Parser)]
#[command(author, version, about, long_about = "A tool to generate secrets. By default it uses numbers, lower case letters and upper case letters (-naA)")]
struct Args {
    #[arg(default_value = None)]
    length: Option<u32>,
    #[arg(short = 'n', long, action = ArgAction::SetTrue, help = "Wether to use numbers")]
    numbers: bool,
    #[arg(short = 'v', long, action = ArgAction::SetTrue, help = "Wether to be verbose")]
    verbose: bool,
    #[arg(short = 'a', long, action = ArgAction::SetTrue, help = "Wether to use lower case letters")]
    lower_letters: bool,
    #[arg(short = 'A', long, action = ArgAction::SetTrue, help = "Wether to use upper case letters")]
    upper_letters: bool,
    #[arg(short = 's', long, action = ArgAction::SetTrue, help = "Wether to use symbols")]
    symbols: bool,
    #[arg(short = 'w', long, action = ArgAction::SetTrue, help = "Wether to use words instead of characters")]
    words: bool
}

fn main() {
    let args = Args::parse();
    let length: Option<u32> = args.length;
    let verbose: bool = args.verbose;
    let character_set: String;
    let secret: String;
    if 
        !args.numbers 
        && !args.lower_letters 
        && ! args.upper_letters
        && !args.symbols
    {
        character_set = make_character_set(
            true, 
            true, 
            true, 
            false
        );
    } else {
        character_set = make_character_set(
            args.numbers, 
            args.lower_letters, 
            args.upper_letters,
            args.symbols
        );
    }
    if args.words {
        let wordlist = load_wordlist_from_embedded("eng.txt").unwrap();
        secret = generate_word_secret(wordlist, args.length.unwrap_or(5) as i32, args.numbers);
    } else {
        secret = generate_character_secret(
            character_set, 
            length.unwrap_or(20) as i32
        );
    }
    if verbose {
        println!("Secret:\n--------\n{}\n--------", secret);
    } else {
        println!("{secret}");
    }
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

fn generate_word_secret(words: Vec<String>, length: i32, with_numbers: bool) -> String {
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
    let leetspeak: HashMap<&str, &str> = [
        ("a", "4"),
        ("e", "3"),
        ("o", "0"),
        ("i", "1"),
    ].iter().cloned().collect();
    if !with_numbers {
        return secret
    }
    for (key, value) in leetspeak {
        secret = secret.replace(key, value);
    }
    secret
}
