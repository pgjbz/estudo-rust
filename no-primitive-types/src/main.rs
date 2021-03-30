use std::io;

fn main() {
    println!("Tuple!");
    let tup: (i8, &str, u16) = (4, "Banana", 65000);
    let (x, y, z) = tup;
    println!("Tuple is a \"array\" with different types: {} - {} - {}", x, y, z);
    println!("Array types!");
    println!("Stack alloc array");
    let array: [u8; 5] = [0; 5];
    println!("Stack array value in pos 0: {}", array[0]);
    println!("Heap array");
    let heap_array: Box<[i32]> = Box::new([45]);
    println!("Heap array values {}", heap_array.get(0).expect("Not expected"));
    println!("Vect type");
    let mut vect: Vec<i32> = Vec::new();
    vect.insert(0,45);
    vect.insert(0,45);
    vect.insert(0,45);
    println!("Vect values");
    for x in 0..vect.len() {
        println!("{}", x);
    }

    input_text();
}

fn input_text(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}