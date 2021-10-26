
//when call the macro vec2 are generated the code above

/**
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
 */

macro_rules! vec2 { 
	[$( $x: expr), *] => { //give the expression a name $x
		{					//* means that have 0 or more arguments
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x); //similar a foreach????
			)*
			temp_vec
		}
	};
}




					//+ definy has to be one or more args
macro_rules! prin { //between curly is the body of macro
	() => (print!("\n"));
	($( $x: expr), +) => { //name of parameter has to be expr
		$(				//inside this curly is the body of expr
			println!("{}", $x);
		)+
	};
}

fn main() {
    let vec = vec2![1, 2, 3];
	println!("{:?}", vec);
	prin![];
	prin!["Hello", "Cambada", "de", "Gente"];
}
