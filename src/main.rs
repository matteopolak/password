use std::collections::HashSet;

use clap::Parser;
use cli::Args;

mod cli;
mod constants;
mod generate;

fn main() {
	let mut args = Args::parse();

	let chars = {
		if let Some(chars) = args.chars {
			let mut unique = HashSet::new();

			if args.digits {
				unique.extend(constants::DIGITS.as_slice());
			}

			if args.lower {
				unique.extend(constants::LETTERS_LOWER.as_slice());
			}

			if args.upper {
				unique.extend(constants::LETTERS_UPPER.as_slice());
			}

			chars.chars().for_each(|c| {
				unique.insert(c);
			});

			unique.into_iter().collect::<Vec<_>>()
		} else {
			let mut chars = Vec::new();

			if args.special || args.all {
				chars.extend_from_slice(constants::SPECIAL.as_slice());
			}

			if !args.digits && !args.lower && !args.upper && !args.special {
				args.all = true;
			}

			if args.digits || args.all {
				chars.extend_from_slice(constants::DIGITS.as_slice());
			}

			if args.lower || args.all {
				chars.extend_from_slice(constants::LETTERS_LOWER.as_slice());
			}

			if args.upper || args.all {
				chars.extend_from_slice(constants::LETTERS_UPPER.as_slice());
			}

			chars.shrink_to_fit();

			chars
		}
	};

	let password = generate::random_string(&chars, args.length);

	println!("{}", password);
}
