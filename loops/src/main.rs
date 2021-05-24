fn main() {
    println!("Loop for");
    for i in 0..10 {
        print!(" - {} - ", i);
    }
    println!("\nLoop for each");

    let arr: [i8; 5] = [0; 5];

    for i in arr.iter() {
        print!(" - {} - ", i);
    }

    println!("\n");

    for (i, index) in arr.iter().enumerate() {
        print!(" - {}|{} - ", i, index);
    }

    println!("\nLoop while");

    let mut i: i8 = 0;

    while i < 10 {
        i+=1;
        println!("I value {}", i);
    }

    println!("Do while???");
    i = 0;

    loop {
        i+=1;
        println!("I value {}", i);
        if i >= 10 { break; }
    }

}
