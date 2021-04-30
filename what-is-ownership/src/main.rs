
fn main() {
    let mut s: String = String::from("Hello"); //the key world "mut" make variable mutable
    s.push_str(", world!");
    println!("{}", s);

    //Assigning a value x to y

    let x: i8 = 5;
    let y: i8 = x;
    println!("{} - {}", x, y);

    //String version

    let s1: String = String::from("hello");
    let s2: String = s1; //s1 loses the the pointer reference, the pointer copied to s2 and "free" the reference in s1
    /*
     println!("{}", s1);

     let s1: String = String::from("hello");
   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
15 |     let s2: String = s1;
   |                      -- value moved here
16 |
17 |     println!("{}", s1);
   |                    ^^ value borrowed here after move
    ***************** Rust also invalidates the first variable *************

    s2 out of the scope, the memory allocated is free
   */

    let s3: String = String::from("Hello");
    let s4: String = s3.clone(); //clone the values explicit

    println!("{} - {} ", s3, s4);

    /*
        Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    */

    let str_take_ownership: String = String::from("hello");  // str_take_ownership comes into scope

             // str_take_ownership's value moves into the function...
    // ... and so is no longer valid here

    takes_ownership(str_take_ownership);
    let s5: String = String::from("Xaropinho");
    let s6: String = takes_and_gives_back(s5);

    /*
        The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
        When a variable that includes data on the heap goes out of scope,
        the value will be cleaned up by drop unless the data has been moved to be owned by another variable.
    */

    println!("{}", s6);

    /*

        println!("{}", str_take_ownership); //compile time error, str_take_ownership was dropped

    */
    let x:i8 = 5;                      // x comes into scope

    makes_copy(x);

    let s7 = String::from("hello");

    let (s8, len) = calculate_length(s7);

    println!("The length of '{}' is {}.", s8, len);
    /*
        It’s possible to return multiple values using a tuple, as shown in Listing 4-5.
    */

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn takes_and_gives_back(some_string: String) -> String { // some_string comes into scope
    println!("{}", some_string);
    return some_string; //return ownership
}

fn makes_copy(some_integer: i8) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.