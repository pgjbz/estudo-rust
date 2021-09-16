use std::rc::Rc;
enum List {
    Const(i32, Rc<List>),
    Nil,
}

use crate::List::{Const, Nil};

fn main() {
    let a = Rc::new(Const(10, Rc::new(Const(11, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); //check ref counting
    let b = Const(12, Rc::clone(&a)); //Rc::clone don't copy entire data, only increase the reference counting
    println!("count after creating b = {}", Rc::strong_count(&a)); //weak_count is used to preventing reference cycles
    {
        let c = Const(13, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after creating c goes out scope = {}", Rc::strong_count(&a));
}
