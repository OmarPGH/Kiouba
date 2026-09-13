use rand::Rng;

pub fn generate_name(min: u8, max: u8) -> String {
	let mut _name: String = String::new();
	let mut rng = rand::thread_rng();
	
	for mut _i in 0..rng.gen_range(min..=max) {
		let _rand_char: char = rng.gen_range('a'..='z');
		_name.push(_rand_char);
		_i += 1;
	}

	_name
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn min_and_max() {
		assert!(generate_name(1, 1).chars().count() == 1);
	}
}