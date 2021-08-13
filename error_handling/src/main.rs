use std::fs::File;
use std::io::{Write, ErrorKind, Read};
use std::{io, fs};
use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
// 	let f = File::open("hello.txt")?;
// 	Ok(())
// }

fn main() {
	// panic!("Are u ok Annie?");
	/* Unrecoverable Errors
	let vect = vec![0, 1, 2];

	println!("{}", vect[99]);

	 */
	// using_match();
	//
	// maybe_closure();
	//
	// unwrap_or_panic();

	// unwrap_with_expected();

	// let result = read_username_from_file().expect("Error propagated");
	// let result = read_username_from_file_return_another_string().unwrap();
	// let result = read_username_from_file_place_holder_quest().unwrap();
	// let result = read_username_from_file_place_holder_quest_eliminate_boiler_plate().unwrap();
	let result = read_username_from_file_ultra_shorter().unwrap();
	eprintln!("{}", result);
}


/*
	propagate error

	Result<String, io::Error>. This means the function is returning a value of the type Result<T, E>
	where the generic parameter T has been filled in with the concrete type
	String and the generic type E has been filled in with the concrete type io::Error.
	. If this function succeeds without any problems,
	the code that calls this function will receive an Ok value that holds a String—the username that
	this function read from the file. If this function encounters any problems, the code that calls
	this function will receive an Err value that holds an instance of io::Error that contains more information
	about what the problems were. We chose io::Error as the return type of this function because
	that happens to be the type of the error value returned from both of the operations we’re calling
	in this function’s body that might fail: the File::open function and the read_to_string method.
	https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
*/
fn read_username_from_file() -> Result<String, io::Error> {
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e), //needs the return, because if not have return don't compile and compile gives a error
	};

	let mut s = String::new();
	let result = match f.read_to_string(&mut s) { //in this case match operator return a Result enum
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	};
	result
}

fn read_username_from_file_return_another_string() -> Result<String, String> {
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(String::from("ERROOO")), //needs the return, because if not have return don't compile and compile gives a error
	};

	let mut s = String::new();
	let result = match f.read_to_string(&mut s) { //in this case match operator return a Result enum
		Ok(_) => Ok(s),
		Err(e) => Err(String::from("Erro 2")),
	};
	result
}

/*
	The ? placed after a Result value is defined to work in almost the same way as the match
	expressions we defined to handle the Result
	https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
*/
fn read_username_from_file_place_holder_quest() -> Result<String, io::Error> {
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}

fn read_username_from_file_place_holder_quest_eliminate_boiler_plate() -> Result<String, io::Error>{
	let mut s = String::new();
	File::open("hello.txt")?.read_to_string(&mut s)?;
	Ok(s)
}


fn read_username_from_file_ultra_shorter() -> Result<String, io::Error> {
	fs::read_to_string("hello.txt")
}

fn using_match() {
	let f = File::open("hello.txt");

	let mut f = match f {
		Ok(file) => file,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				eprintln!("File hello.txt not found, creating...");
				match File::create("hello.txt") {
					Ok(file) => file,
					Err(e) => panic!("Error on creating file {:?}", e)
				}
			}
			other_error => panic!("Error on try to open file {:?}", other_error)
		}
	};

	f.write(b"hello world");
}

/*
    .unwrap functions execute the match expression and return the value of Ok
    if .unwrap return the variant of Result<T, E> execute the panic! method

*/

fn unwrap_or_panic() {
	let f = File::open("hello.txt").unwrap(); //execute panic! if hello.txt not exists
}

/*
    .expected have the same function .unwrap but, the messages on .expected is the message of panic!
*/

fn unwrap_with_expected() {
	let f = File::open("hello.txt").expect("Who?????");
}


fn maybe_closure() {
	let f = File::open("hello.txt").unwrap_or_else(|name_of_error| { //maybe call closure????
		if name_of_error.kind() == ErrorKind::NotFound {
			File::create("hello.txt").unwrap_or_else(|name_of_error| { //if success use Ok value, or_else execute the "closure"...
				panic!("Problem creating the file: {:?}", name_of_error);
			})
		} else {
			panic!("Problem opening the file: {:?}", name_of_error);
		}
	});
}
