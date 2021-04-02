fn main() {
    let s: String = String::from("Hello world");
    let arr: [i32; 5] = [0,1,2,3,4];
    let arr_splice = &arr[1..3];
    println!("Array splice [{}, {}]", arr_splice[0], arr_splice[1]);
    println!("The first word of \"{}\" is \"{}\"", s, first_word(&s));
    println!("The second word of \"{}\" is \"{}\"", s, second_word(&s));
}

fn first_word(s: &String) -> &str{
    let bytes: &[u8] = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    let mut first_space: usize = 0;
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_space == 0 {
            first_space = i;
        } else if item == b' ' || i+1 == bytes.len() && first_space > 0 {
            return &s[first_space+1..i+1];
        }
    }
    &s[..]
}
