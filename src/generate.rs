use rand::{self, distributions::Uniform, Rng, RngCore, SeedableRng};

pub fn random_string(chars: &Vec<char>, length: usize) -> String {
	// Create an empty buffer [0, 0, ..., 0] of 32 bytes
	let mut seed = [0u8; 32];

	// Fill the buffer with random data
	rand::thread_rng().fill_bytes(&mut seed);

	// Use the 32 random bytes as the seed to a (cryptographically secure) number generator
	let rng = rand::rngs::StdRng::from_seed(seed);

	// Create the range in which the generated numbers should fall
	let range = Uniform::new(0, chars.len());

	// Iterate over `length` randomly generated numbers, using each
	// as an index to the `chars` vector, then collect the characters into a string
	rng.sample_iter(range)
		.take(length)
		.map(|i| chars[i])
		.collect::<String>()
}
