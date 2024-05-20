use clap::{Parser, ArgAction}; //Subcommand
use rand::prelude::*;
use rand::distributions::Alphanumeric;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 32)]
    length: u32,
    #[arg(short = 'n', long, action = ArgAction::SetTrue)]
    numbers: bool,
    #[arg(short = 'v', long, action = ArgAction::SetTrue)]
    verbose: bool,
    #[arg(short = 'a', long, action = ArgAction::SetTrue)]
    lower_letters: bool,
    #[arg(short = 'A', long, action = ArgAction::SetTrue)]
    upper_letters: bool,
    #[arg(short = 's', long, action = ArgAction::SetTrue)]
    symbols: bool
}

fn main() {
    let args = Args::parse();
    let number: u32 = args.length;
    let verbose: bool = args.verbose;
    let only_numbers: bool = args.numbers;
    let mut secret: String = String::from("");
    let mut rng = rand::thread_rng();
    let character_set: String = make_character_set(args.numbers, args.lower_letters, args.upper_letters, args.symbols);
    if only_numbers {
        for _ in 0..number{
            secret.push_str(&rng.gen_range(0..character_set).to_string())
        }
    } else {
        for _ in 0..number {
            secret.push(rng.sample(&Alphanumeric) as char);
        }
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
        character_set.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }
    return character_set
}
