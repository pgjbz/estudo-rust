#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {

    pub fn square(size: i32) -> Self {
        Rectangle {
            width: size,
            height: size
        }
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        // self.width < other.width && self.height > other.height
    } 
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {

        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greetings(name: &str) -> String {
    format!("Hello!")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {

        let larger = Rectangle {
            width: 10,
            height: 10
        };

        let smaller = Rectangle::square(5);

        assert!(larger.can_hold(&smaller));

    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle::square(5);
        let larger = Rectangle {
            width: 42,
            height: 33
        };
 
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn two_plus_two_equals_four() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn two_plus_two_not_equals_five() {
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greetings("Xanaina");
        assert!(result.contains("Xanaina"), 
                "Greeting did not contain name, value was `{}`", 
                result);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200")]
    fn grater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works_user_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    // fn make_fail(){
    //     panic!("make it fail");
    // }
}
