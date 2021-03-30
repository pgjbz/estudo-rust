fn main() {
    println!("Return is last statement: {}", my_int());
    println!("Return keyword: {}", double(45));
    void_return("success");

    let x = 5; //type inference

    let y = { //new scope
        let x = 3;
        x + 1 //return value
    };

    println!("The value of y is: {}", y);
}

fn my_int() -> i32 {
    println!("do anything");
    2458741
}

fn double(x: i32) -> i32 {
    return x * 2;
}

fn void_return(txt: &str) {
    println!("Void function: {}", txt);
}