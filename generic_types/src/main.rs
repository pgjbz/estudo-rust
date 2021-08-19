use std::ops::Deref;

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//
// 	let mut largest = list.get(0).unwrap();
//
// 	for item in list.iter() {
// 		if item > largest {
// 			largest = &item;
// 		}
// 	}
//
// 	largest
// }

struct Point<T> {
	x: T,
	y: T,
}

impl<T> Point<T> {

	fn x(&self) -> &T {
		&self.x
	}

}

struct PointTwo<T, U> {
	x: T,
	y: U,
}

impl Point<f32> {
	fn distance_from_origin(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}

impl<T, U> PointTwo<T, U> {
	fn mixup<V, W>(self, other: PointTwo<V, W>) -> PointTwo<T, W> {
		PointTwo {
			x: self.x,
			y: other.y,
		}
	}
}

fn main() {
	// let number_list = vec![34, 50, 25, 100, 65];

	// let result = largest(&number_list);
	// println!("The largest number is {}", result);
	//
	// let char_list = vec!['y', 'm', 'a', 'q'];
	//
	// let result = largest(&char_list);
	// println!("The largest char is {}", result);

	let integer = Point::<i32> { x: 5, y: 10 };
	let float = Point::<f32> { x: 1.0, y: 4.0 };


	eprintln!("integer.x {}", integer.x());
	eprintln!("float.x {}", float.x());
	let distance = float.distance_from_origin();

	eprintln!("distance impl only for f32 generic types {}", distance);

	let integer_float_two = PointTwo::<i32, f32> { x: 1, y: 10.5};
	let float_integer_two = PointTwo { x: 10.5, y: 1};
	let both = PointTwo {x: 1, y: 1};
	let with_string = PointTwo { x: "Hello", y: "World!"};
	let mixed = both.mixup(with_string);
	eprintln!("x: {}, y: {}", mixed.x, mixed.y);
}