use add_one;

fn main() {
    let n = 10;
    println!("The number: {}
        plus 1: {}
    ", n, 
    add_one::add_one(n));
}
