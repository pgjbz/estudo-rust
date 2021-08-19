mod guess;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use crate::guess::Guess;
use std::num::ParseIntError;


fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..101);

	loop {
		println!("Please input your guess: ");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let result = str_to_int(&guess);

		let guess_number: i32 = if let Err(err) = result {
			println!("Please enter a number");
			continue;
		} else {
			result.unwrap()
		};

		let guess = match Guess::new(guess_number) {
			Ok(v) => v,
			Err(e) => {
				eprintln!("{}", e);
				continue;
			}
		};

		println!("You guessed: {}", guess.value());

		match guess.value().cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big"),
			Ordering::Equal => {
				println!("You win");
				break;
			}
		}
	}
}

fn str_to_int(str: &String) -> Result<i32, ParseIntError> {
	let number = str.trim().parse()?;
	Ok(number)
}
