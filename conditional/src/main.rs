use std::io;
use std::io::Read;

fn main() {
    let mut i: i8 = 3;
    let y: i8 = 10;

    if i < 3 {
        println!("I less than 3");
    } else {
        println!("I greater or equals 3");
    }

    if y <= 10 && i == 3 {
        println!("Match");
    }

    if i < 3 {
        println!("I less than 3");
    } else if y == 10 {
        println!("Y equals 10");
    }

    if i > 3 || y == 10 {
        println!("Pass");
    }

    let j = if i >= 3 { 45 } else { 0 };

    println!("{}", j);

    print!("Enter a number: ");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed do read input.");

    let n: i32 = input.trim().parse::<i32>().expect("invalid input");
    if n > 10 {
        println!("N greater than 10");
    } else {
        println!("N less than 10");
    }

    input.clear();

    print!("Enter password: ");

    io::stdin()
        .read_line(&mut input)
        .expect("failed do read input.");

    loop  {
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }
        if input.trim().eq("1234") {
            println!("Pass");
            break;
        }
        input.clear();

        println!("Try again!");
        io::stdin()
            .read_line(&mut input)
            .expect("failed do read input.");

        println!("{}", input);

    }
}
