use crate::constants::{ZERO_TO_TWENTY, TENS};
use crate::options::Opt;

pub struct Group {
	hundreds: u8,
	tens: u8,
	units: u8,
	category: &'static str,
}

impl Group {
	pub fn new(num: u16, category: &'static str) -> Self {
		// sanity check to avoid out of bounds indexing
		assert!(num < 1000);

		Group {
			hundreds: (num / 100) as u8,
			tens: (num / 10 % 10) as u8,
			units: (num % 10) as u8,
			category,
		}
	}

	fn first_digit(&self, opt: &Opt) -> String {
		match self.hundreds {
			0 => String::new(),
			_ => format!(
				"{}{}hundred",
				ZERO_TO_TWENTY[self.hundreds as usize], opt.spacing
			),
		}
	}

	fn two_last_digits(&self) -> String {
		match (self.tens, self.units) {
			(0, _) => ZERO_TO_TWENTY[self.units as usize].to_owned(),
			(1, _) => ZERO_TO_TWENTY[(self.tens * 10 + self.units) as usize].to_owned(),
			(_, 0) => TENS[self.tens as usize].to_owned(),
			(_, _) => format!(
				"{}-{}",
				TENS[self.tens as usize], ZERO_TO_TWENTY[self.units as usize]
			),
		}
	}

	pub fn to_english(&self, opt: &Opt) -> String {
		let first_digit = self.first_digit(opt);
		let two_last_digits = self.two_last_digits();

		let english_group = match (first_digit.is_empty(), two_last_digits.is_empty()) {
			(true, true)	=>	String::new(),
			(true, false)	=>	two_last_digits,
			(false, true)	=>	first_digit,
			(false, false)	=>	format!("{}{}{}", first_digit, opt.spacing, two_last_digits),
		};

		if self.category.is_empty() || english_group.is_empty() {
			english_group
		} else {
			format!("{}{}{}", english_group, opt.spacing, self.category)
		}
	}
}
