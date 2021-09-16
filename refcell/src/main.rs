use std::cell::RefCell;
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    
    let vec = RefCell::new(vec![1, 2, 3]);
    let immutable_vec = vec![1,2,3];
    eprintln!("{:?}", vec);
    eprintln!("{:?}", immutable_vec);
    let mut mut_ref1 = vec.borrow_mut(); //mutable??? only able to have 1 mutable reference and various immutable reference
    // let mut mut_ref2 = vec.borrow_mut(); //panic in runtime
    mut_ref1.push(4);
    // mut_ref2.push(5); //panic in runtime
    //immutable_vec.push(4); cannot borrow immutable vec as mutable, as it not declared as mutable ref
    eprintln!("{:?}", vec); //print borrowed value
    eprintln!("{:?}", mut_ref1);

    let vec2 = RefCell::new(vec![1, 2, 3]);
    {
        let mut mut_ref2 = vec2.borrow_mut();
        mut_ref2.push(4);
        eprintln!("{:?}", vec2); //borrowed value
    }
    vec2.borrow_mut().push(5);
    eprintln!("{:?}", vec2); //take the reference again
}

//
// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

// - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
// - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
// source: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

