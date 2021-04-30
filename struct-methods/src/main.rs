#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
    
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {

    let rect1: Rectangle = Rectangle{
        width: 10,  height: 20
    };
    let rect2: Rectangle = Rectangle{
        width: 5,  height: 20
    };
    let rect3: Rectangle = Rectangle{
        width: 30,  height: 20
    };
    let square: Rectangle = Rectangle::square(10);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Rect 1{:?}", rect1);
    println!("Rect 2{:?}", rect2);
    println!("Rect 3{:?}", rect3);
    println!("Square{:?}", square);
    println!("Area: {}", rect1.area());
}
