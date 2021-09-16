pub fn add_one(n: i32) -> i32 {
    n + 1
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn check_if_sum_one(){
        assert_eq!(add_one(1), 2);
    }
}
