use clap::Parser;

#[derive(Parser)]
#[clap(
	author,
	version,
	about,
	help_template = "
{author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"
)]
pub struct Args {
	/// Whether to use every option
	#[clap(short = 'a', long)]
	pub all: bool,

	/// List of characters to use in addition to other options
	#[clap(short = 'c', long, value_parser)]
	pub chars: Option<String>,

	/// Whether to use digits
	#[clap(short = 'd', long)]
	pub digits: bool,

	/// Length of the password
	#[clap(short = 'l', long, default_value_t = 12)]
	pub length: usize,

	/// Whether to use lowercase letters
	#[clap(short = 'o', long)]
	pub lower: bool,

	/// Number of passwords to generate
	#[clap(short = 'n', long, default_value_t = 1)]
	pub num: usize,

	/// Whether to use special characters
	#[clap(short = 's', long)]
	pub special: bool,

	/// Whether to use uppercase letters
	#[clap(short = 'u', long)]
	pub upper: bool,
}
