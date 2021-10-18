

fn main() {
    let num: Option<i32> = Some(5);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), //more conditions like if, but using the Some(x) value in operation
        Some(x) => println!("{}", x),
        None => println!("No value"),
    }

    let x = Some(10);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 4;
    let y = false;

    match x {
        (4 | 5 | 6) if y  => println!("yes"), //or, not bitwise or, () is not necessary, but determine precedence
        _ => println!("no"),
    }
}
