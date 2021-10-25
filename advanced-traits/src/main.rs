use std::{fmt::{self}, ops::Add};

#[derive(Debug)]
struct Counter {
    value: usize
}
    
impl Counter {
    fn new(value: usize) -> Self {
        Self {
            value
        }
    }
}

impl Iterator for Counter {
    
    type Item = usize; //<PlaceholderType=ConcreteType>
                        //only one implementation

    fn next(&mut self) -> Option<Self::Item> {
        if self.value >= 10000 {
            None
        } else {
            self.value += 1;
            Some(self.value)
        }
    }
}

trait Provider<T> {
    fn get(&self) -> T;
}

impl Provider<usize> for Counter { //advanced traits prevent to multiple implementation for struct of the same trait
    fn get(&self) -> usize {
        self.value
    }
}

impl Provider<u8> for Counter { //advanced traits prevent to multiple implementation for struct of the same trait
    fn get(&self) -> u8 {
        8
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Add for Point { //if not use generic parameter, the default of rhs is Self
    type Output = Point; 

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

trait OutlinePrint: fmt::Display { //To implement this trait have to implement Display trait
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


impl OutlinePrint for Point { //Point doesn't implement std:fmt:Display
}


#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters { //use generic parameter to define rhs (Right hand side)
    type Output = Millimeters ;

    fn add(self, rhs: Meters) -> Self::Output {
        Self(self.0 + (rhs.0 * 1000))
    }
}

fn main() {

    let mut counter = Counter::new(0);

    for value in &mut counter {
        println!("{}", value);
    }

    Provider::<u8>::get(&counter);
    println!("{:?}", counter);

    println!("Pointers are equals? {}", Point::new(1, 0) + Point::new(2, 3) == Point::new(3, 3));
    Point::new(10, 10).outline_print();

    let meters = Meters(10);
    let millimeters = Millimeters(1000);
    let sum =  Millimeters(1000) + Meters(10);
    println!("Sum of {:?} and {:?} result is {:?}", meters, millimeters, sum);

    /*
        Newtype Pattern to implement External Traits on External types.
        This pattern have no runtime penalty 
        source: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
    */

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("w = {}", w);
}
