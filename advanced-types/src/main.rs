type Kilometers = u32; //basically an alias

type Thunk = Box<dyn Fn() + Send + 'static>; //alias to long type

fn takes_long_type(f: Thunk) {
    f.as_ref();
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

/*
fn bar() -> ! { ! is never type, the word `continue` has the value never, that is, never return
}
 in the match expression like: 
    let guess: i32 = match guess.trim().parse() {
        Ok(val) => val,
        Err(_) => continue
    }
    continue never produce a value, its make continue a valid value to return, contrary to return a different type like

    let guess: i32 = match guess.trim().parse() {
        Ok(val) => val,
        Err(_) => "don't compile"
    }

    loop is another type thats return !, like expression above

    let err = match return_err() {
        Ok(()) => loop {
            println!("Infinite loop");
        }, 
        Err(val) => val
    };

    fn return_err() -> Result<(), String> {
        if 1 > 10 {
            Err(String::from("Hello"))
        } else {
            Ok(())
        }
    }

    this is valid, because loop never return a value

 */

fn generic<T: ?Sized>(t: &T) { //the ?Sized is is the method to say "this object implement trait Sized or not" to compiler
    // --snip-- //only trait that's have this syntax are Sized
}

fn main() {
    let x: u32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
    takes_long_type(returns_long_type()); //only for remove unused warn
    // let s1: str = "Hello there!"; //don't compile because "Hello there!" is a &str and str is a different type that's don't contain a Sized trait implemented
    // let s2: str = "How's it going?";

}


