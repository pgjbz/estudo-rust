
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Alabama => { println!("Supported state {:?}", state) }
                UsState::Alaska => { println!("Supported state {:?}", state) }
                _ => { println!("Not supported {:?}", state)}
            }
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match  x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn print_number(x: Option<i32>) {
    println!("{:?}", x.expect("no value"));
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Texas)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    print_number(five);
    print_number(six);
}
