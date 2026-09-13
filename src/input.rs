use dialoguer::{Input};

pub struct Returned {
	pub number_of_names: u8,
	pub min_number_of_letters: u8,
	pub max_number_of_letters: u8
}

pub fn input() -> Returned {
	let number_of_names: u8 = Input::new()
		.with_prompt("number of names you need")
		.default(5)
		.validate_with(|input: &u8| -> Result<(), &str> {
			if *input > 0 {
				Ok(())
			} else {
				Err("must be more than 0")
			}
		})
		.interact_text()
		.unwrap();
	let min_number_of_letters: u8 = Input::new()
		.with_prompt("min number of letters")
		.default(4)
		.validate_with(|input: &u8| -> Result<(), &str> {
			if *input > 1 {
				Ok(())
			} else {
				Err("must be more than 1")
			}
		})
		.interact_text()
		.unwrap();
	let max_number_of_letters: u8 = Input::new()
		.with_prompt("max number of letters")
		.default(7)
		.validate_with(|input: &u8| -> Result<(), &str> {
			if *input <= 1 {
				Err("must be more than 1")
			} else if *input < min_number_of_letters {
				Err("must be more than or equal the min")
			} else {
				Ok(())
			}
		})
		.interact_text()
		.unwrap();

	let result = Returned {
		number_of_names,
		min_number_of_letters,
		max_number_of_letters
	};

	result
}