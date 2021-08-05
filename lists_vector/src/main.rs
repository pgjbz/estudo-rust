fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(20);
    vec.insert(0, 24); //choose a position
    vec.push(40); //insert in final
    for (i, v) in vec.iter().enumerate() {
        println!("{} - {}", i, v);
    }

    let vec_macro = vec![0, 1, 2];

    println!("Mutable variable in loop");
    for mut i in vec_macro {
        i+=1;
        println!("{}", i);
    }
    {
        let v = vec![8u8, 9, 10]; //free on out of scope
        for i in v {
            println!("{}", i);
        }
    }

    let mut vec_element = vec![1, 2, 3, 4, 5];

    match vec_element.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100); //get return a Optional, is more secure than [] get

    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; //reference, if want a reference when access the v vector in second time have a borrowing
    let first = v[0]; //copy of value

    v.push(6);
    //
    println!("The first element is: {}", first);

    println!("No mutable reference");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("Mutable reference");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for r in row {
        match r {
            SpreadsheetCell::Int(v) => println!("{}", v),
            SpreadsheetCell::Float(v) => println!("{}", v),
            SpreadsheetCell::Text(v) => println!("{}", v)
        }
    }

}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}