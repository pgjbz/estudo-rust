fn main() {
    let reference_to_nothing: String = dangle();
}

fn dangle() -> String {
    let s: String = String::from("hello");
    s
}

/*$ cargo run
Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
--> src/main.rs:5:16
|
5 | fn dangle() -> &String {
    |                ^ expected named lifetime parameter
        |
        = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    help: consider using the `'static` lifetime
        |
        5 | fn dangle() -> &'static String {
        |                ^^^^^^^^

        error: aborting due to previous error

        For more information about this error, try `rustc --explain E0106`.
        error: could not compile `ownership`

        To learn more, run the command again with --verbose.



        This error message refers to a feature we haven’t covered yet: lifetimes. We’ll discuss lifetimes in detail in Chapter 10.
        But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem:


        fn dangle() -> &String { // dangle returns a reference to a String

            let s = String::from("hello"); // s is a new String

            &s // we return a reference to the String, s
        } // Here, s goes out of scope, and is dropped. Its memory goes away.
          // Danger!

       The solution here is to return the String directly:


        fn no_dangle() -> String {
            let s = String::from("hello");

            s
        }
*/

/*
The Rules of References
Let’s recap what we’ve discussed about references:

At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.

*/