use adder::*;

#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100, got 200")]
fn grater_than_100() {
    Guess::new(200);
}