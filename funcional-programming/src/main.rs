use std::{collections::HashMap, thread, time::Duration};

struct Cacher<T> 
where
    T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap::<u32, u32>,
}



impl<T> Cacher<T>
where
    T: Fn(u32) -> u32
{

    fn new(calculation: T) -> Self {
        Cacher { calculation , value: HashMap::new() }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, arg);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];

    let equal_to_x  = move |z| z == x; //move word informe the compile "closure will borrow the value"

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y)); //if note use move keyword x cannot be used.... What you do now???
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 { this function has moved to closure
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    
    // let expansive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // }; //closure declaration with type
    //let expansive_result = simulated_expensive_calculation(intensity); moved to use closure
    //let expansive_result = expansive_closure(intensity);
    let mut expansive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }); //storing closure with generic type 
    if intensity < 25 {
        println!("Today, do {} pushups!", expansive_result.value(intensity));
        println!("Next, do {} situps!", expansive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expansive_result.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test] 
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
