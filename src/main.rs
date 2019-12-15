use numbers_to_english::{number_to_english, Opt};
use std::env;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::new(env::args())?;

    let num = get_number()?;
    println!("{}", number_to_english(num, &opt));
    Ok(())
}

fn get_number() -> Result<u64, Box<dyn Error>> {
    let mut buffer = String::new();
    println!("Please enter your number:");

    io::stdin().read_line(&mut buffer)?;

    buffer.trim().parse::<u64>().map_err(|err| err.into())
}
