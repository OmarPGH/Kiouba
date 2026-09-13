mod generate_name;
mod input;

use dialoguer::{Confirm};
use generate_name::generate_name;
use input::input;

fn main() {
	let input = input();
	let mut counter: u16 = 1;
	loop {
		let mut result: Vec<String> = Vec::with_capacity(input.number_of_names as usize);

		for _i in 0..input.number_of_names {
			result.push(generate_name(input.min_number_of_letters, input.max_number_of_letters));
		};

		println!("\nResult [{}]: {}\n", counter, result.join(" - "));

		let confirmed: bool = Confirm::new()
			.with_prompt("Do you want another one?")
			.default(true)
			.interact()
			.unwrap();

		println!("\n----------");

		if !confirmed {
			break;
		}

		counter += 1;
	}
}