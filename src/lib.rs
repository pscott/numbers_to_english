pub mod constants;
use crate::constants::{CATEGORIES, TENS, ZERO_TO_TWENTY};

pub struct Group {
	hundreds: u8,
	tens: u8,
	units: u8,
	category: &'static str,
}

impl Group {
	pub fn new(num: u16, category: &'static str) -> Self {
		// sanity check to prevent out of bounds indexing
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

	fn two_last_digits(&self, opt: &Opt) -> String {
		match (self.tens, self.units) {
			(0, _) => ZERO_TO_TWENTY[self.units as usize].to_owned(),
			(1, _) => ZERO_TO_TWENTY[(self.tens * 10 + self.units) as usize].to_owned(),
			(_, 0) => TENS[self.tens as usize].to_owned(),
			(_, _) => format!(
				"{}{}{}",
				TENS[self.tens as usize], opt.hyphen, ZERO_TO_TWENTY[self.units as usize]
			),
		}
	}

	pub fn to_english(&self, opt: &Opt) -> Option<String> {
		let first_digit = self.first_digit(opt);
		let two_last_digits = self.two_last_digits(opt);

		let english_group = match (first_digit.is_empty(), two_last_digits.is_empty()) {
			(true, true)	=>	return None,
			(true, false)	=>	two_last_digits,
			(false, true)	=>	first_digit,
			(false, false)	=>	format!("{}{}{}", first_digit, opt.spacing, two_last_digits),
		};

		if self.category.is_empty() {
			Some(english_group)
		} else {
			Some(format!("{}{}{}", english_group, opt.spacing, self.category))
		}
	}
}

pub struct Opt {
	pub group_separator:	String,
	pub spacing:			String,
	pub hyphen:				String,
}

impl Opt {
	pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
		let mut group_separator = String::from(", ");
		let mut spacing = String::from(" ");
		let mut hyphen = String::from("-");

		args.next();
		while let Some(opt) = args.next() {
			match opt.as_ref() {
				"--group_separator"	=>	group_separator = args.next().unwrap_or_else(||  group_separator),
				"--spacing"			=>	spacing = args.next().unwrap_or_else(|| spacing),
				"--hyphen"			=>	hyphen = args.next().unwrap_or_else(|| hyphen),
				_					=>	return Err("Invalid option"),
			}
		}

		Ok(Self {
			group_separator,
			spacing,
			hyphen,
		})
	}
}

impl Default for Opt {
	fn default() -> Self {
		Opt {
			group_separator: String::from(", "),
			spacing: String::from(" "),
			hyphen: String::from("-"),
		}
	}
}

pub fn number_to_english(mut num: u64, opt: &Opt) -> String {
	let mut vec: Vec<String> = vec![];
	let mut category = 0;

	while num != 0 {
		let slice = (num % 1000) as u16;
		let group = Group::new(slice, CATEGORIES[category]);
		if let Some(english_group) = group.to_english(opt) {
			vec.push(english_group)
		}
		category += 1;
		num /= 1000;
	}

	if vec.is_empty() {
		vec.push(String::from("zero"));
	}

	let mut iter = vec.into_iter().rev().peekable();
	let mut english_num = String::new();

	let group_separator = opt.group_separator.as_str();
	while let Some(item) = iter.next() {
		english_num.push_str(item.as_str());
		if iter.peek().is_some() {
			english_num.push_str(group_separator);
		}
	};
	english_num
}