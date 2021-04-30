struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let immutable_user: User = User {
        name: String::from("Shaman"),
        email: String::from("shaman@shaman.com"),
        active: true,
        sign_in_count: 1
    };

    let mut mutable_user: User = User {
        name: String::from("Shaman"),
        email: String::from("shaman@shaman.com"),
        active: true,
        sign_in_count: 1
    };
    print!("Old user mutable: ");
    print_user(&mutable_user);
    print!("New user mutable: ");
    mutable_user.name = String::from("Virgo");
    print_user(&mutable_user);
    print_user(&immutable_user);
    print_user(&build_user1(String::from("Build user 1"), String::from("email.build1@mail.com")));
    print_user(&build_user2(String::from("Build user 2"), String::from("email.build2@mail.com")));
    let user_from_another1: User = User {
        name: String::from(&immutable_user.name),
        email: String::from("ha.tim@bum.com"),
        active: true,
        sign_in_count: 1
    };
    print_user(&user_from_another1);
    print_user(&immutable_user);
    let user_from_another2: User = User{
        name: String::from("From another"),
        email: String::from("from.another@mail.com"),
        ..immutable_user
    };
    print_user(&user_from_another2);
    print_user(&immutable_user);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black: Color = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of retangle is {}", area(&rect1));

    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user1(name: String, email: String) -> User {
    User {
        name: email,
        email: name,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(name: String, email: String) -> User {
    User {
        name,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User){
    println!("name: {}, email: {}, active: {}, sign_in_count: {}", user.name, user.email, user.active, user.sign_in_count);
}
