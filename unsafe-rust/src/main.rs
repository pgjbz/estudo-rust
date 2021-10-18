use std::slice;


static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; //only be changed in a unsafe code
    }
}

fn main() {
	call_unsafe_function();
	
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

	let (a, b) = split_at_mut(r, 3);

	println!("a - {:?} | b - {:?}", vec![a], vec![b]);
	call_external_codes();


	add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER); //only access in unsafe blocl
    }

	creating_raw_pointers_with_cast();
}

unsafe fn unsafe_function() {
	println!("Unsafe function only be called by unsafe block");
}

fn call_unsafe_function() {
	unsafe {
		unsafe_function();
	}
}

fn creating_raw_pointers_with_cast() {
	let mut num = 5;
	//its possible create raw pointers in safe code
	let r1 = &num as *const i32; //raw pointer static
	let r2 = &mut num as *mut i32; //raw pointer mutable
	//raw pointers is created using as keyword to cast a valid reference to raw pointer
	unsafe { //to access and modify raw pointers is necessary unsafe block
		println!("{}", *r1);
		println!("{}", *r2);
		*r2 = 10; 
		println!("{}", *r1);
		println!("{}", *r2);
	}

	let address = 0x012345usize;
	let r = address as *const i32;

	unsafe {
		println!("address - {}", address);
		println!("r - {}", *r); //Segmentation fault (core dumped), because is a invalid address
	}
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) { //safe abstraction to unsafe code
	let len = slice.len();
	let ptr = slice.as_mut_ptr();

	assert!(mid <= len);

	unsafe {
		(
			slice::from_raw_parts_mut(ptr, mid),
			slice::from_raw_parts_mut(ptr.add(mid), len - mid)
		)
	}
}



fn call_external_codes() {
	extern "C" {
		fn abs(input: i32) -> i32;
		fn sqrt(input: f64) -> f64;
	}

	unsafe {
		println!("Absolute value of -3 according to C: {}", abs(-3));
		println!("Sqrt of 100 according to C: {}", sqrt(100f64));
	}
}

#[no_mangle] //not change the name
pub extern "C" fn call_from_c() { //create exportable function to another languages
    println!("Just called a Rust function from C!");
}