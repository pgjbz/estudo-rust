fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {  //Fn is a Trait fn is a function pointer
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop
}

fn print_status(n: Status) {
    println!("The status is {:?}", n);
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x|  x + 2)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    let another_answer = do_twice(|x| {x +5}, 10); //accept closure
    println!("The another answer is {}", another_answer);

    let closure = return_closure();

    println!("The use of closure returns: {}", closure(10));

    let mut list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect(); //reference like Java
    list_of_status.push(Status::Stop);
    list_of_status.into_iter().for_each(print_status);
}
