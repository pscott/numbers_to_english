#[derive(Default)]
pub struct Config {
	pub group_separator: &'static str,
	pub spacing: &'static str,
}

impl Config {
	pub fn new(group_separator: &'static str, spacing: &'static str) -> Self {
		Config {
			group_separator,
			spacing,
		}
	}
}