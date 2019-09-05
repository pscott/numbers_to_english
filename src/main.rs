// add options
// read from STDIN
// convert number
// print number
// Ask for new number OR stop executing
// type ctrl + c to exit ?
use std::io;
use numbers_to_english::converter::number_to_english;

fn main() -> io::Result<()> {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer)?;

	let trimmed = buffer.trim();
	match trimmed.parse::<u64>() {
		Ok(num)	=>	println!("{}", number_to_english(num)),
		Err(..)	=>	eprintln!("Not a valid unsigned integer"),
	};
	Ok(())
}