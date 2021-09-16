#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let number_in_a_box = Box::new(5);
    println!("Number in a box is {}", number_in_a_box);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    println!("{:?}", list);
    
}
