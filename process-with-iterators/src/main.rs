fn main() {

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter(); //create a iter

    // for n in v1_iter {
    //     println!("Value {}", n);
    // }

    assert_eq!(v1_iter.next(), Some(&1)); //to use next iter have to be mutable
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None)
}
