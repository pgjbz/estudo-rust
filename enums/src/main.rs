use std::borrow::Borrow;

enum IpAddrKindExampleOne {
    V4,
    V6,
}

enum IpAddrKindExampleTwo {
    V4(String),
    V6(String),
}

enum IpAddrKindExampleThree {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddExampleOne {
    kind: IpAddrKindExampleOne,
    address: String,
}

struct Ipv4Address {}
struct Ipv6Address {}

enum IpAddExampleThree {
    V4(Ipv4Address),
    V6(Ipv6Address)
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Write(value) => println!("{}", value),
            _ => println!("Nothing")
        }
    }
}

fn main() {
    let fourExampleOne: IpAddrKindExampleOne = IpAddrKindExampleOne::V4;
    let sixExampleOne: IpAddrKindExampleOne = IpAddrKindExampleOne::V6;

    let homeFourExampleOne: IpAddExampleOne = IpAddExampleOne {
        kind: fourExampleOne,
        address: String::from("127.0.0.1"),
    };

    let homeSixExampleOne: IpAddExampleOne = IpAddExampleOne {
        kind: sixExampleOne,
        address: String::from("::1"),
    };

    let fourExampleTwo: IpAddrKindExampleTwo = IpAddrKindExampleTwo::V4(String::from("127.0.0.1"));
    let sixExampleTwo: IpAddrKindExampleTwo = IpAddrKindExampleTwo::V6(String::from("::1"));

    let fourExampleThree: IpAddrKindExampleThree = IpAddrKindExampleThree::V4(127,0,0,1);
    let sixExampleThree: IpAddrKindExampleThree = IpAddrKindExampleThree::V6(String::from("::1"));

    println!("First example");
    print_ip_address_example_one(homeFourExampleOne);
    print_ip_address_example_one(homeSixExampleOne);
    println!("Second example");
    print_ip_address_example_two(fourExampleTwo);
    print_ip_address_example_two(sixExampleTwo);
    println!("Third example");
    print_ip_address_example_three(fourExampleThree);
    print_ip_address_example_three(sixExampleThree);
    println!("Methods in enum");
    let m: Message = Message::Write(String::from("Hello"));
    m.call();
    println!("Option enum");
    let some_number: Option<i8> = Some(5);
    let some_string: Option<&str> = Some("String");
    let absent_number: Option<i8> = None;
    println!("{}", some_number.unwrap());
    println!("{}", absent_number.unwrap_or_else(||2));
}

fn rout(ip_kind_example_one: IpAddrKindExampleOne) {}

fn print_ip_address_example_one(example: IpAddExampleOne) {
    println!("{}", example.address);
}

fn print_ip_address_example_two(example: IpAddrKindExampleTwo) {
    match example {
        IpAddrKindExampleTwo::V4(value) => println!("{}", value),
        IpAddrKindExampleTwo::V6(value) => println!("{}", value)
    }
}

fn print_ip_address_example_three(example: IpAddrKindExampleThree) {
    match example {
        IpAddrKindExampleThree::V4(f, s, t, fo) => println!("{}.{}.{}.{}", f, s, t, fo),
        IpAddrKindExampleThree::V6(value) => println!("{}", value)
    }
}