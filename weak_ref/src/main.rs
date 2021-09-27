use std::{cell::RefCell, rc::{Rc, Weak}};


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>, 
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("Strong and weak counting");

    let leaf2 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );

    {
        let branch2 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf2)]),
        });

        *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);

        println!(
            "branch2 strong = {}, weak = {}",
            Rc::strong_count(&branch2),
            Rc::weak_count(&branch2),
        );

        println!(
            "leaf2 strong = {}, weak = {}",
            Rc::strong_count(&leaf2),
            Rc::weak_count(&leaf2),
        );
    }

    println!("leaf2 parent = {:?}", leaf2.parent.borrow().upgrade());
    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2),
    );
}
