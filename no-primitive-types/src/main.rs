

fn main() {
    println!("Tuple!");
    let tuple: (i8, &str, u16);
    tuple = (4, "Banana", 65000);
    println!("Tuple is a \"array\" with different types: {}, {}, {}", tuple.0, tuple.1, tuple.2);
    println!("Array types!");
    println!("Stack alloc array");
    let array: [u8; 5] = [255,1,2,3,4];
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
}
