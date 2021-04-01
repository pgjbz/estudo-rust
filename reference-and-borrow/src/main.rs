
fn main() {

    let mut s1: String = String::from("Hello");
    let len: usize = calculate_length(&s1); //&s1 pass the reference of s1, not loses the ownership

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1); //s1 has to be mutable

    println!("New s1 value {}", s1);

    {
        let r3 = &mut s1;
    } // r3 goes out of scope here, so we can make a new reference with no problems.

    let r1: &mut String = &mut s1;



    /*
        : you can have only one mutable reference to a particular piece of data in a particular scope.
        let r2: &mut String = &mut s1;

        println!("{}, {}", r1, r2);
    */

    println!("{}", r1);


    let mut s = String::from("hello");
    /*
    let r4 = &s; // no problem
    let r5 = &s; // no problem
    let r6 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r4, r5, r6);

         let r6 = &mut s; // BIG PROBLEM
   |              ^^^^^^ mutable borrow occurs here

     */

    let r4 = &s; // no problem
    let r5 = &s; // no problem
    println!("{} and {}", r4, r5);
    // r1 and r2 are no longer used after this point

    let r6 = &mut s; // no problem
    println!("{}", r6);


}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn change(some_string: &mut String) { //fn change(some_string: &String) don't work, because is not mutable reference
    some_string.push_str(", world");
}