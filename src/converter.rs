use crate::options::Opt;
use crate::constants::CATEGORIES;
use crate::group::Group;

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

	vec.reverse();

	let mut iter = vec.iter().peekable();
	let mut english_num = String::new();

	while let Some(item) = iter.next() {
		let english_group = match iter.peek() {
			Some(_)	=>	format!("{}{}", item, opt.group_separator),
			None	=>	item.to_owned(),
		};
		english_num.push_str(english_group.as_str());
	}
	english_num
}