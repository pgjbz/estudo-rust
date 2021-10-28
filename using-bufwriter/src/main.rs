use std::io::{self, Write};

fn main() {
	//using BufWriter
    let stdout = io::stdout(); // get the global stdout entity
	let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
	writeln!(handle, "foo: {}", 42).unwrap(); // add `?` if you care about errors here

	//simple lock
	let stdout = io::stdout(); // get the global stdout entity
	let mut handle = stdout.lock(); // acquire a lock on it
	writeln!(handle, "foo: {}", 42).unwrap(); // add `?` if you care about errors here
}
