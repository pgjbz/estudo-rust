mod guess;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use crate::guess::Guess;


fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..101);

	loop {
		println!("Please input your guess: ");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess = match Guess::new(
			match guess.trim().parse() {
				Ok(v) => v,
				Err(e) => {
					eprintln!("Please enter a number");
					continue;
				}
			}) {
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
