use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1.0", author = "Matthew Polak")]
pub struct Args {
	/// List of characters to use in addition to other options
	#[clap(short = 'c', long, value_parser)]
	pub chars: Option<String>,

	/// Number of passwords to generate
	#[clap(short = 'n', long, default_value_t = 1)]
	pub num: usize,

	/// Whether to use digits
	#[clap(short = 'd', long, takes_value = false)]
	pub digits: bool,

	/// Whether to use lowercase letters
	#[clap(short = 'o', long, takes_value = false)]
	pub lower: bool,

	/// Whether to use uppercase letters
	#[clap(short = 'u', long, takes_value = false)]
	pub upper: bool,

	/// Whether to use special characters
	#[clap(short = 's', long, takes_value = false)]
	pub special: bool,

	/// Length of the password
	#[clap(short = 'l', long, default_value_t = 12)]
	pub length: usize,

	/// Whether to use every option
	#[clap(short = 'a', long, takes_value = false)]
	pub all: bool,
}
