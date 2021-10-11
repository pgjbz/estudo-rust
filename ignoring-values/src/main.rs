fn foo(_:i32, y: i32) {
    println!("This function only use y parameter: {}", y);
}


fn main() {
    
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

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

    match numbers {
        (first, _, third, _, fifth) => { //ignoring values in tuple
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }


    foo(1, 4);

}
