fn main() {
    let mut s = String::new();
    let data = "initial content";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("Initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push(' ');
    s1.push_str(s2);
    eprintln!("s1 is {}", s1);

    let s1 = String::from("Banana");
    let s2 = String::from(", apple");
    let s3 = s1 + &s2; //lose ownership of s1 variable

    //eprintln!("s1 value is {}", s1); borrow error
    eprintln!("s2 value is {}", s2);
    eprintln!("s3 value is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    eprintln!("{}", s);

    let s1 = String::from("hello");
    //let h = s1[0]; dont compile, because string cannot be indexed by integer type

    /* dont compile?????
     let hello = "Здравствуйте";
     let answer = &hello[0];
     eprintln!("{}", answer);
    */

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }


}
