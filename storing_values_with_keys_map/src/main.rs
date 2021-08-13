use std::collections::HashMap;

fn main() {
    let mut hash_map = HashMap::new();
    hash_map.insert("First", 1);
    hash_map.insert("Second", 2);

    for (index, value) in &hash_map {
        eprintln!("{} - {}", index, value);
    }
    hash_map.insert("Third", 3);

    for (index, value) in hash_map {
        eprintln!("{} - {}", index, value);
    }

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    for (index, value) in &scores {
        eprintln!("{} - {}", index, value);
    }

    let score = scores.get("Red");
    eprintln!("{}", score.unwrap());

    scores.insert(String::from("Blue"), 25);

    let score = scores.get("Blue");
    eprintln!("{}", score.unwrap());

    eprintln!("{:?}", scores);

    scores.entry(String::from("Blue")).or_insert(80);
    scores.entry(String::from("Yellow")).or_insert(80);

    eprintln!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
