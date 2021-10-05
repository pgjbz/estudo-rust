fn main() {

    //let Some(x)  = Some(5); -> refutable pattern in local binging None not covered.

    let x = 1;

    match x {
         1 => println!("one"),
         2 => println!("two"),
         3 => println!("three"),
         _ => println!("anything"), //non-exhautive patterns if this line nonexists
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), //another scope for y
                                                        //after this, y in Some(y) out of scope and y in let y = 10 are valid
        _ => println!("Default case, x = {:?}, y = {:?}", x, y)
    } 
    println!("at the end: x = {:?}, y = {:?}", x, y);

    println!("\nMultiple Patterns");
    
    let x = 1;
    
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything")
    }
    
    println!("\nMultiple Ranges of Values with..");

    let x:u32 = 6;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    println!("\nMultiple Ranges of chars with..");
    
    
    let x = 'z';
    
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }
    

}
