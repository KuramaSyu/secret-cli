use clap::{Parser, ArgAction}; //Subcommand
use rand::prelude::*;
use rand::distributions::Alphanumeric;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = 32)]
    length: u32,
    #[arg(short = 'n', long, action = ArgAction::SetTrue)]
    only_numbers: bool
}

fn main() {
    let args = Args::parse();
    let number: u32 = args.length;
    let only_numbers: bool = args.only_numbers;
    let mut secret: String = String::from("");
    let mut rng = rand::thread_rng();
    if only_numbers {
        for _ in 0..number{
            secret.push_str(&rng.gen_range(0..10).to_string())
        }
    } else {
        for _ in 0..number {
            secret.push(rng.sample(&Alphanumeric) as char);
        }
    }
    println!("Secret: {}", secret);
}
