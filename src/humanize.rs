static ZERO_TO_TEN: [&str; 10] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
static TEN_TO_TWENTY: [&str; 9] = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "nineteen"];
static TENS: [&str; 10] = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eigthy", "ninety"];

pub struct Cluster {
	hundreds: u8,
	tens: u8,
	units: u8,
}

impl Cluster {
	pub fn new(num: u16) -> Self {
		assert!(num < 1000);

		Cluster {
			hundreds: (num / 100) as u8,
			tens: (num / 10 % 10) as u8,
			units: (num % 10) as u8,
		}
	}

	pub fn humanize(&self) -> String {
		let first_digit = match self.hundreds {
			0 => String::new(),
			_ => format!("{} hundred", ZERO_TO_TEN[self.hundreds as usize]),
		};
		let under_twenty = self.tens * 10 + self.units;

		let last_digits = if self.units == 0 {
			TENS[self.tens as usize].to_owned()
		} else if under_twenty < 10 {
			ZERO_TO_TEN[under_twenty as usize].to_owned()
		} else if under_twenty < 20 {
			TEN_TO_TWENTY[(under_twenty  - 10) as usize].to_owned()
		} else {
			format!("{} {}", TENS[self.tens as usize], ZERO_TO_TEN[self.units as usize])
		};

		match (first_digit.is_empty(), last_digits.is_empty()) {
			(true, true) => String::new(),
			(true, false) => last_digits,
			(false, true) => first_digit,
			(false, false) => format!("{} {}", first_digit, last_digits)
		}
	}
}

pub fn number_to_english(num: usize) -> String {
	if num == 0 {
		return String::from("zero")
	}
	let cluster = Cluster::new(num as u16);
	cluster.humanize()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ten_to_twenty() {
		let res = number_to_english(12);
		assert_eq!(res, "twelve");
	}
	
	#[test]
	fn bigger_than_twenty() {
		let res = number_to_english(21);
		assert_eq!(res, "twenty one");
	}

	#[test]
	fn simple_tens() {
		let res = number_to_english(50);
		assert_eq!(res, "fifty");
	}

	#[test]
	fn simple_digit() {
		let res = number_to_english(5);
		assert_eq!(res, "five");
	}

	#[test]
	fn zero() {
		let res = number_to_english(0);
		assert_eq!(res, "zero");
	}


	#[test]
	fn nine_nine_nine() {
		let res = number_to_english(999);
		assert_eq!(res, "nine hundred ninety nine");
	}

	#[test]
	fn simple_hundred() {
		let res = number_to_english(200);
		assert_eq!(res, "two hundred");
	}
	// #[test]
	// fn bigger_than_a_thousand() {
		// let res = number_to_english(1000);
	// }
}