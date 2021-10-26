use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Mamamia;

fn main() {
    Pancakes::hello_macro();
	Mamamia::hello_macro();
}
