fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: another_name@ 3..=7,
        } => println!("Found an id in range: {}", another_name), //save value in another name of variable and use value
        Message::Hello { id: another_range@ 10..=12 } => {
            println!("Found an id in another range: {}", another_range)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
