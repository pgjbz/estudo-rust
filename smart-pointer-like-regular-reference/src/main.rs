use std::ops::Deref;

fn main() {
    let a = 1;
    let a_ref = &a;
    assert_eq!(1, a);
    assert_eq!(1, *a_ref);
    
    let b = 5;
    let b_box = Box::new(b); //difference betwen a_ref is a b_box a copy of b value on the heap
    assert_eq!(5, b);
    assert_eq!(5, *b_box);

    let c = 5;
    let c_my_box = MyBox::new(c);
    assert_eq!(5, c);
    assert_eq!(5, *c_my_box);

    let d = MyBox::new(String::from("Test"));
    hello(&d); //Implicit Deref Coercions
    hello(&(*d)[..]); //Derref if not have implicit deref
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}