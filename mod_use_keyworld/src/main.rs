mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){
            println!("Add.")
        }

        pub fn wait(){
            println!("Waiting...")
        }
    }
}

// use crate::front_of_house::hosting; -> from the root
// use self::front_of_house::hosting; -> from the same behavior
pub use self::front_of_house::hosting::{add_to_waitlist, wait}; // -> for import a function, struct, etc. Only, pub word make others imports to use this import
use std::collections::HashMap;
use rand::Rng;
// use std::fmt::Result;
// use std::io::Result as IoResult; //alias for import with same name


pub fn eat_a_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
    wait();
}

fn main() {
    eat_a_restaurant();
    let mut map: HashMap<i8, i8> = HashMap::new();
    map.insert(1, 2);
    for(k, v) in map.iter() {
        println!("{} - {}", k, v);
    }
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Number generated {}", secret_number);
}
