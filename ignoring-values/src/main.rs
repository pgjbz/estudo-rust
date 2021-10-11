fn foo(_:i32, y: i32) {
    println!("This function only use y parameter: {}", y);
}


fn main() {

    println!("Ignoring values using _");
    
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    println!("Ignoring values in match");
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { //ignore both values using underscore to ignore values inside Some, compare if both are Some, if one is None jump to the _
            println!("Can't overwrite an existing customized value");
        }
        _ => { //any match, underscore here ignore all values and match
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    println!("\nIgnoring values in tuple");
    match numbers {
        (first, _, third, _, fifth) => { //ignoring values in tuple
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    let some_string = Some(String::from("Hello"));
    println!("\nIgnoring values in if let");
    if let Some(_) = some_string { //if use Some(_) the value is not borrow
        println!("Found string...");
    }

    println!("{:?}", some_string); //value of some_string is moved on if let Some(_s)....

    foo(1, 4);

    println!("\nUse .. syntax to ignore rest of values: ");

    println!("In struct");

    struct Point {
        x: i32,
        _y: i32, //ignore unused value
        _z: i32, //ignore unused value
    }

    let origin = Point {x: 0, _y: 0, _z: 0};

    match origin {
        Point { x,.. } => println!("x is {}", x) //ignore rest os values
    }

    println!("In tuple");
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => { //take the first value and the last value
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // match numbers {
    //     (.., second, ..) => { //cannot be used two of '..' in a tuple
    //         println!("Some numbers: {}", second); //its impossible to rust to determine how many values are ignored in this type of case
    //     }
    // }

}
