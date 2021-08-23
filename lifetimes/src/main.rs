fn main() {
    let string1 = String::from("abcd");
    let string2;

    {
        let string3 = "xyz";
        string2 = string3; //why this work????????
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }


    let string1 = String::from("long string is long");

    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    //println!("The longest string is {}", result); variable string2 out of the scope in this point, code do not compile

    println!("{}", string2); //dont lose de reference
    println!("{}", string1);

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { //specify the lifetime to compiler, lifetime name is a, i still learning about it
    if x.len() > y.len() {                          //if that function always return the first parameter, would’t need to specify a lifetime on the y parameter.
        x                                           //like that fn longest<'a>(x: &'a str, y: &str) -> &'a str { x }
    } else {                                        //because y does not any relationship on lifetime of x
        y                                           //if u dont return any parameter and returned value is a value created inside function you dont need to specify the lifetime,
    }                                               //for example
}                                                   //fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { let result = String::from("Return"); result.as_str() }
                                                    //does not compile because result does not have any relationship of parameters,

/*
    Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
    Once they’re connected, Rust has enough information to allow memory-safe operations and disallow
    operations that would create dangling pointers or otherwise violate memory safety.

    source:  https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#thinking-in-terms-of-lifetimes
*/


/*
let r;
{
	let x = 5u8;
	r = &x;
}

println!("Number is {}", r);

5 |         r = &x;
|             ^^ borrowed value does not live long enough
6 |     }
|     - `x` dropped here while still borrowed
7 |
8 |     println!("Number is {}", r);
|                              - borrow later used here*/