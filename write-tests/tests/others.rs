use adder::*;

#[test]
fn less_or_equals_than_100() {
    let guess = Guess::new(99);
    assert_eq!(guess.value, 99);
}