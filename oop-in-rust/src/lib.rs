pub struct AvarageCollection { //public struct
    list: Vec<i32>, //private attribute
    average: f64 //private attribute
}

impl AvarageCollection {

    pub fn new() -> Self {
        AvarageCollection {
            list: vec![],
            average: 0.0
        }
    }

    pub fn add(&mut self, value: i32) { //public method
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) { //private method
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn insert_item_avarage() {
        let mut avg_collection = AvarageCollection::new();
        avg_collection.add(100);
       
        assert_eq!(100.0, avg_collection.average());
    }

    #[test]
    fn remove_item_avarage() {
        let mut avg_collection = AvarageCollection::new();
        avg_collection.add(100);
        let pop = avg_collection.remove();
        assert!(pop.is_some());
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
