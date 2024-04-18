use clap::Parser; //Subcommand
use rand::prelude::*;
use rand::distributions::Alphanumeric;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "32")]
    length: String
}

fn main() {
    let args = Args::parse();
    let number: i32 = args.length.parse().expect("length needs to be a number");
    let mut rng = rand::thread_rng();
    let mut secret: String = String::from("");
    for _ in 0..number {
        secret.push(rng.sample(&Alphanumeric) as char);
    }
    println!("Secret: {}", secret);
}
