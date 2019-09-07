pub struct Opt {
	pub group_separator: String,
	pub spacing: String,
}

impl Opt {
	pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
		let mut group_separator = String::from(",");
		let mut spacing = String::from(" ");

		args.next();
		while let Some(opt) = args.next() {
			match opt.as_ref() {
				"--group_separator"	=>	group_separator = args.next().unwrap_or_else(||  group_separator),
				"--spacing"			=>	spacing = args.next().unwrap_or_else(|| spacing),
				_					=>	return Err("Invalid option"),
			}
		}

		Ok(Self {
			group_separator,
			spacing,
		})
	}
}