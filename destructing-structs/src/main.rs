struct Point {
    x: i32, 
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {x, y}
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

impl Message {
    pub fn execute(&self) {
        match self {
            Message::Quit => {
                println!("The quit variant has no data to destructure.")
            },
            Message::Move {x, y} => {
                println!("Move in x direction {} and in the y direction {}", x, y)
            },
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
                "Change the color to red {}, green {}, blue {}", 
                r, g, b),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {}, value {}", 
                h, s, v)

        }
    }
}

fn main() {
    let point = Point::new(10, 20);

    let Point {x: a, y: b} = point; //create the variables a and b that batch the values of the x and y fields of the Point struct
    let Point { x, y} = point;

    println!("a = {}, b = {}", a, b);
    println!("x = {}, y = {}", x, y);

    let p = Point::new(0, 7);
    
    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y)
    }

    println!("\nDestructing enum...");

    Message::ChangeColor(Color::Rgb(255,255,255)).execute();
    Message::ChangeColor(Color::Hsv(255,255,255)).execute();
    Message::Quit.execute();
    Message::Write("haha".to_string()).execute();
    Message::Move{x: 10, y: 20}.execute();


    println!("\nDestructing enum and struct...");

    let (feet, inches, Point { x, y }) = (3u8, 10u8, Point::new(3,-10));

    println!("feet {}", feet);
    println!("inches {}", inches);
    println!("x {}", x);
    println!("y {}", y);
    
}
