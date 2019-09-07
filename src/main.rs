use numbers_to_english::converter::number_to_english;
use numbers_to_english::options::Opt;
use std::env;
use std::io;
use std::error::Error;

fn get_number() -> Result<u64, Box<dyn Error>> {

    let mut buffer = String::new();
	println!("Please enter your number:");

    io::stdin().read_line(&mut buffer)?;

    let trimmed_buffer = buffer.trim();
    match trimmed_buffer.parse::<u64>() {
		Ok(num)	=>	Ok(num),
		Err(_)	=>	Err("Not a valid unsigned integer".into()),
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let opt = Opt::new(env::args())?;

	let num = get_number()?;
	println!("{}", number_to_english(num, &opt));
	Ok(())
}
