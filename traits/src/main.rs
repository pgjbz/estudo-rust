use std::fmt::{Display, Formatter};
use std::cmp::Ordering;

pub struct Dog {
    age: i32
}

pub struct Cat {
    live: i32
}

pub struct Parrot {
    love: i32
}

pub trait Animal {
    fn eat(&self) {
        println!("Eating...");
    }

    fn run(&self);
}

impl Animal for Dog {
    fn eat(&self) {
        println!("Dog eating with {} year old", &self.age);
    }

    fn run(&self) {
        println!("Dogo running....");
    }
}

impl Animal for Cat {
    fn eat(&self) {
        println!("Cat eating with {} lives", &self.live);
    }

    fn run(&self) {
        println!("Cat running....");
    }
}

impl Animal for Parrot {
    fn run(&self) {
        println!("Parrot running....");
    }
}

impl Display for Parrot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("Parrot love {}", self.love);
        Ok(())
    }
}

pub fn call_run(animal: &impl Animal) {
    animal.run();
}

pub fn call_eat<T: Animal>(animal: &T) {
    animal.eat();
}

pub fn use_different_types(animal1: &impl Animal, animal2: &impl Animal) {
    animal1.eat();
    animal2.eat();
}

pub fn use_same_type<T: Animal>(animal1: &T, animal2: &T) {
    println!("Force same type");
    animal1.eat();
    animal2.eat();
}

pub fn trait_bound<T>(animal: &T)  //or <T: Animal + Display>
    where
        T: Animal + Display{ //only use a struct if implements Animal and Display
    animal.eat();
}

pub fn create_animal() -> impl Animal + Display {
    Parrot {
        love: 99999
    }
}

#[derive(Eq)]
pub struct Person {
    name: String,
    height: u32,
}


impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"height\": {}, \"name\": \"{}\"}}", self.height, self.name);
        Ok(())
    }
}

fn main() {
    let dog = Dog {
        age: 20
    };

    let cat = Cat {
        live: 7
    };

    let parrot = Parrot {
        love: 99999999
    };

    call_eat(&dog);
    call_run(&dog);
    call_eat(&cat);
    call_run(&cat);
    call_eat(&parrot);
    call_run(&parrot);

    println!("Use impl Type and <T: Type> differences");

    use_different_types(&cat, &parrot); //can pass different implementations of trait

    use_same_type(&cat, &cat); //only same implementation

    trait_bound(&parrot);

    let animal = create_animal();
    trait_bound(&animal);

    let p1 = Person {
        height: 170,
        name: String::from("Bob")
    };
    let p2 = Person {
        name: String::from("Alex"),
        height: 171
    };

    if p2 == p1 {
        println!("Persons have same height");
    } else if p2 > p1 {
        println!("p2 is bigger than p1");
    } else {
        println!("p1 is bigger than p2");
    }

    let pair = Pair {
        x: p1,
        y: p2
    };
    pair.cmp_display();

    let list: Vec<Box<dyn Animal>> = vec![Box::new(parrot), Box::new(cat), Box::new(dog)];

    for i in list {
        i.eat();
    }
}
