enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {

    /*
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    } */

    // ^ replaced with

    let coin_array: [Option<Coin>; 5] = [Some(Coin::Penny),
        Some(Coin::Quarter),
        Some(Coin::Nickel),
        Some(Coin::Penny),
        Some(Coin::Dime)];

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three")
    }

    let mut count = 0;

    for (i, coin) in coin_array.iter().enumerate() {
        if let Some(Coin::Penny) = coin {
            println!("Penny in position {}", i);
        } else {
            println!("No penny in position {}", i);
            count+=1;
        }
    }

    println!("Total of no pennies: {}", count)
}
